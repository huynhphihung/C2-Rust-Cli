use std::{
    io::{Result, pipe},
    process::{self, Command, Stdio},
};
use tokio::{
    io::AsyncWriteExt,
    net::{TcpStream, tcp::OwnedWriteHalf},
};
use uuid::Uuid;

use crate::{
    agent::Agent,
    shell::Shell,
    task::{TaskAction, TaskMessage, TaskResult},
    types::message::Message,
};

pub fn task_execute(task: TaskMessage, agent_id: Uuid, shell: &mut Shell) -> TaskResult {
    let mut stdout = String::new();
    let mut stderr = String::new();
    let mut success = true;

    match task.action {
        TaskAction::Command(command) => match shell.execute(&command) {
            Ok(result) => {
                stdout = String::from_utf8_lossy(&result.stdout).into_owned();
                stderr = String::from_utf8_lossy(&result.stderr).into_owned();
                success = result.status.success();
            }

            Err(e) => {
                stderr = format!("failed to execute command: {e}");
                success = false;
            }
        },
    };

    TaskResult {
        task_id: task.task_id,
        agent_id,
        stdout,
        stderr,
        success,
    }
}

pub async fn task_handler(
    writer: &mut OwnedWriteHalf,
    task: TaskMessage,
    agent: &mut Agent,
) -> Result<()> {
    let result = task_execute(task, agent.info.agent_id, &mut agent.shell);
    let response = Message::TaskResult(result);
    let mut payload = serde_json::to_vec(&response)?;
    payload.push(b'\n');
    writer.write_all(&payload).await?;
    Ok(())
}
