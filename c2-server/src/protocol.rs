use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RegisterMessage {
    pub agent_id: Uuid,
    pub hostname: String,
    pub username: String,
    pub os: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HeartBeatMessage {
    pub agent_id: Uuid,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RequestTaskMessage {
    pub agent_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    Sent,
    Completed,
    Failed,
    Running,
}
