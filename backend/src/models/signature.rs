use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Signature {
    pub id: String,
    pub session_id: Option<String>,
    pub name: String,
    pub signature: String,
    pub approved: Option<i32>,
    pub ts_created: i64,
    pub ts_modified: Option<i64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SignatureSummary {
    pub id: String,
    pub name: String,
    pub ts_created: i64,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct CachedSignature {
    pub id: String,
    pub name: String,
    pub signature: String,
    pub ts_created: i64,
}

#[derive(Debug, Serialize)]
pub struct ListSignaturesResponse {
    pub signatures: Vec<SignatureSummary>,
}

#[derive(Debug, Serialize)]
pub struct SignatureResponse {
    pub id: String,
    pub name: String,
    pub signature: String,
    pub ts_created: i64,
    pub approved: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateSignatureRequest {
    pub session_id: String,
    pub name: String,
    pub email: Option<String>,
    pub signature: String,
}

#[derive(Debug, Serialize)]
pub struct CreateSignatureResponse {
    pub id: String,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSignatureRequest {
    pub name: Option<String>,
    pub signature: Option<String>,
    pub approved: Option<bool>,
}
