use std::{collections::HashMap, net::SocketAddr};

use serde::{Deserialize, Serialize};
use tokio::time::Instant;
use uuid::Uuid;

use crate::protocol::RegisterMessage;

#[derive(Debug, Clone)]
pub struct Session {
    pub session_id: Uuid,
    pub info: RegisterMessage,
    pub peer_addr: SocketAddr,
    pub connected_at: Instant,
    pub last_seen: Instant,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SessionInfo {
    pub session_id: Uuid,
    pub peer_addr: SocketAddr,
    pub info: RegisterMessage,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum SessionStatus {
    Online,
    Offline,
}

#[derive(Debug)]
pub struct SessionManager {
    pub session_manager: HashMap<Uuid, Session>,
}

impl From<&Session> for SessionInfo {
    fn from(session: &Session) -> Self {
        Self {
            session_id: session.session_id,
            peer_addr: session.peer_addr,
            info: session.info.clone(),
        }
    }
}

impl Session {
    pub fn new(agent_info: RegisterMessage, peer_addr: SocketAddr) -> Self {
        Self {
            session_id: Uuid::new_v4(),
            info: agent_info,
            peer_addr,
            connected_at: Instant::now(),
            last_seen: Instant::now(),
        }
    }
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            session_manager: HashMap::new(),
        }
    }

    pub fn insert(&mut self, session: Session) {
        self.session_manager.insert(session.session_id, session);
    }

    pub fn remove(&mut self, id: &Uuid) -> Option<Session> {
        self.session_manager.remove(id)
    }

    pub fn get(&mut self, id: &Uuid) -> Option<&Session> {
        self.session_manager.get(id)
    }

    pub fn get_mut(&mut self, id: &Uuid) -> Option<&mut Session> {
        self.session_manager.get_mut(id)
    }

    pub fn list(&mut self) -> impl Iterator<Item = (&Uuid, &Session)> {
        self.session_manager.iter()
    }

    pub fn sessions(&mut self) -> impl Iterator<Item = &Session> {
        self.session_manager.values()
    }

    pub fn len(&mut self) -> usize {
        self.session_manager.len()
    }
}
