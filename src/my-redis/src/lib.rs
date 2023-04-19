
mod db;
use db::Db;

mod frame;
pub use frame::Frame;

mod parse;
use parse::{Parse, ParseError};

mod connection;
pub use connection::Connection;

mod cmd;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;
