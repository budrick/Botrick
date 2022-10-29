use crate::bot::{Command, CommandMessage, CommandResult};
use irc::client::Sender;

#[allow(dead_code)]
pub struct SleepCommand {
    pub sender: Sender,
    pub command: CommandMessage,
}
impl Command for SleepCommand {
    fn execute(&self) -> CommandResult {
        println!("Sleeping for 10...");
        std::thread::sleep(std::time::Duration::from_secs(10));
        println!("Waking after 10...");
        Ok(())
    }
}
