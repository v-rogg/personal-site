use axum::{extract::State, Json};
use std::sync::Arc;

use crate::{
    error::AppError,
    models::{CreateEventRequest, CreateEventResponse},
    state::AppState,
};

pub async fn create_event(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateEventRequest>,
) -> Result<Json<CreateEventResponse>, AppError> {
    let now = chrono::Utc::now().timestamp_millis();
    let event_id = uuid::Uuid::new_v4().to_string();

    // Ensure session exists
    sqlx::query(
        r#"
        INSERT INTO sessions (id, ts_created, ts_last_seen)
        VALUES (?, ?, ?)
        ON CONFLICT(id) DO UPDATE SET ts_last_seen = excluded.ts_last_seen
        "#,
    )
    .bind(&req.session_id)
    .bind(now)
    .bind(now)
    .execute(&state.db)
    .await?;

    // Insert event
    let properties = req
        .properties
        .map(|p| serde_json::to_string(&p).unwrap_or_default());

    sqlx::query(
        r#"
        INSERT INTO events (id, session_id, event_name, ts, properties)
        VALUES (?, ?, ?, ?, ?)
        "#,
    )
    .bind(&event_id)
    .bind(&req.session_id)
    .bind(&req.event_name)
    .bind(now)
    .bind(properties)
    .execute(&state.db)
    .await?;

    Ok(Json(CreateEventResponse { event_id }))
}
