use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskExecutionLog {
    pub task_id: String,
    pub command: String,
    pub status: String,
    pub message: String,
    pub metadata: HashMap<String, serde_json::Value>,
    pub created_at: OffsetDateTime,
}
