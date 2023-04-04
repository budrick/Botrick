use crate::werdleactor::WerdleActorHandle;

use super::{Command, CommandMessage, CommandResult};
use irc::client::Sender;

pub struct WerdleCommand {
    pub sender: Sender,
    pub command: CommandMessage,
    pub werdle_handle: WerdleActorHandle,
}
impl Command for WerdleCommand {
    fn execute(&self) -> CommandResult {
        println!("{:?}", self.command.clone());
        println!("WerdleCommand executed");

        let input = self.command.params.trim();

        if input.is_empty() {
            self.werdle_handle.get_state(self.command.clone());
        } else {
            self.werdle_handle
                .guess(self.command.params.clone(), self.command.clone());
        }
        // let a = self.werdle_handle.get_word(self.command.clone());

        Ok(())
    }
}
