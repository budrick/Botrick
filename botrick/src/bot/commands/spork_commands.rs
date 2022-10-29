use crate::bot::mention_nick;

use super::{Command, CommandMessage, CommandResult};
use anyhow::Context;
use irc::client::Sender;

pub struct SporkCommand {
    pub sender: Sender,
    pub command: CommandMessage,
}
impl Command for SporkCommand {
    fn execute(&self) -> CommandResult {
        let db = sporker::getdb()?;
        let s = sporker::Spork::new(db);

        let words: Vec<&str> = self.command.params.split_whitespace().collect();
        let startw = if !words.is_empty() {
            s.start_with_word(words[0])
        } else {
            s.start()
        };

        let output: String = match startw {
            Some(word) => {
                let mut words = sporker::build_words(word, &s);
                words.insert(0, mention_nick(&self.command.nick));
                words.join(" ")
            }
            _ => String::from("Couldn't do it could I"),
        };

        self.sender
            .send_privmsg(&self.command.channel, output)
            .with_context(|| "Failed to send message")
    }
}

pub struct SporklikeCommand {
    pub sender: Sender,
    pub command: CommandMessage,
}
impl Command for SporklikeCommand {
    fn execute(&self) -> CommandResult {
        let db = sporker::getdb()?;
        let s = sporker::Spork::new(db);

        // Get all our cmdline args
        let words: Vec<&str> = self.command.params.split_whitespace().collect();

        // Fewer than 2 args? Go away.
        if words.is_empty() {
            return self
                .sender
                .send_privmsg(
                    &self.command.channel,
                    String::from("Talking about nobody is it"),
                )
                .with_context(|| String::from("Failed to send message"));
        }

        let saidby = words[0];

        // If we have more than one arg, take the first one and run with it.
        // Otherwise, find out own start word. With blackjack. And hookers.
        let startw = match words.len() {
            1 => s.start_like(saidby),
            _ => s.start_with_word_like(words[1], saidby),
        };

        let output: String = match startw {
            Some(word) => {
                let mut words = sporker::build_words_like(word, &s, saidby);
                words.insert(0, mention_nick(&self.command.nick));
                words.join(" ")
            }
            _ => String::from("Couldn't do it could I"),
        };

        self.sender
            .send_privmsg(&self.command.channel, output)
            .with_context(|| "Failed to send message")
    }
}
