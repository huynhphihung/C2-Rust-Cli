use std::collections::{HashMap, VecDeque};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{enum_folder::message::TaskAction, protocol::TaskStatus};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Task {
    pub task_id: Uuid,
    pub action: TaskAction,
    pub status: TaskStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskResult {
    pub task_id: Uuid,
    pub stdout: String,
    pub stderr: String,
    pub success: bool,
    pub agent_id: Uuid,
}

impl Task {
    pub fn new(action: TaskAction) -> Self {
        Self {
            task_id: Uuid::new_v4(),
            action,
            status: TaskStatus::Pending,
        }
    }
}

pub struct TaskManager {
    pub tasks: HashMap<Uuid, VecDeque<Task>>,
    pub running_tasks: HashMap<Uuid, Task>,
}

impl TaskManager {
    pub fn new() -> Self {
        Self {
            tasks: HashMap::new(),
            running_tasks: HashMap::new(),
        }
    }

    pub fn add_task(&mut self, agent_id: Uuid, task: Task) {
        self.tasks.entry(agent_id).or_default().push_back(task);
    }

    pub fn next_task(&mut self, agent_id: &Uuid) -> Option<Task> {
        let mut task = self.tasks.get_mut(agent_id)?.pop_front()?;

        task.status = TaskStatus::Running;
        self.running_tasks.insert(task.task_id, task.clone());

        Some(task)
    }

    pub fn complete_task(&mut self, task_id: &Uuid) -> Option<Task> {
        let mut task = self.running_tasks.remove(task_id)?;
        task.status = TaskStatus::Completed;

        Some(task)
    }
}
