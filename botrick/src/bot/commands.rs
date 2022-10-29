mod colors_command;
mod sleep_command;
mod spork_commands;

pub use colors_command::ColorsCommand;
pub use sleep_command::SleepCommand;
pub use spork_commands::*;

pub type CommandResult = anyhow::Result<()>;

pub trait Command {
    fn execute(&self) -> CommandResult;
}
#[derive(Debug, Clone)]
pub struct CommandMessage {
    pub command: String,
    pub params: String,
    pub channel: String,
    pub nick: String,
}
