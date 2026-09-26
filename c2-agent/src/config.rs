use std::{
    net::{Ipv4Addr, SocketAddr},
    time::Duration,
};

#[derive(Debug)]
pub struct Config {
    pub server_addr: SocketAddr,
    pub reconnect_delay: Duration,
    pub heartbeat_interval: Duration,
    pub task_delay: Duration,
}

impl Config {
    pub fn new() -> Self {
        Self {
            server_addr: SocketAddr::new(
                std::net::IpAddr::V4(Ipv4Addr::new(192, 168, 0, 104)),
                8888,
            ),
            reconnect_delay: Duration::from_secs(5),
            heartbeat_interval: Duration::from_secs(5),
            task_delay: Duration::from_millis(100),
        }
    }
}
