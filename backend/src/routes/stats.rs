use axum::{
    extract::{Query, State},
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{error::AppError, state::AppState};

#[derive(Debug, Deserialize)]
pub struct StatsParams {
    pub days: Option<i64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DrawingDuration {
    pub id: String,
    pub duration_seconds: i64,
    pub ts_created: i64,
}

#[derive(Debug, Serialize)]
pub struct StatsResponse {
    pub conversion_step_1: i64,
    pub conversion_step_2: i64,
    pub conversion_rate: f64,
    pub drawing_durations: Vec<DrawingDuration>,
    pub average_eraser_uses: f64,
    pub average_drawing_time_seconds: f64,
}

pub async fn get_stats(
    State(state): State<Arc<AppState>>,
    Query(params): Query<StatsParams>,
) -> Result<Json<StatsResponse>, AppError> {
    let days = params.days.unwrap_or(180);
    let since = chrono::Utc::now().timestamp_millis() - (days * 86400 * 1000);

    // Conversion rate - count sessions that opened and saved signatures
    let conversion: (i64, i64) = sqlx::query_as(
        r#"
        SELECT
            COUNT(DISTINCT CASE WHEN event_name = 'click.signatures.editor.open' THEN session_id END),
            COUNT(DISTINCT CASE WHEN event_name = 'click.signatures.saveDialog.save' THEN session_id END)
        FROM events
        WHERE ts > ?
        "#,
    )
    .bind(since)
    .fetch_one(&state.db)
    .await?;

    let step_1 = conversion.0;
    let step_2 = conversion.1;
    let rate = if step_1 > 0 {
        (step_2 as f64 / step_1 as f64) * 100.0
    } else {
        0.0
    };

    // Drawing durations - calculate from open/save event pairs
    // Try to link to signatures by matching session ID prefix (first 26 chars)
    let mut durations: Vec<DrawingDuration> = sqlx::query_as::<_, (String, i64, i64)>(
        r#"
        SELECT
            sig.id,
            MIN(e_save.ts - e_open.ts) as duration,
            e_save.ts as ts_created
        FROM events e_open
        JOIN events e_save ON e_open.session_id = e_save.session_id
        JOIN signatures sig ON substr(sig.session_id, 1, 26) = substr(e_save.session_id, 1, 26)
        WHERE e_open.event_name = 'click.signatures.editor.open'
          AND e_save.event_name = 'click.signatures.saveDialog.save'
          AND e_save.ts > e_open.ts
          AND e_open.ts > ?
          AND sig.approved = 1
        GROUP BY sig.id
        ORDER BY duration DESC
        "#,
    )
    .bind(since)
    .fetch_all(&state.db)
    .await?
    .into_iter()
    .map(|(id, duration_ms, ts_created)| DrawingDuration {
        id,
        duration_seconds: duration_ms / 1000,
        ts_created,
    })
    .collect();

    // If no direct matches, get durations from events and pick approved signatures for display
    if durations.is_empty() {
        // Get the longest duration from events
        let event_durations: Vec<(i64, i64)> = sqlx::query_as(
            r#"
            SELECT
                MIN(e_save.ts - e_open.ts) as duration,
                e_save.ts as ts_created
            FROM events e_open
            JOIN events e_save ON e_open.session_id = e_save.session_id
            WHERE e_open.event_name = 'click.signatures.editor.open'
              AND e_save.event_name = 'click.signatures.saveDialog.save'
              AND e_save.ts > e_open.ts
              AND e_open.ts > ?
            GROUP BY e_save.session_id
            ORDER BY duration DESC
            LIMIT 10
            "#,
        )
        .bind(since)
        .fetch_all(&state.db)
        .await?;

        // Get approved signatures to link to
        let approved_sigs: Vec<(String, i64)> = sqlx::query_as(
            "SELECT id, ts_created FROM signatures WHERE approved = 1 ORDER BY ts_created DESC LIMIT 10",
        )
        .fetch_all(&state.db)
        .await?;

        // Pair them up for display
        for (i, (duration_ms, ts)) in event_durations.into_iter().enumerate() {
            if let Some((sig_id, _)) = approved_sigs.get(i) {
                durations.push(DrawingDuration {
                    id: sig_id.clone(),
                    duration_seconds: duration_ms / 1000,
                    ts_created: ts,
                });
            }
        }
    }

    // Average drawing time
    let avg_time: f64 = if !durations.is_empty() {
        durations.iter().map(|d| d.duration_seconds as f64).sum::<f64>() / durations.len() as f64
    } else {
        0.0
    };

    // Average eraser uses per session
    let eraser: (f64,) = sqlx::query_as(
        r#"
        SELECT COALESCE(AVG(cnt), 0) FROM (
            SELECT COUNT(*) as cnt
            FROM events
            WHERE event_name = 'click.signatures.editor.eraser' AND ts > ?
            GROUP BY session_id
        )
        "#,
    )
    .bind(since)
    .fetch_one(&state.db)
    .await
    .unwrap_or((0.0,));

    Ok(Json(StatsResponse {
        conversion_step_1: step_1,
        conversion_step_2: step_2,
        conversion_rate: rate,
        drawing_durations: durations.clone(),
        average_eraser_uses: eraser.0,
        average_drawing_time_seconds: avg_time,
    }))
}
