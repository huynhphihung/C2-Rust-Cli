use clap::Parser;
use std::net::SocketAddr;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(long, default_value = "127.0.0.1")]
    pub server: String,
}
