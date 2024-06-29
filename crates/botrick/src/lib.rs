pub mod actors;
pub mod args;
pub mod color;
pub mod config;
pub mod data;
pub mod irc;

pub const VERSION_STR: &str = concat!(env!("CARGO_PKG_NAME"), " ", env!("CARGO_PKG_VERSION"),);
pub const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
