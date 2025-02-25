#[macro_use]
mod macros;

mod error;
mod message;
mod server;

pub type Result<T> = std::result::Result<T, Error>;
pub use error::Error;
pub use server::Server;

pub fn handle_message(_buf: &[u8]) -> message::Message {
    message::Message::test()
}
