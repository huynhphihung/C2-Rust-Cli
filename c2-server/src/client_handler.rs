use std::{
    io::{Error, ErrorKind, Result},
    net::SocketAddr,
    sync::Arc,
};

use tokio::{
    io::{AsyncBufReadExt, BufReader},
    net::TcpStream,
    sync::Mutex,
    time::Instant,
};
use uuid::Uuid;

use crate::{
    enum_folder::message::{Message, TaskAction},
    handler::{handle_request_task, handle_response_task, handler_register},
    operator::{self, OperatorManager},
    session::SessionManager,
    task::{Task, TaskManager},
};

pub async fn handle_client(
    stream: TcpStream,
    addr: SocketAddr,
    session_manager: Arc<Mutex<SessionManager>>,
    task_manager: Arc<Mutex<TaskManager>>,
    operator_manager: &Arc<Mutex<OperatorManager>>,
) -> Result<()> {
    let (read_half, mut write_half) = stream.into_split();
    let mut reader = BufReader::new(read_half);

    let mut payload = Vec::new();
    let mut current_session_id: Option<Uuid> = None;
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

        let message: Message = serde_json::from_slice(&payload).map_err(|error| {
            Error::new(ErrorKind::InvalidData, format!("Invalid message: {error}"))
        })?;

        println!("Received message: {:?}", message);

        match message {
            Message::Register(info) => {
                handler_register(info, addr, &session_manager, &mut current_session_id).await;
            }
            Message::Heartbeat(heartbeat) => {
                let mut manager = session_manager.lock().await;

                if let Some(session) = manager.get_mut(&heartbeat.agent_id) {
                    session.last_seen = Instant::now()
                }
            }
            Message::TaskResult(result) => {
                {
                    let mut manager = task_manager.lock().await;

                    manager.complete_task(&result.task_id);
                }
                println!("Task result received: {result:?}");
                handle_response_task(result, operator_manager).await?;
            }
            Message::NoTask => {}
            Message::RequestTask(request) => {
                handle_request_task(request, &task_manager, &mut write_half).await?;
            }
            Message::Task(task) => {
                println!("Task: {:?}", task);
            }
        }
    }

    if let Some(session_id) = current_session_id {
        let mut manager = session_manager.lock().await;

        manager.remove(&session_id);
    }

    Ok(())
}
