use std::{io::stdout, time::Duration};

use crossterm::{
    ExecutableCommand,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};

use crate::{config::Config, message::Message, task::TaskAction, terminal::app::OperatorApp};

mod client;
mod command;
mod config;
mod message;
mod task;
mod terminal;
mod utils;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    let config = Config::new();
    color_eyre::install()?;
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;
    let mut app = OperatorApp::new(config.server_addr);

    let result = app.run_app(&mut terminal).await;

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    result
}
