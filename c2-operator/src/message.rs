use std::net::SocketAddr;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::task::{TaskAction, TaskResult};

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum Message {
    CreateTask { agent_id: Uuid, action: TaskAction },
    TaskResult(TaskResult),
    RequestSession,
    SessionList(Vec<SessionInfo>),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SessionInfo {
    pub session_id: Uuid,
    pub peer_addr: SocketAddr,
    pub info: Info,
}

#[derive(Default)]
pub struct ShellState {
    pub input: String,
    pub output: Vec<String>,
    pub scroll: u16,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Info {
    pub agent_id: Uuid,
    pub hostname: String,
    pub username: String,
    pub os: String,
}
