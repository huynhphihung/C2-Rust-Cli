use std::io::Result;

use crate::config::Config;
use crate::server::Server;

mod client_handler;
mod config;
mod enum_folder;
mod handler;
mod operator;
mod operator_handler;
mod protocol;
mod server;
mod session;
mod task;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::new();
    let server = Server::new(config.agent_addr, config.operator_addr).await?;

    tokio::try_join!(server.run_agent(), server.run_operator())?;
    Ok(())
}
