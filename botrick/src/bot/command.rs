use std::collections::HashMap;

use irc::{client::Sender, proto::Message, proto::Command::*};
use crate::config::Config;
use regex::Regex;

// Handler function signature
pub type CommandHandler = fn(Command, Sender, Config);

#[derive(Debug, Clone)]
pub struct Command {
    pub command: String,
    pub params: String,
    pub channel: String,
    pub nick: String,
}

pub struct Commander {
    commands: HashMap<String, CommandHandler>,
    config: Config,
    regex: Regex,
}
impl Commander {
    pub fn new(config: Config) -> Self {
        let regex = Regex::new(r"^(\S+)(\s*)(.*)").unwrap();
        Self {
            commands: HashMap::new(),
            config,
            regex
        }
    }

    pub fn insert(&mut self, command: &str, handler: CommandHandler) {
        self.commands.insert(command.to_string(), handler);
    }

    pub fn process(&self, message: &Message) {
        let cmd = self.extract(message);
        println!("{:#?}", cmd);
    }

    pub fn extract(&self, message: &Message) -> Option<Command> {
        if let PRIVMSG(ref _channel, ref text) = message.command {
            if text.starts_with('\u{001}') {
                return None;
            }

            // Where was the message sent, and who sent it? We need these to allow responses.
            let responsetarget = message.response_target().unwrap_or("");
            let responsenick = message.source_nickname().unwrap_or("");

            // Get first word and spaces following. No matches = 'default' command
            let firstword = self.regex.captures(text);
            let (cmd, _, cmd_params) = match firstword {
                Some(matches) => (
                    matches.get(1).unwrap().as_str(),
                    matches.get(2).unwrap().as_str(),
                    matches.get(3).unwrap().as_str()
                ),
                // No (probable) commands? Dispatch a special builtin
                _ => ("default", "", text.as_str()),
            };

            // If there is a valid command, create a CommandMessage to pass around to handlers.
            Some(Command {
                nick: String::from(responsenick),
                channel: String::from(responsetarget),
                command: String::from(cmd),
                params: String::from(cmd_params),
            })
        } else {
            None
        }
    }

    pub fn get_handler(&self, command: Command) -> Option<&CommandHandler> {
        if self.commands.contains_key((&command.command)) {
            self.commands.get(&command.command)
        } else {
            self.commands.get("default")
        }
    }

    pub fn command_exists(&self, command: &Command) -> bool {
        self.commands.contains_key(&command.command)
    }

}