use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Event {
    pub id: String,
    pub session_id: String,
    pub event_name: String,
    pub ts: i64,
    pub properties: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateEventRequest {
    pub session_id: String,
    pub event_name: String,
    pub properties: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct CreateEventResponse {
    pub event_id: String,
}
