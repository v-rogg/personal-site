use serde_json::json;
use wasm_bindgen::JsValue;
use worker::*;

fn check_auth_header(req: &Request, secret_key: String) -> bool {
    let auth_header = req.headers().get("Authorization").unwrap();

    match auth_header {
        Some(s) => s == format!("Bearer {}", secret_key),
        _ => false,
    }
}

fn parse_timeframe_filter(timeframe: &str) -> String {
    match timeframe {
        "all" => "1 = 1".to_string(), // No time filter
        _ if timeframe.starts_with('d') => {
            if let Ok(days) = timeframe[1..].parse::<u32>() {
                format!("timestamp >= now() - INTERVAL {} DAY", days)
            } else {
                "timestamp >= now() - INTERVAL 30 DAY".to_string() // Default fallback
            }
        }
        _ if timeframe.starts_with('m') => {
            if let Ok(months) = timeframe[1..].parse::<u32>() {
                format!("timestamp >= now() - INTERVAL {} MONTH", months)
            } else {
                "timestamp >= now() - INTERVAL 1 MONTH".to_string() // Default fallback
            }
        }
        _ if timeframe.starts_with('q') => {
            if let Ok(quarters) = timeframe[1..].parse::<u32>() {
                let months = quarters * 3;
                format!("timestamp >= now() - INTERVAL {} MONTH", months)
            } else {
                "timestamp >= now() - INTERVAL 3 MONTH".to_string() // Default fallback
            }
        }
        _ if timeframe.starts_with('y') => {
            if let Ok(years) = timeframe[1..].parse::<u32>() {
                format!("timestamp >= now() - INTERVAL {} YEAR", years)
            } else {
                "timestamp >= now() - INTERVAL 1 YEAR".to_string() // Default fallback
            }
        }
        _ => "timestamp >= now() - INTERVAL 30 DAY".to_string(), // Default fallback
    }
}

async fn execute_posthog_query(
    query: &str,
    project_id: &str,
    bearer_token: &str,
) -> Result<serde_json::Value> {
    let posthog_url = format!(
        "https://eu.i.posthog.com/api/projects/{}/query/",
        project_id
    );

    let query_body = json!({
        "query": {
            "kind": "HogQLQuery",
            "query": query
        }
    });

    let headers = Headers::new();
    headers.set("Authorization", &format!("Bearer {}", bearer_token))?;
    headers.set("Content-Type", "application/json")?;

    let mut init = RequestInit::new();
    init.with_method(Method::Post);
    init.with_headers(headers);
    init.with_body(Some(JsValue::from_str(&query_body.to_string())));

    let request = Request::new_with_init(&posthog_url, &init)?;
    let mut response = Fetch::Request(request).send().await?;
    let response_json: serde_json::Value = response.json().await?;

    Ok(response_json)
}

