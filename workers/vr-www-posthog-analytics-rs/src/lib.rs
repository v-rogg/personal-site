use worker::*;
use wasm_bindgen::JsValue;
use std::collections::HashMap;
use chrono::Datelike;

fn check_auth_header(req: &Request, secret_key: String) -> bool {
    let auth_header = req.headers().get("Authorization").unwrap();

    match auth_header {
        Some(s) => s == format!("Bearer {}", secret_key),
        _ => false,
    }
}

#[event(fetch)]
async fn fetch(
    req: Request,
    env: Env,
    _ctx: Context,
) -> Result<Response> {
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

            let mut headers = Headers::new();
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

        .get_async("/week", |req, ctx| async move {
            if !check_auth_header(&req, ctx.secret("SECRET_KEY")?.to_string()) {
                return Response::error("Not authorized", 401)
            };

            // Prepare the PostHog query
            let project_id = ctx.secret("POSTHOG_PROJECT_ID")?.to_string();
            let posthog_url = format!("https://eu.i.posthog.com/api/projects/{}/query/", project_id).to_string();
            let bearer_token = ctx.secret("POSTHOG_API_KEY")?.to_string();

            let mut all_results = Vec::new();
            let mut offset = 0;
            let mut has_more = true;

            // Query all pages
            while has_more {
                let query_body = serde_json::json!({
                    "query": {
                        "kind": "HogQLQuery",
                        "query": format!("SELECT event, toStartOfDay(timestamp) AS day, count() AS occurrences FROM events GROUP BY event, day ORDER BY day DESC, occurrences DESC LIMIT 1000 OFFSET {}", offset)
                    }
                });

                // Make the POST request to PostHog
                let mut headers = Headers::new();
                headers.set("Authorization", &format!("Bearer {}", bearer_token))?;
                headers.set("Content-Type", "application/json")?;

                let mut init = RequestInit::new();
                init.with_method(Method::Post);
                init.with_headers(headers);
                init.with_body(Some(JsValue::from_str(&query_body.to_string())));

                let request = Request::new_with_init(&posthog_url, &init)?;
                let mut response = Fetch::Request(request).send().await?;
                let response_json: serde_json::Value = response.json().await?;

                // Extract results and check if there are more pages
                if let Some(results) = response_json.get("results").and_then(|r| r.as_array()) {
                    all_results.extend(results.clone());
                }

                has_more = response_json.get("hasMore").and_then(|h| h.as_bool()).unwrap_or(false);
                offset += 1000;
            }

            // Transform results to weekly format
            let mut weekly_data: HashMap<String, HashMap<String, u64>> = HashMap::new();

            for result in all_results {
                if let Some(result_array) = result.as_array() {
                    if result_array.len() >= 3 {
                        let event = result_array[0].as_str().unwrap_or("").to_string();
                        let day = result_array[1].as_str().unwrap_or("").to_string();
                        let count = result_array[2].as_u64().unwrap_or(0);

                        // Parse the date and convert to week start
                        if let Ok(date) = chrono::DateTime::parse_from_rfc3339(&day) {
                            let days_from_monday = date.weekday().num_days_from_monday() as i64;
                            let week_start = date.date_naive() - chrono::Duration::days(days_from_monday);
                            let week_key = format!("{}T00:00:00+02:00", week_start);

                            weekly_data.entry(week_key)
                                .or_insert_with(HashMap::new)
                                .entry(event)
                                .and_modify(|e| *e += count)
                                .or_insert(count);
                        }
                    }
                }
            }

            // Convert to desired format
            let mut weeks: Vec<serde_json::Value> = weekly_data.into_iter().map(|(week, actions)| {
                let actions_vec: Vec<serde_json::Value> = actions.into_iter().map(|(name, count)| {
                    serde_json::json!({
                        "name": name,
                        "count": count
                    })
                }).collect();

                serde_json::json!({
                    "week": week,
                    "actions": actions_vec
                })
            }).collect();

            // Sort by week descending
            weeks.sort_by(|a, b| {
                let week_a = a.get("week").and_then(|w| w.as_str()).unwrap_or("");
                let week_b = b.get("week").and_then(|w| w.as_str()).unwrap_or("");
                week_b.cmp(week_a)
            });

            Response::from_json(&weeks)
        })
        .run(req, env)
        .await

    // Ok(http::Response::builder()
    //     .status(http::StatusCode::OK)
    //     .body(Body::empty())?)
}
