use std::{io::Result, net::SocketAddr, sync::Arc};

use tokio::{net::TcpListener, sync::Mutex};

use crate::{
    client_handler::handle_client, operator::OperatorManager, operator_handler::handle_operator,
    session::SessionManager, task::TaskManager,
};

pub struct Server {
    pub agent_listener: TcpListener,
    pub operator_listener: TcpListener,
    pub session_manager: Arc<Mutex<SessionManager>>,
    pub task_manager: Arc<Mutex<TaskManager>>,
    pub operator_manager: Arc<Mutex<OperatorManager>>,
}

impl Server {
    pub async fn new(server_addr: SocketAddr, operator_addr: SocketAddr) -> Result<Self> {
        let agent_listener = TcpListener::bind(server_addr).await?;
        let operator_listener = TcpListener::bind(operator_addr).await?;

        Ok(Self {
            agent_listener,
            operator_listener,
            session_manager: Arc::new(Mutex::new(SessionManager::new())),
            task_manager: Arc::new(Mutex::new(TaskManager::new())),
            operator_manager: Arc::new(Mutex::new(OperatorManager::new())),
        })
    }

    pub async fn run_agent(&self) -> Result<()> {
        println!("Server started");

        loop {
            let (stream, addr) = self.agent_listener.accept().await?;

            println!("Client connected: {}", addr);

            let session_manager = Arc::clone(&self.session_manager);
            let task_manager = Arc::clone(&self.task_manager);
            let operator_manager = Arc::clone(&self.operator_manager);

            tokio::spawn(async move {
                if let Err(e) = handle_client(
                    stream,
                    addr,
                    session_manager,
                    task_manager,
                    &operator_manager,
                )
                .await
                {
                    eprintln!("Client error: {}", e)
                }
            });
        }
    }

    pub async fn run_operator(&self) -> Result<()> {
        loop {
            let (stream, addr) = self.operator_listener.accept().await?;
            println!("Operator is connected: {}", addr);

            let task_manager = Arc::clone(&self.task_manager);
            let operator_manager = Arc::clone(&self.operator_manager);
            let session_manager = Arc::clone(&self.session_manager);

            tokio::spawn(async move {
                if let Err(e) = handle_operator(
                    stream,
                    addr,
                    task_manager,
                    session_manager,
                    operator_manager,
                )
                .await
                {
                    eprintln!("Operator error: {}", e)
                }
            });
        }
    }
}
