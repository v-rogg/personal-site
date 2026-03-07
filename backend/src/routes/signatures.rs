use axum::{
    extract::{Path, State},
    Json,
};
use base64::Engine;
use flate2::{write::GzEncoder, Compression};
use std::io::Write;
use std::sync::Arc;

use crate::{
    error::AppError,
    models::{
        CachedSignature, CreateSignatureRequest, CreateSignatureResponse, ListSignaturesResponse,
        SignatureResponse, SignatureSummary, UpdateSignatureRequest,
    },
    state::AppState,
};

/// Compress signature data to gzip+base64 for storage
fn compress_signature(signature: &str) -> String {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(signature.as_bytes()).unwrap();
    let compressed = encoder.finish().unwrap();
    base64::engine::general_purpose::STANDARD.encode(compressed)
}

pub async fn list_signatures(State(state): State<Arc<AppState>>) -> Json<ListSignaturesResponse> {
    let cache = state.signature_cache.get_all().await;

    Json(ListSignaturesResponse {
        signatures: cache
            .iter()
            .map(|s| SignatureSummary {
                id: s.id.clone(),
                name: s.name.clone(),
                ts_created: s.ts_created,
            })
            .collect(),
    })
}

pub async fn get_signature(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<SignatureResponse>, AppError> {
    // Try cache first (signatures stored compressed, sent compressed for client-side decompression)
    if let Some(sig) = state.signature_cache.get_by_id(&id).await {
        return Ok(Json(SignatureResponse {
            id: sig.id,
            name: sig.name,
            signature: sig.signature,
            ts_created: sig.ts_created,
            approved: true,
        }));
    }

    // Fallback to DB (for unapproved/unreviewed signatures)
    let sig = sqlx::query_as::<_, CachedSignature>(
        "SELECT id, name, signature, ts_created FROM signatures WHERE id = ?",
    )
    .bind(&id)
    .fetch_optional(&state.db)
    .await?
    .ok_or(AppError::NotFound)?;

    // Check if approved (NULL = not reviewed, 1 = approved, 0 = rejected)
    let approved: (Option<i32>,) = sqlx::query_as("SELECT approved FROM signatures WHERE id = ?")
        .bind(&id)
        .fetch_one(&state.db)
        .await?;

    Ok(Json(SignatureResponse {
        id: sig.id,
        name: sig.name,
        signature: sig.signature,
        ts_created: sig.ts_created,
        approved: approved.0 == Some(1),
    }))
}

pub async fn create_signature(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateSignatureRequest>,
) -> Result<Json<CreateSignatureResponse>, AppError> {
    let now = chrono::Utc::now().timestamp_millis();

    // Generate unique petname ID
    let id = generate_unique_petname(&state).await?;

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

    // Create or get user if email provided
    if let Some(email) = &req.email {
        let user_id = get_or_create_user(&state, email, now).await?;

        // Link session to user
        sqlx::query("UPDATE sessions SET user_id = ? WHERE id = ?")
            .bind(user_id)
            .bind(&req.session_id)
            .execute(&state.db)
            .await?;
    }

    // Compress and insert signature (approved = NULL means not yet reviewed)
    let compressed_signature = compress_signature(&req.signature);
    sqlx::query(
        r#"
        INSERT INTO signatures (id, session_id, name, signature, approved, ts_created)
        VALUES (?, ?, ?, ?, NULL, ?)
        "#,
    )
    .bind(&id)
    .bind(&req.session_id)
    .bind(&req.name)
    .bind(&compressed_signature)
    .bind(now)
    .execute(&state.db)
    .await?;

    // Send confirmation email
    let url = format!("{}/?s={}", state.config.base_url, id);
    if let Err(e) = state
        .email_service
        .send_signature_confirmation(&req.name, &id, &url, req.email.as_deref())
        .await
    {
        tracing::error!("Failed to send signature confirmation email: {}", e);
    }

    Ok(Json(CreateSignatureResponse { id, url }))
}

pub async fn update_signature(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(req): Json<UpdateSignatureRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    if req.name.is_none() && req.signature.is_none() && req.approved.is_none() {
        return Ok(Json(serde_json::json!({ "success": true, "message": "No changes requested" })));
    }

    // Check if signature was previously in cache (i.e., was approved)
    let was_cached = state.signature_cache.contains(&id).await;

    let now = chrono::Utc::now().timestamp_millis();

    let mut query = String::from("UPDATE signatures SET ts_modified = ?");
    let mut params: Vec<String> = vec![now.to_string()];

    if let Some(name) = &req.name {
        query.push_str(", name = ?");
        params.push(name.clone());
    }

    // Compress signature data on updates
    let compressed_signature = req.signature.as_ref().map(|s| compress_signature(s));
    if let Some(sig) = &compressed_signature {
        query.push_str(", signature = ?");
        params.push(sig.clone());
    }

    if let Some(approved) = req.approved {
        query.push_str(", approved = ?");
        params.push(if approved { "1".to_string() } else { "0".to_string() });
    }

    query.push_str(" WHERE id = ?");
    params.push(id.clone());

    // Build the query dynamically
    let mut q = sqlx::query(&query);
    for param in &params {
        q = q.bind(param);
    }

    let result = q.execute(&state.db).await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    // Handle cache updates based on approval status change and field updates
    let is_now_approved = req.approved.unwrap_or(was_cached);

    match (was_cached, is_now_approved) {
        // Was approved, now rejected -> remove from cache
        (true, false) => {
            state.signature_cache.remove(&id).await;
        }
        // Was not approved, now approved -> add to cache from DB
        (false, true) => {
            if let Err(e) = state.signature_cache.add_from_db(&state.db, &id).await {
                tracing::error!("Failed to add signature to cache: {}", e);
                return Err(AppError::Cache(format!("Failed to add signature to cache: {}", e)));
            }
        }
        // Was approved, still approved with changes -> update in cache
        (true, true) if req.name.is_some() || req.signature.is_some() => {
            if !state.signature_cache.update(&id, req.name.clone(), compressed_signature).await {
                tracing::warn!("Signature {} not found in cache for update", id);
            }
        }
        // No cache change needed
        _ => {}
    }

    Ok(Json(serde_json::json!({ "success": true })))
}

async fn generate_unique_petname(state: &Arc<AppState>) -> anyhow::Result<String> {
    loop {
        let id = petname::petname(3, "-").unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let exists: Option<(String,)> =
            sqlx::query_as("SELECT id FROM signatures WHERE id = ?")
                .bind(&id)
                .fetch_optional(&state.db)
                .await?;

        if exists.is_none() {
            return Ok(id);
        }
    }
}

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
