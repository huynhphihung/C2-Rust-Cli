use std::sync::Arc;
use tokio::sync::Mutex;

use tokio::net::tcp::OwnedWriteHalf;

pub struct OperatorManager {
    pub writer: Option<Arc<Mutex<OwnedWriteHalf>>>,
}

impl OperatorManager {
    pub fn new() -> Self {
        Self { writer: None }
    }
}
