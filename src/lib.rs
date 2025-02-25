#[macro_use]
mod macros;

mod error;
mod message;
mod server;
mod utils;

pub type Result<T> = std::result::Result<T, Error>;
pub use error::Error;
pub use server::Server;

use message::Message;

pub fn handle_message(buf: &[u8]) -> Message {
    match Message::try_from(buf) {
        Ok(msg) => Message::test(msg),
        Err(err) => {
            eprintln!("{err}");
            Message::error()
        }
    }
}
