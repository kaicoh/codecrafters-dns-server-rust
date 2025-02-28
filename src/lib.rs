#[macro_use]
mod macros;

mod args;
mod error;
mod message;
mod resolver;
mod server;
mod utils;

pub type Result<T> = std::result::Result<T, Error>;

pub use args::Args;
pub use error::Error;
pub use server::Server;

use message::{Answer, Message};
