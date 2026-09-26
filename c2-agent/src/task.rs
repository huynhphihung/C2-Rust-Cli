use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct TaskResult {
    pub task_id: Uuid,
    pub agent_id: Uuid,
    pub stdout: String,
    pub stderr: String,
    pub success: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskMessage {
    pub task_id: Uuid,
    pub action: TaskAction,
    pub status: TaskStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TaskAction {
    Command(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    Sent,
    Completed,
    Failed,
    Running,
}
