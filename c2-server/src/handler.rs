use std::{
    io::{Error, ErrorKind, Result},
    net::SocketAddr,
    sync::Arc,
};

use tokio::{
    io::{AsyncWrite, AsyncWriteExt},
    net::tcp::OwnedWriteHalf,
    sync::Mutex,
};
use uuid::Uuid;

use crate::{
    enum_folder::message::{Message, OperatorMessage},
    operator::OperatorManager,
    protocol::{RegisterMessage, RequestTaskMessage},
    session::{Session, SessionInfo, SessionManager},
    task::{Task, TaskManager, TaskResult},
};

pub async fn handler_register(
    info: RegisterMessage,
    addr: SocketAddr,
    session_manager: &Arc<Mutex<SessionManager>>,
    current_session_id: &mut Option<Uuid>,
) {
    let session = Session::new(info, addr);
    let session_id = session.session_id;

    {
        let mut manager = session_manager.lock().await;
        manager.insert(session);

        println!("Total sessions: {}", manager.len());
    }
    *current_session_id = Some(session_id);
}

pub async fn handle_request_task<W>(
    request: RequestTaskMessage,
    task_manager: &Arc<Mutex<TaskManager>>,
    writer: &mut W,
) -> Result<()>
where
    W: AsyncWrite + Unpin,
{
    let task = {
        let mut manager = task_manager.lock().await;
        manager.next_task(&request.agent_id)
    };

    let response = match task {
        Some(task) => Message::Task(task),
        None => Message::NoTask,
    };

    let mut payload =
        serde_json::to_vec(&response).map_err(|error| Error::new(ErrorKind::InvalidData, error))?;

    payload.push(b'\n');

    writer.write_all(&payload).await?;

    Ok(())
}

pub async fn handle_response_task(
    result: TaskResult,
    operator_manager: &Arc<Mutex<OperatorManager>>,
) -> Result<()> {
    let response = Message::TaskResult(result);

    let mut payload = serde_json::to_vec(&response)?;
    payload.push(b'\n');

    let manager = operator_manager.lock().await;

    if let Some(writer) = &manager.writer {
        let mut writer = writer.lock().await;
        writer.write_all(&payload).await?;
    }

    Ok(())
}

pub async fn handle_request_session(
    session_manager: &Arc<Mutex<SessionManager>>,
    writer: &Arc<Mutex<OwnedWriteHalf>>,
) -> Result<()> {
    let mut manager = session_manager.lock().await;
    let session_infos = manager
        .sessions()
        .map(|session| SessionInfo::from(session))
        .collect();

    let response = OperatorMessage::SessionList(session_infos);
    let mut payload = serde_json::to_vec(&response)?;
    payload.push(b'\n');
    let mut writer = writer.lock().await;
    writer.write_all(&payload).await?;
    Ok(())
}
