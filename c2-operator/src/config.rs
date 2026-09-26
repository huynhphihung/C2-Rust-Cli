use std::net::{Ipv4Addr, SocketAddr};

pub struct Config {
    pub server_addr: SocketAddr,
}

impl Config {
    pub fn new() -> Self {
        Self {
            server_addr: SocketAddr::new(std::net::IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5555),
        }
    }
}
