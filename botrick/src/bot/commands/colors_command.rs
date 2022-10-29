use crate::bot::{Command, CommandMessage, CommandResult};
use crate::color::{colorize, Color};
use anyhow::Context;
use irc::client::Sender;

pub struct ColorsCommand {
    pub sender: Sender,
    pub command: CommandMessage,
}
impl Command for ColorsCommand {
    fn execute(&self) -> CommandResult {
        let mut colstring = String::new();

        for col in 0..99 {
            colstring
                .push_str(colorize(Color::Num(col), None, format!("{:02}", col).as_str()).as_str());

            if (col + 1) % 20 == 0 {
                colstring.push_str("\r\n");
            } else {
                colstring.push(' ');
            }
        }

        self.sender
            .send_privmsg(&self.command.channel, colstring)
            .with_context(|| "Failed to send message")
    }
}
