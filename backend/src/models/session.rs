use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Session {
    pub id: String,
    pub user_id: Option<String>,
    pub user_agent: Option<String>,
    pub country_code: Option<String>,
    pub region: Option<String>,
    pub ts_created: i64,
    pub ts_last_seen: i64,
}

#[derive(Debug, Deserialize)]
pub struct CreateSessionRequest {
    pub session_id: String,
}

#[derive(Debug, Serialize)]
pub struct CreateSessionResponse {
    pub session_id: String,
    pub created: bool,
}
