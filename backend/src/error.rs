use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Not found")]
    NotFound,

    #[error("CAPTCHA validation failed")]
    CaptchaFailed,

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Request error: {0}")]
    Request(#[from] reqwest::Error),

    #[error("Email error: {0}")]
    Email(String),

    #[error("Cache error: {0}")]
    Cache(String),

    #[error("Internal error: {0}")]
    Internal(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::NotFound => (StatusCode::NOT_FOUND, "Not found"),
            AppError::CaptchaFailed => (StatusCode::BAD_REQUEST, "CAPTCHA validation failed"),
            AppError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error"),
            AppError::Request(_) => (StatusCode::BAD_GATEWAY, "External service error"),
            AppError::Email(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Email error"),
            AppError::Cache(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Cache error"),
            AppError::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal error"),
        };

        tracing::error!("Error: {:?}", self);

        (status, Json(serde_json::json!({ "error": message }))).into_response()
    }
}
