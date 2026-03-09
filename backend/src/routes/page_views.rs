use axum::{
    extract::{Path, State},
    Json,
};
use std::sync::Arc;

use crate::{
    error::AppError,
    models::{
        CreatePageViewRequest, CreatePageViewResponse, UpdatePageViewRequest,
        UpdatePageViewResponse,
    },
    state::AppState,
};

pub async fn create_page_view(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreatePageViewRequest>,
) -> Result<Json<CreatePageViewResponse>, AppError> {
    let now = chrono::Utc::now().timestamp_millis();
    let page_view_id = uuid::Uuid::new_v4().to_string();

    // Ensure session exists
    sqlx::query(
        r#"
        INSERT INTO sessions (id, user_agent, ts_created, ts_last_seen)
        VALUES (?, ?, ?, ?)
        ON CONFLICT(id) DO UPDATE SET
            ts_last_seen = excluded.ts_last_seen,
            user_agent = COALESCE(sessions.user_agent, excluded.user_agent)
        "#,
    )
    .bind(&req.session_id)
    .bind(&req.user_agent)
    .bind(now)
    .bind(now)
    .execute(&state.db)
    .await?;

    // Only store external referrers
    let referrer = req
        .referrer
        .filter(|r| !r.contains("valentinrogg.de") && !r.is_empty());

    // Insert page view
    sqlx::query(
        r#"
        INSERT INTO page_views (id, session_id, path, referrer, ts_start, max_scroll_percent)
        VALUES (?, ?, ?, ?, ?, 0)
        "#,
    )
    .bind(&page_view_id)
    .bind(&req.session_id)
    .bind(&req.path)
    .bind(referrer)
    .bind(now)
    .execute(&state.db)
    .await?;

    Ok(Json(CreatePageViewResponse { page_view_id }))
}

pub async fn update_page_view(
    State(state): State<Arc<AppState>>,
    Path(page_view_id): Path<String>,
    Json(req): Json<UpdatePageViewRequest>,
) -> Result<Json<UpdatePageViewResponse>, AppError> {
    let now = chrono::Utc::now().timestamp_millis();

    if req.ended.unwrap_or(false) {
        // Page unload - calculate duration
        sqlx::query(
            r#"
            UPDATE page_views
            SET ts_end = ?,
                duration_seconds = ? - ts_start,
                max_scroll_percent = MAX(max_scroll_percent, ?)
            WHERE id = ? AND session_id = ?
            "#,
        )
        .bind(now)
        .bind(now)
        .bind(req.max_scroll_percent.unwrap_or(0))
        .bind(page_view_id)
        .bind(&req.session_id)
        .execute(&state.db)
        .await?;
    } else if let Some(scroll) = req.max_scroll_percent {
        // Scroll update - only update if higher
        sqlx::query(
            r#"
            UPDATE page_views
            SET max_scroll_percent = MAX(max_scroll_percent, ?)
            WHERE id = ? AND session_id = ?
            "#,
        )
        .bind(scroll)
        .bind(page_view_id)
        .bind(&req.session_id)
        .execute(&state.db)
        .await?;
    }

    // Update session last seen
    sqlx::query("UPDATE sessions SET ts_last_seen = ? WHERE id = ?")
        .bind(now)
        .bind(&req.session_id)
        .execute(&state.db)
        .await?;

    Ok(Json(UpdatePageViewResponse { success: true }))
}
