use std::{
    io::{Error, ErrorKind, Result},
    net::SocketAddr,
    sync::Arc,
};

use tokio::{
    io::{AsyncBufReadExt, BufReader},
    net::TcpStream,
    sync::Mutex,
};

use crate::{
    enum_folder::message::{Message, OperatorMessage, TaskAction},
    handler::handle_request_session,
    operator::OperatorManager,
    session::SessionManager,
    task::{Task, TaskManager},
};

pub async fn handle_operator(
    stream: TcpStream,
    addr: SocketAddr,
    task_manager: Arc<Mutex<TaskManager>>,
    session_manager: Arc<Mutex<SessionManager>>,
    operator_manager: Arc<Mutex<OperatorManager>>,
) -> Result<()> {
    let (read_half, write_half) = stream.into_split();
    let mut reader = BufReader::new(read_half);
    let mut writer = Arc::new(Mutex::new(write_half));

    {
        let mut manager = operator_manager.lock().await;
        manager.writer = Some(writer.clone())
    }

    let mut payload = Vec::new();
    loop {
        payload.clear();
        let bytes_read = reader.read_until(b'\n', &mut payload).await?;

        if bytes_read == 0 {
            println!("Client disconnected: {:?}", addr);
            break;
        }

        if payload.last() == Some(&b'\n') {
            payload.pop();
        }

        if payload.last() == Some(&b'\r') {
            payload.pop();
        }

        if payload.is_empty() {
            continue;
        }

        let message: OperatorMessage = serde_json::from_slice(&payload).map_err(|error| {
            Error::new(ErrorKind::InvalidData, format!("Invalid message: {error}"))
        })?;

        println!("Message: {:?}", message);

        match message {
            OperatorMessage::CreateTask { agent_id, action } => {
                let task = Task::new(action);
                task_manager.lock().await.add_task(agent_id, task);
            }
            OperatorMessage::Interact { agent_id } => {}
            OperatorMessage::RequestSession => {
                handle_request_session(&session_manager, &mut writer).await?
            }
            other => {
                todo!()
            }
        }
    }

    Ok(())
}
