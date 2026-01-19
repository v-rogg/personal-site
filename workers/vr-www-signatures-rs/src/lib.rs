use core::str;
use flate2::{Compression, write::GzEncoder};
use js_sys::Date;
use serde::{Deserialize, Serialize};
use std::{
    io::{Read, Write},
    iter::once,
};
use wasm_bindgen::prelude::*;
use worker::*;

fn check_auth_header(req: &Request, secret_key: String) -> bool {
    let auth_header = req.headers().get("Authorization").unwrap();

    match auth_header {
        Some(s) => s == format!("Bearer {}", secret_key),
        _ => false,
    }
}

#[wasm_bindgen]
pub fn get_time() -> String {
    Date::new_0().to_iso_string().as_string().unwrap()
}

#[event(fetch)]
async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    once(console_error_panic_hook::hook);
    Router::new()

        .get_async("/health", |_req, _ctx| async move {
            Response::from_json(&serde_json::json!({
                "status": "healthy",
            }))
        })

        .get_async("/", |req, ctx| async move {
            if !check_auth_header(&req, ctx.secret("SECRET_KEY")?.to_string()) {
                return Response::error("Not authorized", 401)
            };

            let db = ctx.env.d1("DB")?;

            let query = db.prepare("SELECT id FROM signatures WHERE approved = true");
            let result = query.all().await?;

            #[derive(Deserialize, Serialize)]
            struct SignatureId {
                id: String,
            }

            let signatures: Vec<SignatureId> = result.results::<SignatureId>()?.into_iter().collect();
            Response::from_json(&signatures)
        })

        .get_async("/all", |req, ctx| async move {
            if !check_auth_header(&req, ctx.secret("SECRET_KEY")?.to_string()) {
                return Response::error("Not authorized", 401)
            };

            let db = ctx.env.d1("DB")?;
            let query = db.prepare("SELECT id, approved FROM signatures");
            let result = query.all().await?;

            #[derive(Deserialize)]
            struct SignatureIdApproved {
                id: String,
                approved: Option<u32>,
            }

            let signatures: Vec<SignatureIdApproved> = result.results::<SignatureIdApproved>()?.into_iter().collect();

            #[derive(Serialize)]
            struct SignatureIdApprovedBool {
                id: String,
                approved: Option<bool>,
            }

            let signatures_converted: Vec<SignatureIdApprovedBool> = signatures.into_iter().map(|sig| SignatureIdApprovedBool {
                id: sig.id,
                approved: sig.approved.map(|v| match v {
                    0 => false,
                    1 => true,
                    _ => false
                })
            }).collect();
            Response::from_json(&signatures_converted)
        })

        .get_async("/:id", |req, ctx| async move {
            if !check_auth_header(&req, ctx.secret("SECRET_KEY")?.to_string()) {return Response::error("Not authorized", 401)};
            let id = ctx.param("id").unwrap();
            let db = ctx.env.d1("DB")?;

            let stmt =
                db.prepare("SELECT id, name, ts_created, ts_modified, approved, signature FROM signatures WHERE id = ?");
            let query = stmt.bind(&[id.into()])?;


            #[derive(Deserialize)]
            struct SignatureResult {
                id: String,
                name: String,
                ts_created: String,
                ts_modified: Option<String>,
                approved: Option<u32>,
                signature: String,
            }
            let result = match query.first::<SignatureResult>(None).await? {
                Some(r) => r,
                None => return Response::error("Not found", 404),
            };

            #[derive(Serialize)]
            struct SignatureResponse {
                id: String,
                name: String,
                ts_created: String,
                ts_modified: Option<String>,
                approved: Option<bool>,
                signature: String,
            }

            let decoded_signature_data = match base64::Engine::decode(&base64::engine::general_purpose::STANDARD, &result.signature) {
                Ok(compressed_data) => {
                    let mut decoder = flate2::read::GzDecoder::new(&compressed_data[..]);
                    let mut decompressed_data = String::new();
                    decoder.read_to_string(&mut decompressed_data).unwrap();
                    decompressed_data
                },
                Err(_) => result.signature
            };

            let signature = SignatureResponse {
                id: result.id,
                name: result.name,
                ts_created: result.ts_created,
                ts_modified: result.ts_modified,
                approved: result.approved.map(|v| match v {
                    0 => false,
                    1 => true,
                    _ => false
                }),
                signature: decoded_signature_data
            };

            Response::from_json(&signature)
        })

        .get_async("/:id/check", |req, ctx| async move {
            if !check_auth_header(&req, ctx.secret("SECRET_KEY")?.to_string()) {return Response::error("Not authorized", 401)};
            let id = ctx.param("id").unwrap();
            let db = ctx.env.d1("DB")?;

            let stmt =
                db.prepare("SELECT id, approved FROM signatures WHERE id = ?");
            let query = stmt.bind(&[id.into()])?;


            #[derive(Deserialize)]
            struct SignatureCheckResult {
                id: String,
                approved: Option<u32>
            }

            let result = match query.first::<SignatureCheckResult>(None).await? {
                Some(r) => r,
                None => return Response::error("Not found", 404),
            };

            #[derive(Serialize)]
            struct SignatureResponse {
                id: String,
                approved: Option<bool>
            }

            let signature = SignatureResponse {
                id: result.id,
                approved: result.approved.map(|v| match v {
                    0 => false,
                    1 => true,
                    _ => false
                }),
            };

            Response::from_json(&signature)
        })

        // Find signature by Unix timestamp (with 10 second tolerance)
        .get_async("/by-timestamp/:timestamp", |req, ctx| async move {
            if !check_auth_header(&req, ctx.secret("SECRET_KEY")?.to_string()) {
                return Response::error("Not authorized", 401)
            };

            let timestamp_str = ctx.param("timestamp").unwrap();
            let timestamp: i64 = match timestamp_str.parse() {
                Ok(ts) => ts,
                Err(_) => return Response::error("Invalid timestamp", 400),
            };

            let db = ctx.env.d1("DB")?;

            // Convert Unix timestamp to both ISO and space-separated formats for comparison (10 second tolerance)
            // D1 stores timestamps in two formats:
            // - ISO: "2024-12-17T10:32:41.046Z"
            // - Space-separated: "2024-11-26 21:54:29"
            use chrono::{Utc, TimeZone};
            let ts_lower = Utc.timestamp_opt(timestamp - 10, 0).unwrap();
            let ts_upper = Utc.timestamp_opt(timestamp + 10, 0).unwrap();

            // Format for ISO comparison (YYYY-MM-DDTHH:MM:SS)
            let ts_lower_iso = ts_lower.format("%Y-%m-%dT%H:%M:%S").to_string();
            let ts_upper_iso = ts_upper.format("%Y-%m-%dT%H:%M:%SZ").to_string();

            // Format for space-separated comparison (YYYY-MM-DD HH:MM:SS)
            let ts_lower_space = ts_lower.format("%Y-%m-%d %H:%M:%S").to_string();
            let ts_upper_space = ts_upper.format("%Y-%m-%d %H:%M:%S").to_string();

            #[derive(Deserialize)]
            struct SignatureResult {
                id: String,
                ts_created: String,
            }

            // Query for signatures created within the time window, handling both formats
            let stmt = db.prepare(
                "SELECT id, ts_created FROM signatures WHERE (ts_created >= ? AND ts_created <= ?) OR (ts_created >= ? AND ts_created <= ?) ORDER BY ts_created DESC LIMIT 1"
            );
            let query = stmt.bind(&[
                ts_lower_iso.into(),
                ts_upper_iso.into(),
                ts_lower_space.into(),
                ts_upper_space.into(),
            ])?;

            let result = match query.first::<SignatureResult>(None).await? {
                Some(r) => r,
                None => return Response::error("Not found", 404),
            };

            #[derive(Serialize)]
            struct SignatureResponse {
                id: String,
                ts_created: String,
            }

            Response::from_json(&SignatureResponse {
                id: result.id,
                ts_created: result.ts_created,
            })
        })

        .post_async("/", |mut req, ctx| async move {
            if !check_auth_header(&req, ctx.secret("SECRET_KEY")?.to_string()) {
                return Response::error("Not authorized", 401)
            }

            #[derive(Deserialize)]
            struct SignatureInsert {
                name: String,
                signature: String,
                email: Option<String>
            }

            let insert = match req.json::<SignatureInsert>().await {
                Ok(insert) => insert,
                Err(_) => return Response::error("Wrong input data", 400)
            };

            // let id = uuid::Uuid::new_v4().to_string();
            // let id = human_ids::generate(Some(human_ids::Options{
            //     capitalize: false,
            //     add_adverb: true,
            //     separator: Some("-"),
            //     adjective_count: 0
            // }));

            let id = petname::petname(3, "-").unwrap();


            let time = get_time();
            let db = ctx.env.d1("DB")?;

            let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
            encoder.write_all(insert.signature.clone().as_bytes()).expect("Failed to write data");
            let compressed_data = encoder.finish().expect("Failed to finish compression");
            let signature_data_string = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, compressed_data);

            let mut params = vec![
                JsValue::from(&id),
                JsValue::from(&insert.name),
                JsValue::from(&time),
                JsValue::from(&signature_data_string)
            ];


            let query = if let Some(email) = insert.email {
                params.push(JsValue::from(email));
                "INSERT INTO signatures (id, name, ts_created, signature, email) VALUES (?, ?, ?, ?, ?)"
            } else {
                "INSERT INTO signatures (id, name, ts_created, signature) VALUES (?, ?, ?, ?)"
            };

            let result = db.prepare(query)
                .bind(&params)?
                .run()
                .await?;

            match result.success() {
                true => Response::from_json(&serde_json::json!({
                            "success": true,
                            "id": id
                        })),
                false => Response::error("Could not insert", 400)
            }
        })

        .put_async("/:id", |mut req, ctx| async move {
            let id = ctx.param("id").unwrap();

            #[derive(Deserialize, Debug)]
            struct SignatureUpdate {
                name: Option<String>,
                signature: Option<String>,
                approved: Option<bool>,
            }

            let update = match req.json::<SignatureUpdate>().await {
                Ok(update) => update,
                Err(_) => return Response::error("Wrong input data", 400),
            };

            if update.name.is_none() && update.signature.is_none() && update.approved.is_none() {
                return Response::ok("No changes requested");
            }

            let time = get_time();
            let db = ctx.env.d1("DB")?;

            let mut query = String::from("UPDATE signatures SET ts_modified = ?");
            let mut params: Vec<JsValue> = vec![time.into()];

            if let Some(name) = update.name {
                query.push_str(", name = ?");
                params.push(name.into());
            }

            if let Some(signature) = update.signature {
                query.push_str(", signature = ?");
                params.push(signature.into());
            }

            if let Some(approved) = update.approved {
                query.push_str(", approved = ?");
                params.push((if approved { 1 } else { 0 }).into());
            }

            query.push_str(" WHERE id = ?");
            params.push(id.into());

            let result = db.prepare(&query)
                .bind(&params)?
                .run()
                .await?;

            match result.success() {
                true => Response::from_json(&serde_json::json!({
                    "success": true
                })),
                false => Response::error("Could not update", 400)
            }
        })

        .run(req, env)
        .await
}
