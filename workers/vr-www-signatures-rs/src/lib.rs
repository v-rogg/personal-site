use js_sys::Date;
use serde::{Deserialize, Serialize};
use std::iter::once;
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
                signature: result.signature
            };

            Response::from_json(&signature)
        })

        .post_async("/", |mut req, ctx| async move {
            if !check_auth_header(&req, ctx.secret("SECRET_KEY")?.to_string()) {
                return Response::error("Not authorized", 401)
            }

            #[derive(Deserialize)]
            struct SignatureInsert {
                name: String,
                signature: String,
            }

            let insert = match req.json::<SignatureInsert>().await {
                Ok(insert) => insert,
                Err(_) => return Response::error("Wrong input data", 400)
            };

            let id = uuid::Uuid::new_v4().to_string();
            let time = get_time();
            let db = ctx.env.d1("DB")?;

            let result = db.prepare("INSERT INTO signatures (id, name, ts_created, signature) VALUES (?, ?, ?, ?)")
                .bind(&[
                    JsValue::from(&id),
                    JsValue::from(&insert.name),
                    JsValue::from(&time),
                    JsValue::from(&insert.signature)
                ])?
                .run()
                .await?;

            match result.success() {
                true => Response::ok("Ok"),
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
                true => Response::ok("Ok"),
                false => Response::error("Could not update", 400)
            }
        })

        .run(req, env)
        .await
}