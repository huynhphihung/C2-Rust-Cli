use std::{
    io::{Error, ErrorKind, Result},
    net::SocketAddr,
};

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{
        TcpStream,
        tcp::{OwnedReadHalf, OwnedWriteHalf},
    },
};

use crate::types::message::Message;

pub struct Client {
    pub server_addr: SocketAddr,
    pub reader: Option<BufReader<OwnedReadHalf>>,
    pub writer: Option<OwnedWriteHalf>,
}

impl Client {
    pub fn new(server_addr: SocketAddr) -> Self {
        Self {
            server_addr,
            reader: None,
            writer: None,
        }
    }
    pub async fn connect_to_server(&mut self) -> Result<()> {
        let stream = TcpStream::connect(self.server_addr).await?;
        let (read_half, write_half) = stream.into_split();

        self.reader = Some(BufReader::new(read_half));
        self.writer = Some(write_half);
        Ok(())
    }

    pub async fn send(&mut self, message: &Message) -> Result<()> {
        let writer = self
            .writer
            .as_mut()
            .ok_or_else(|| Error::new(ErrorKind::NotConnected, "Client is not connected"))?;

        let mut json = serde_json::to_vec(message)
            .map_err(|error| Error::new(ErrorKind::InvalidData, error))?;

        println!("{}", String::from_utf8_lossy(&json));

        json.push(b'\n');

        writer.write_all(&json).await?;

        Ok(())
    }

    pub async fn receive_msg(&mut self) -> Result<Message> {
        let reader = self
            .reader
            .as_mut()
            .ok_or_else(|| Error::new(ErrorKind::NotConnected, "Client is not connected"))?;

        let mut line = String::new();
        let bytes_read = reader.read_line(&mut line).await?;

        if bytes_read == 0 {
            self.disconnected();
            return Err(Error::new(ErrorKind::UnexpectedEof, "Server disconnected"));
        }

        serde_json::from_str(line.trim_end())
            .map_err(|error| Error::new(ErrorKind::InvalidData, error))
    }

    pub fn disconnected(&mut self) {
        self.reader = None;
        self.writer = None
    }
}
