use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{error::AppError, state::AppState};

async fn get_or_create_user(state: &Arc<AppState>, email: &str, now: i64) -> anyhow::Result<String> {
    // Check if user exists
    let existing: Option<(String,)> = sqlx::query_as("SELECT id FROM users WHERE email = ?")
        .bind(email)
        .fetch_optional(&state.db)
        .await?;

    if let Some((id,)) = existing {
        return Ok(id);
    }

    // Create new user with UUID
    let user_id = uuid::Uuid::new_v4().to_string();
    sqlx::query("INSERT INTO users (id, email, ts_created) VALUES (?, ?, ?)")
        .bind(&user_id)
        .bind(email)
        .bind(now)
        .execute(&state.db)
        .await?;

    Ok(user_id)
}

#[derive(Debug, Deserialize)]
pub struct ContactRequest {
    pub session_id: String,
    pub email: String,
    pub message: String,
    pub captcha_solution: String,
}

#[derive(Debug, Serialize)]
pub struct ContactResponse {
    pub success: bool,
}

pub async fn submit_contact(
    State(state): State<Arc<AppState>>,
    Json(req): Json<ContactRequest>,
) -> Result<Json<ContactResponse>, AppError> {
    // Validate CAPTCHA
    state.captcha_service.verify(&req.captcha_solution).await?;

    let now = chrono::Utc::now().timestamp_millis();

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

    // Create or get user
    let user_id = get_or_create_user(&state, &req.email, now).await?;

    // Link session to user
    sqlx::query("UPDATE sessions SET user_id = ? WHERE id = ?")
        .bind(&user_id)
        .bind(&req.session_id)
        .execute(&state.db)
        .await?;

    // Track event
    let event_id = uuid::Uuid::new_v4().to_string();
    sqlx::query("INSERT INTO events (id, session_id, event_name, ts) VALUES (?, ?, 'contact.submit', ?)")
        .bind(&event_id)
        .bind(&req.session_id)
        .bind(now)
        .execute(&state.db)
        .await?;

    // Send email
    state
        .email_service
        .send_contact_request(&req.email, &req.message)
        .await
        .map_err(|e| AppError::Email(e.to_string()))?;

    Ok(Json(ContactResponse { success: true }))
}
