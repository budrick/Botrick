use crate::{bot::{Command, CommandMessage, CommandResult}, channelizer::StringSender};
use irc::client::Sender;

#[allow(dead_code)]
pub struct WerdleCommand {
    pub sender: Sender,
    pub command: CommandMessage,
    pub wsender: StringSender,
}
impl Command for WerdleCommand {
    fn execute(&self) -> CommandResult {
        let _ = self.wsender.send(self.command.params.clone());
        Ok(())
    }
}
