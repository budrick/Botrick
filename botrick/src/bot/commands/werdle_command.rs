use crate::{
    bot::{Command, CommandMessage, CommandResult},
    werdplay::{WerdleMessage, WerdleSender},
};
use irc::client::Sender;

#[allow(dead_code)]
pub struct WerdleCommand {
    pub sender: Sender,
    pub command: CommandMessage,
    pub wsender: WerdleSender,
}

impl Command for WerdleCommand {
    fn execute(&self) -> CommandResult {
        let m = WerdleMessage {
            guess: self.command.params.clone(),
            ircsender: self.sender.clone(),
            irctarget: self.command.channel.clone(),
        };
        let _ = self.wsender.send(m);
        Ok(())
    }
}
