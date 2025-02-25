use crate::{err, message::Message, Result};
use std::net::{SocketAddr, ToSocketAddrs, UdpSocket};

const BUF_SIZE: usize = 512;

type Handler = dyn Fn(&[u8]) -> Message;

pub struct Server {
    addr: SocketAddr,
    handler: Option<Box<Handler>>,
}

impl Server {
    fn new(addr: SocketAddr) -> Self {
        Self {
            addr,
            handler: None,
        }
    }

    pub fn bind<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        addr.to_socket_addrs()?
            .next()
            .ok_or(err!("SocketAddr is None"))
            .map(Self::new)
    }

    pub fn handler<F>(self, handler: F) -> Self
    where
        F: Fn(&[u8]) -> Message + 'static,
    {
        Self {
            handler: Some(Box::new(handler)),
            ..self
        }
    }

    pub fn run(self) -> Result<()> {
        let socket = UdpSocket::bind(self.addr)?;
        let handler = self.handler.ok_or(err!("Message handler is not set"))?;
        let mut buf = clean_buf();

        while let Ok((size, addr)) = socket.recv_from(&mut buf) {
            let msg = handler(&buf[..size]);
            socket.send_to(&msg.as_bytes(), addr)?;
            buf = clean_buf();
        }

        Ok(())
    }
}

fn clean_buf() -> [u8; BUF_SIZE] {
    [0u8; BUF_SIZE]
}
