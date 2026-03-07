use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PageView {
    pub id: String,
    pub session_id: String,
    pub path: String,
    pub referrer: Option<String>,
    pub ts_start: i64,
    pub ts_end: Option<i64>,
    pub duration_seconds: Option<i64>,
    pub max_scroll_percent: i32,
}

#[derive(Debug, Deserialize)]
pub struct CreatePageViewRequest {
    pub session_id: String,
    pub path: String,
    pub referrer: Option<String>,
    pub user_agent: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CreatePageViewResponse {
    pub page_view_id: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePageViewRequest {
    pub session_id: String,
    pub max_scroll_percent: Option<i32>,
    pub ended: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct UpdatePageViewResponse {
    pub success: bool,
}
