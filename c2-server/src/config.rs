use std::net::{IpAddr, Ipv4Addr, SocketAddr};

pub struct Config {
    pub operator_addr: SocketAddr,
    pub agent_addr: SocketAddr,
}

impl Config {
    pub fn new() -> Self {
        Self {
            agent_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 8888),
            operator_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 5555),
        }
    }
}
