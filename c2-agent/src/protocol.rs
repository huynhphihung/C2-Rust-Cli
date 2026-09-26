use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct HeartBeatMessage {
    pub agent_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestTaskMessage {
    pub agent_id: Uuid,
}
