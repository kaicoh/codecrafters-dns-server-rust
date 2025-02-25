#[macro_use]
mod macros;

mod error;
mod server;

pub type Result<T> = std::result::Result<T, Error>;
pub use error::Error;
pub use server::Server;
