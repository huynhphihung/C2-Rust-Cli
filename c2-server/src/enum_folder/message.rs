use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    protocol::{HeartBeatMessage, RegisterMessage, RequestTaskMessage},
    session::{Session, SessionInfo},
    task::{Task, TaskResult},
};

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum Message {
    Register(RegisterMessage),
    Heartbeat(HeartBeatMessage),
    RequestTask(RequestTaskMessage),
    NoTask,
    Task(Task),
    TaskResult(TaskResult),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskAction {
    Command(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum OperatorMessage {
    CreateTask { agent_id: Uuid, action: TaskAction },
    Interact { agent_id: Uuid },
    RequestSession,
    SessionList(Vec<SessionInfo>),
}
