pub mod args;
pub mod bot;
// mod channelizer;
pub mod color;
pub mod config;
pub mod logger;
pub mod werdleactor;

pub mod actors;
pub mod irc;

pub const VERSION_STR: &str = concat!(env!("CARGO_PKG_NAME"), " ", env!("CARGO_PKG_VERSION"),);
