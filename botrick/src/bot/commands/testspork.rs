use crate::bot::mention_nick;

use crate::bot::commands::{Command, CommandResult};
use crate::bot::command::create_bot_command;
use crate::bot::CommandParams;
use anyhow::Context;
create_bot_command!(SporkTestCommand, {
    let db = sporker::getdb()?;
        let s = sporker::Spork::new(db);

        let words: Vec<&str> = self.params.command.params.split_whitespace().collect();
        let startw = if !words.is_empty() {
            s.start_with_word(words[0])
        } else {
            s.start()
        };

        let output: String = match startw {
            Some(word) => {
                let mut words = sporker::build_words(word, &s);
                words.insert(0, mention_nick(&self.params.command.nick));
                words.join(" ")
            }
            _ => String::from("Couldn't do it could I"),
        };

        self.params.sender
            .send_privmsg(&self.params.command.channel, output)
            .with_context(|| "Failed to send message")
}, self);