use serde::{Deserialize, Serialize};

use crate::{
    agent::AgentInfo,
    protocol::{HeartBeatMessage, RequestTaskMessage},
    task::{TaskMessage, TaskResult},
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum Message {
    Register(AgentInfo),
    Heartbeat(HeartBeatMessage),
    TaskResult(TaskResult),
    Task(TaskMessage),
    NoTask,
    RequestTask(RequestTaskMessage),
}