#[event(fetch)]
async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    console_error_panic_hook::set_once();

    Router::new()
        .get_async("/health", |_req, ctx| async move {
            // Check PostHog API connectivity
            let project_id = ctx.secret("POSTHOG_PROJECT_ID")?.to_string();
            let posthog_url = format!("https://eu.i.posthog.com/api/projects/{}/query/", project_id);
            let bearer_token = ctx.secret("POSTHOG_API_KEY")?.to_string();

            let simple_query = serde_json::json!({
                "query": {
                    "kind": "HogQLQuery",
                    "query": "SELECT 1 LIMIT 1"
                }
            });

            let headers = Headers::new();
            headers.set("Authorization", &format!("Bearer {}", bearer_token))?;
            headers.set("Content-Type", "application/json")?;

            let mut init = RequestInit::new();
            init.with_method(Method::Post);
            init.with_headers(headers);
            init.with_body(Some(JsValue::from_str(&simple_query.to_string())));

            let request = Request::new_with_init(&posthog_url, &init)?;

            match Fetch::Request(request).send().await {
                Ok(response) => {
                    if response.status_code() == 200 {
                        Response::from_json(&serde_json::json!({
                            "status": "healthy",
                            "posthog_api": "online",
                        }))
                    } else {
                        Response::from_json(&serde_json::json!({
                            "status": "degraded",
                            "posthog_api": "error",
                            "error": format!("PostHog API returned status: {}", response.status_code()),
                        }))
                    }
                }
                Err(_) => {
                    Response::from_json(&serde_json::json!({
                        "status": "unhealthy",
                        "posthog_api": "offline",
                        "error": "Failed to connect to PostHog API",
                    }))
                }
            }
        })

        .get_async("/conversion-rate", |req, ctx| async move {
            if !check_auth_header(&req, ctx.secret("SECRET_KEY")?.to_string()) {
                return Response::error("Not authorized", 401)
            };

            let project_id = ctx.secret("POSTHOG_PROJECT_ID")?.to_string();
            let bearer_token = ctx.secret("POSTHOG_API_KEY")?.to_string();

            // Parse timeframe query parameter
            let url = req.url()?;
            let timeframe = url.query_pairs()
                .find(|(key, _)| key == "timeframe")
                .map(|(_, value)| value.to_string())
                .unwrap_or_else(|| "d180".to_string()); // Default to 180 days

            let time_filter = parse_timeframe_filter(&timeframe);

            let conversion_query = format!(r#"
                SELECT
                    step_1,
                    step_2,
                    0 AS step_1_average_conversion_time,
                    0 AS step_1_median_conversion_time,
                    [] AS row_number,
                    '' AS final_prop
                FROM (
                    SELECT
                        COUNT(DISTINCT sessions_with_open) AS step_1,
                        COUNT(DISTINCT sessions_with_both) AS step_2
                    FROM (
                        SELECT
                            properties.$session_id AS session_id,
                            if(has(groupArray(event), 'click.signatures.editor.open'), session_id, NULL) AS sessions_with_open,
                            if(has(groupArray(event), 'click.signatures.editor.open') AND has(groupArray(event), 'click.signatures.saveDialog.save'), session_id, NULL) AS sessions_with_both
                        FROM events
                        WHERE
                            {}
                            AND event IN ('click.signatures.editor.open', 'click.signatures.saveDialog.save')
                            AND properties.$session_id IS NOT NULL
                            AND ifNull(not(match(toString(properties.$host), '^(localhost|127\\.0\\.0\\.1)($|:)')), 1)
                        GROUP BY session_id
                    )
                )
            "#, time_filter);

            let result = execute_posthog_query(&conversion_query, &project_id, &bearer_token).await?;

            let mut step_1 = 0;
            let mut step_2 = 0;
            let mut conversion_rate = 0.0;

            if let Some(results) = result.get("results").and_then(|r| r.as_array()) {
                if let Some(first_result) = results.get(0).and_then(|r| r.as_array()) {
                    if let Some(s1) = first_result.get(0).and_then(|v| v.as_u64()) {
                        step_1 = s1;
                    }
                    if let Some(s2) = first_result.get(1).and_then(|v| v.as_u64()) {
                        step_2 = s2;
                    }
                }
            }

            if step_1 > 0 {
                conversion_rate = (step_2 as f64 / step_1 as f64) * 100.0;
            }

            let response = json!({
                "conversion_step_1": step_1,
                "conversion_step_2": step_2,
                "conversion_rate": conversion_rate
            });

            Response::from_json(&response)
        })

        .get_async("/drawing-durations", |req, ctx| async move {
            if !check_auth_header(&req, ctx.secret("SECRET_KEY")?.to_string()) {
                return Response::error("Not authorized", 401)
            };

            let project_id = ctx.secret("POSTHOG_PROJECT_ID")?.to_string();
            let bearer_token = ctx.secret("POSTHOG_API_KEY")?.to_string();

            // Parse query parameters
            let url = req.url()?;
            let timeframe = url.query_pairs()
                .find(|(key, _)| key == "timeframe")
                .map(|(_, value)| value.to_string())
                .unwrap_or_else(|| "d180".to_string()); // Default to 180 days

            let aggregate = url.query_pairs()
                .find(|(key, _)| key == "aggregate")
                .map(|(_, value)| value.to_string());

            let time_filter = parse_timeframe_filter(&timeframe);

            // If aggregate=average, return the average drawing time
            if aggregate.as_deref() == Some("average") {
                let avg_query = format!(r#"
                    SELECT avg(duration) AS average_drawing_time_seconds
                    FROM (
                        SELECT
                            session_id,
                            max(save_time) - min(open_time) AS duration
                        FROM (
                            SELECT
                                properties.$session_id AS session_id,
                                if(event = 'click.signatures.editor.open', timestamp, NULL) AS open_time,
                                if(event = 'click.signatures.saveDialog.save', timestamp, NULL) AS save_time
                            FROM events
                            WHERE event IN ('click.signatures.editor.open', 'click.signatures.saveDialog.save')
                                  AND properties.$session_id IS NOT NULL
                                  AND properties.$host NOT LIKE 'localhost%'
                                  AND properties.$host NOT LIKE '127.0.0.1%'
                                  AND {}
                        )
                        WHERE session_id IS NOT NULL
                        GROUP BY session_id
                        HAVING min(open_time) IS NOT NULL AND max(save_time) IS NOT NULL
                    )
                "#, time_filter);

                let result = execute_posthog_query(&avg_query, &project_id, &bearer_token).await?;

                let mut avg_drawing_time_seconds = 0.0;
                if let Some(results) = result.get("results").and_then(|r| r.as_array()) {
                    if let Some(first_result) = results.get(0).and_then(|r| r.as_array()) {
                        if let Some(value) = first_result.get(0).and_then(|v| v.as_f64()) {
                            avg_drawing_time_seconds = value;
                        }
                    }
                }

                return Response::from_json(&json!({
                    "average_drawing_time_seconds": avg_drawing_time_seconds
                }));
            }

            // Default: return all drawing session durations within the timeframe, ordered by duration descending
            let drawing_durations_query = format!(r#"
                SELECT
                    properties.$session_id AS session_id,
                    max(timestamp) - min(timestamp) AS duration_seconds,
                    toUnixTimestamp(max(timestamp)) AS ts_created
                FROM events
                WHERE
                    event IN ('click.signatures.editor.open', 'click.signatures.saveDialog.save')
                    AND properties.$session_id IS NOT NULL
                    AND properties.$host NOT LIKE 'localhost%'
                    AND properties.$host NOT LIKE '127.0.0.1%'
                    AND {}
                GROUP BY properties.$session_id
                HAVING
                    countIf(event = 'click.signatures.editor.open') > 0
                    AND countIf(event = 'click.signatures.saveDialog.save') > 0
                ORDER BY duration_seconds DESC
            "#, time_filter);

            let result = execute_posthog_query(&drawing_durations_query, &project_id, &bearer_token).await?;

            let signatures_key = ctx.secret("SIGNATURES_WORKER_KEY")?.to_string();
            // Use SIGNATURES_WORKER_URL env var for the signatures worker URL
            // In production: https://signatures.valentinrogg.com
            // In local dev: http://localhost:8787
            let signatures_base_url = ctx.var("SIGNATURES_WORKER_URL")
                .map(|v| v.to_string())
                .unwrap_or_else(|_| "http://localhost:8787".to_string());

            let mut drawing_durations = Vec::new();
            if let Some(results) = result.get("results").and_then(|r| r.as_array()) {
                for result_row in results {
                    if let Some(row_array) = result_row.as_array() {
                        if row_array.len() >= 3 {
                            if let (Some(_session_id), Some(duration), Some(ts_created)) = (
                                row_array[0].as_str(),
                                row_array[1].as_f64(),
                                row_array[2].as_u64()
                            ) {
                                // Resolve timestamp to signature ID via signatures worker
                                let sig_url = format!("{}/by-timestamp/{}", signatures_base_url, ts_created);
                                let sig_headers = Headers::new();
                                sig_headers.set("Authorization", &format!("Bearer {}", signatures_key))?;

                                let mut sig_init = RequestInit::new();
                                sig_init.with_method(Method::Get);
                                sig_init.with_headers(sig_headers);

                                let sig_request = Request::new_with_init(&sig_url, &sig_init)?;
                                let sig_response = Fetch::Request(sig_request).send().await;

                                if let Ok(mut resp) = sig_response {
                                    if resp.status_code() == 200 {
                                        if let Ok(sig_data) = resp.json::<serde_json::Value>().await {
                                            if let Some(signature_id) = sig_data.get("id").and_then(|v| v.as_str()) {
                                                drawing_durations.push(json!({
                                                    "id": signature_id,
                                                    "duration_seconds": duration,
                                                    "ts_created": ts_created
                                                }));
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            let response = json!({
                "drawing_durations": drawing_durations,
                "has_data": !drawing_durations.is_empty()
            });

            Response::from_json(&response)
        })

        .get_async("/eraser-uses", |req, ctx| async move {
            if !check_auth_header(&req, ctx.secret("SECRET_KEY")?.to_string()) {
                return Response::error("Not authorized", 401)
            };

            let project_id = ctx.secret("POSTHOG_PROJECT_ID")?.to_string();
            let bearer_token = ctx.secret("POSTHOG_API_KEY")?.to_string();

            // Parse timeframe query parameter
            let url = req.url()?;
            let timeframe = url.query_pairs()
                .find(|(key, _)| key == "timeframe")
                .map(|(_, value)| value.to_string())
                .unwrap_or_else(|| "d180".to_string()); // Default to 180 days

            let time_filter = parse_timeframe_filter(&timeframe);

            let eraser_query = format!(r#"
                SELECT avg(eraser_count) AS average_eraser_uses
                FROM (
                    SELECT properties.$session_id AS session_id,
                           count() AS eraser_count
                    FROM events
                    WHERE event = 'click.signatures.editor.eraser'
                      AND properties.$session_id IS NOT NULL
                      AND {}
                    GROUP BY session_id
                )
            "#, time_filter);

            let result = execute_posthog_query(&eraser_query, &project_id, &bearer_token).await?;

            let mut avg_eraser_uses = 0.0;
            if let Some(results) = result.get("results").and_then(|r| r.as_array()) {
                if let Some(first_result) = results.get(0).and_then(|r| r.as_array()) {
                    if let Some(value) = first_result.get(0).and_then(|v| v.as_f64()) {
                        avg_eraser_uses = value;
                    }
                }
            }

            let response = json!({
                "average_eraser_uses": avg_eraser_uses
            });

            Response::from_json(&response)
        })

        .run(req, env)
        .await
}
