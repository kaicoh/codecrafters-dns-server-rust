use super::{Answer, Message, Result};
use std::net::{SocketAddr, UdpSocket};

#[derive(Debug)]
pub struct Resolver {
    addr: Option<SocketAddr>,
}

impl Resolver {
    pub fn new(addr: Option<SocketAddr>) -> Self {
        Self { addr }
    }

    pub fn resolve(&self, buf: &[u8], socket: &UdpSocket) -> Result<Message> {
        let incoming = Message::try_from(buf);

        match incoming {
            Ok(msg) => {
                let id = msg.id();
                let mut reply_msg = Message::reply(msg);
                let mut answers: Vec<Answer> = vec![];

                for (i, q) in reply_msg.questions.iter().enumerate() {
                    match self.addr {
                        Some(ref forward_to) => {
                            let query = Message::query(id + i as u16, q);

                            socket.send_to(&query.as_bytes(), forward_to)?;

                            let mut buf = [0u8; 512];
                            let size = socket.recv(&mut buf)?;

                            let mut resolve_msg = Message::try_from(&buf[..size])?;
                            answers.append(&mut resolve_msg.answers);
                        }
                        None => {
                            answers.push(Answer::from(q));
                        }
                    }
                }

                for answer in answers {
                    reply_msg = reply_msg.set_answer(answer);
                }

                Ok(reply_msg)
            }
            Err(err) => {
                eprintln!("Cannot parse incoming message: {err}");
                Ok(Message::error())
            }
        }
    }
}
