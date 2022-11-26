use crate::bot::Sender;
use crate::bot::Config;
use crate::bot::command::Command;

pub fn bots(command: Command, sender: Sender, config: Config) {
    let _ = sender
        .send_privmsg(
            &command.channel,
            String::from("Reporting in! [Rust] just %spork or %sporklike, yo."),
        );  
}