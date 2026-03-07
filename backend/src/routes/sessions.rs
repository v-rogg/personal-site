use axum::{
    extract::State,
    http::HeaderMap,
    Json,
};
use std::sync::Arc;

use crate::{
    error::AppError,
    models::{CreateSessionRequest, CreateSessionResponse},
    services::GeoIpService,
    state::AppState,
};

pub async fn create_session(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(req): Json<CreateSessionRequest>,
) -> Result<Json<CreateSessionResponse>, AppError> {
    let now = chrono::Utc::now().timestamp_millis();

    // Extract client IP and look up geo location
    let client_ip = GeoIpService::extract_client_ip(&headers);
    let geo = client_ip
        .as_ref()
        .map(|ip| state.geoip_service.lookup(ip))
        .unwrap_or_default();

    let result = sqlx::query(
        r#"
        INSERT INTO sessions (id, ts_created, ts_last_seen, country_code, region)
        VALUES (?, ?, ?, ?, ?)
        ON CONFLICT(id) DO UPDATE SET
            ts_last_seen = excluded.ts_last_seen,
            country_code = COALESCE(sessions.country_code, excluded.country_code),
            region = COALESCE(sessions.region, excluded.region)
        "#,
    )
    .bind(&req.session_id)
    .bind(now)
    .bind(now)
    .bind(&geo.country_code)
    .bind(&geo.region)
    .execute(&state.db)
    .await?;

    Ok(Json(CreateSessionResponse {
        session_id: req.session_id,
        created: result.rows_affected() > 0,
    }))
}
