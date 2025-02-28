use crate::{err, resolver::Resolver, Result};
use std::net::{SocketAddr, ToSocketAddrs, UdpSocket};

const BUF_SIZE: usize = 512;

pub struct Server {
    addr: SocketAddr,
    resolver: Option<Resolver>,
}

impl Server {
    fn new(addr: SocketAddr) -> Self {
        Self {
            addr,
            resolver: None,
        }
    }

    pub fn bind<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        addr.to_socket_addrs()?
            .next()
            .ok_or(err!("SocketAddr is None"))
            .map(Self::new)
    }

    pub fn resolver<A: ToSocketAddrs>(self, addr: Option<A>) -> Result<Self> {
        let addr = addr
            .map(|a| a.to_socket_addrs())
            .transpose()?
            .and_then(|mut addrs| addrs.next());

        Ok(Self {
            resolver: Some(Resolver::new(addr)),
            ..self
        })
    }

    pub fn run(self) -> Result<()> {
        let socket = UdpSocket::bind(self.addr)?;
        let resolver = self.resolver.ok_or(err!("Message resolver is not set"))?;
        let mut buf = clean_buf();

        while let Ok((size, addr)) = socket.recv_from(&mut buf) {
            let msg = resolver.resolve(&buf[..size], &socket)?;
            socket.send_to(&msg.as_bytes(), addr)?;
            buf = clean_buf();
        }

        Ok(())
    }
}

fn clean_buf() -> [u8; BUF_SIZE] {
    [0u8; BUF_SIZE]
}
