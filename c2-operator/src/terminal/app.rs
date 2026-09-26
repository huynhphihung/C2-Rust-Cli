use std::{io::Stdout, net::SocketAddr, time::Duration};

use color_eyre::eyre::{self, Result};
use crossterm::event::{self, Event, KeyEvent};
use ratatui::{Terminal, backend::CrosstermBackend};
use tokio::sync::mpsc;

use crate::{
    client::{Client, receive_msg},
    message::{Message, ShellState},
    task::TaskAction,
    terminal::{
        key_handler::{menu, session, shell},
        state::{AppState, SessionState, View},
        ui::ui::ui,
    },
};

pub struct OperatorApp {
    pub app_state: AppState,
    pub client: Client,
    pub rx: mpsc::Receiver<Message>,
    pub tx: mpsc::Sender<Message>,
}

impl OperatorApp {
    pub fn new(server_addr: SocketAddr) -> Self {
        let (tx, rx) = mpsc::channel(100);
        Self {
            app_state: AppState::new(),
            client: Client::new(server_addr),
            rx,
            tx,
        }
    }

    pub fn handle_network_manages(&mut self) {
        while let Ok(message) = self.rx.try_recv() {
            match message {
                Message::TaskResult(result) => {
                    if let Some(session) = self
                        .app_state
                        .sessions
                        .iter_mut()
                        .find(|session| session.info.info.agent_id == result.agent_id)
                    {
                        if !result.stdout.is_empty() {
                            session.shell_state.output.push(result.stdout);
                        }

                        if !result.stderr.is_empty() {
                            session.shell_state.output.push(result.stderr);
                        }
                    }
                }
                Message::SessionList(sessions) => {
                    self.app_state.sessions = sessions
                        .into_iter()
                        .map(|info| SessionState {
                            info,
                            shell_state: ShellState::default(),
                        })
                        .collect();

                    if !self.app_state.sessions.is_empty() {
                        self.app_state.session_list_state.select(Some(0));
                    } else {
                        self.app_state.session_list_state.select(None);
                    }
                }
                _ => {}
            }
        }
    }

    pub async fn run_app(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    ) -> Result<()> {
        self.client.connect_to_server().await?;

        self.start_network_reader()?;

        loop {
            self.handle_network_manages();

            terminal.draw(|frame| ui(frame, self))?;

            if event::poll(Duration::from_millis(50))? {
                if let Event::Key(key) = event::read()? {
                    self.on_key(key).await?;
                }
            }

            if self.app_state.should_quit {
                break;
            }
        }
        Ok(())
    }

    pub fn start_network_reader(&mut self) -> Result<()> {
        let mut reader = self
            .client
            .reader
            .take()
            .ok_or_else(|| eyre::eyre!("Reader is not available"))?;

        let tx = self.tx.clone();

        tokio::spawn(async move {
            loop {
                match receive_msg(&mut reader).await {
                    Ok(message) => {
                        if tx.send(message).await.is_err() {
                            break;
                        }
                    }

                    Err(error) => {
                        eprintln!("Network error: {}", error);
                        break;
                    }
                }
            }
        });

        Ok(())
    }

    pub async fn requeset_sessions(&mut self) -> Result<()> {
        self.client.send(&Message::RequestSession).await?;

        Ok(())
    }

    pub fn current_session_mut(&mut self) -> Result<&mut SessionState> {
        let session_id = self
            .app_state
            .current_session
            .ok_or_else(|| eyre::eyre!("No session selected"))?;

        self.app_state
            .sessions
            .iter_mut()
            .find(|session| session.info.session_id == session_id)
            .ok_or_else(|| eyre::eyre!("No session found"))
    }

    pub async fn create_task(&mut self) -> Result<()> {
        let session_id = self
            .app_state
            .current_session
            .ok_or_else(|| eyre::eyre!("No session selected"))?;

        // Chỉ lấy data, không giữ mutable borrow
        let (agent_id, command) = {
            let session = self
                .app_state
                .sessions
                .iter()
                .find(|session| session.info.session_id == session_id)
                .ok_or_else(|| eyre::eyre!("Session not found"))?;

            let command = session.shell_state.input.trim().to_string();

            if command.is_empty() {
                return Ok(());
            }

            (session.info.info.agent_id, command)
        };
        // borrow session kết thúc ở đây

        // clear local command
        if command == "clear" {
            if let Some(session) = self
                .app_state
                .sessions
                .iter_mut()
                .find(|session| session.info.session_id == session_id)
            {
                session.shell_state.input.clear();
                session.shell_state.output.clear();
                session.shell_state.scroll = 0;
            }

            return Ok(());
        }

        let task = Message::CreateTask {
            agent_id,
            action: TaskAction::Command(command.clone()),
        };

        self.client.send(&task).await?;

        // borrow lại sau khi send xong
        if let Some(session) = self
            .app_state
            .sessions
            .iter_mut()
            .find(|session| session.info.session_id == session_id)
        {
            session.shell_state.output.push(format!("$ {}", command));

            session.shell_state.input.clear();
        }

        Ok(())
    }

    pub async fn on_key(&mut self, key: KeyEvent) -> Result<()> {
        match self.app_state.current_window {
            View::Menu => menu::handle_key(self, key).await?,
            View::Sessions => session::handle_key(&mut self.app_state, key),
            View::Shell => shell::handle_key(self, key).await?,
        }
        Ok(())
    }
}
