use std::{
    io::{Error, ErrorKind},
    net::SocketAddr,
};

use color_eyre::eyre::{self, Ok, Result};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{
        TcpStream,
        tcp::{OwnedReadHalf, OwnedWriteHalf},
    },
};

use crate::message::Message;

pub struct Client {
    pub server_addr: SocketAddr,
    pub writer: Option<OwnedWriteHalf>,
    pub reader: Option<BufReader<OwnedReadHalf>>,
}
impl Client {
    pub fn new(server_addr: SocketAddr) -> Self {
        Self {
            writer: None,
            reader: None,
            server_addr,
        }
    }

    pub async fn connect_to_server(&mut self) -> Result<()> {
        let stream = TcpStream::connect(self.server_addr).await?;
        let (read_half, write_half) = stream.into_split();

        self.writer = Some(write_half);
        self.reader = Some(BufReader::new(read_half));
        Ok(())
    }

    pub async fn send(&mut self, message: &Message) -> Result<()> {
        let writer = self
            .writer
            .as_mut()
            .ok_or_else(|| eyre::eyre!("Operator is not connected"))?;

        let mut json = serde_json::to_vec(message)
            .map_err(|error| Error::new(ErrorKind::InvalidData, error))?;

        json.push(b'\n');

        writer.write_all(&json).await?;

        Ok(())
    }

    fn disconnected(&mut self) {
        self.writer = None;
        self.reader = None;
    }
}

pub async fn receive_msg(reader: &mut BufReader<OwnedReadHalf>) -> Result<Message> {
    let mut payload = Vec::new();
    let bytes_read = reader.read_until(b'\n', &mut payload).await?;

    if bytes_read == 0 {
        return Err(eyre::eyre!("Server disconnected"));
    }

    if payload.last() == Some(&b'\n') {
        payload.pop();
    }

    if payload.last() == Some(&b'\n') {
        payload.pop();
    }

    let message = serde_json::from_slice(&payload)?;
    Ok(message)
}
