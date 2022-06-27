use crate::sporker;
use anyhow::Context;
use lazy_static::lazy_static;
use regex::Regex;

pub fn parse_command(message: &irc::proto::Message) -> Option<CommandMessage> {
    if let irc::proto::Command::PRIVMSG(ref _channel, ref text) = message.command {
        lazy_static! {
            static ref COMMAND_RE: Regex = Regex::new(r"^%(\S+)(\s*)").unwrap();
        }

        // Where was the message sent, and who sent it? We need these to allow responses.
        let responsetarget = message.response_target().unwrap_or("");
        let responsenick = message.source_nickname().unwrap_or("");

        // Short-circuit command parsing for `.bots`
        // .bots is a special case that doesn't use our regular prefix
        if text.starts_with(".bots") {
            return Some(CommandMessage {
                nick: String::from(responsenick),
                channel: String::from(responsetarget),
                command: String::from(".bots"),
                text: String::from(""),
            });
        }

        // Try and regex out a "normal" command. `cmd` becomes the command itself, `spaces` any following whitespace
        let maybe_cmd = COMMAND_RE.captures(text);
        let (cmd, spaces) = match maybe_cmd {
            Some(matches) => (
                matches.get(1).unwrap().as_str(),
                matches.get(2).unwrap().as_str(),
            ),
            _ => ("", ""),
        };

        // Command text is the rest of the line minus the command + space prefix
        let cmd_text = text
            .strip_prefix(format!("%{}{}", cmd, spaces).as_str())
            .map_or("", |v| v);

        // If there is a valid command, create a CommandMessage to pass around to handlers.
        match cmd {
            "" => None,
            _ => Some(CommandMessage {
                nick: String::from(responsenick),
                channel: String::from(responsetarget),
                command: String::from(cmd),
                text: String::from(cmd_text),
            }),
        }
    } else {
        None
    }
}

pub fn prepend_nick(nick: &String) -> String {
    format!("{}: ", nick)
}

// Dispatch handlers for BotCommands
pub fn handle_command_message(cmd: CommandMessage, sender: irc::client::Sender) -> CommandResult {
    match cmd.command.as_str() {
        ".bots" => BotsCommand::execute(cmd, sender),
        "spork" => SporkCommand::execute(cmd, sender),
        "sporklike" => SporklikeCommand::execute(cmd, sender),
        _ => Ok(()),
    }
}

pub type CommandResult = anyhow::Result<()>;

pub trait Command {
    fn execute(command: CommandMessage, sender: irc::client::Sender) -> CommandResult;
}

pub struct CommandMessage {
    pub command: String,
    pub text: String,
    pub channel: String,
    pub nick: String,
}

pub struct BotsCommand;
impl Command for BotsCommand {
    fn execute(command: CommandMessage, sender: irc::client::Sender) -> CommandResult {
        sender
            .send_privmsg(
                command.channel,
                String::from("Reporting in! [Rust] just %spork or %sporklike, yo."),
            )
            .with_context(|| "Failed to send message")
    }
}

pub struct SporkCommand;
impl Command for SporkCommand {
    fn execute(command: CommandMessage, sender: irc::client::Sender) -> CommandResult {
        let db = sporker::getdb()?;
        let s = sporker::Spork::new(db);

        let words: Vec<&str> = command.text.split_whitespace().collect();
        let startw = if !words.is_empty() {
            s.start_with_word(words[0])
        } else {
            s.start()
        };

        let output: String = match startw {
            Some(word) => {
                let mut words = sporker::build_words(word, &s);
                words.insert(0, prepend_nick(&command.nick));
                words.join(" ")
            }
            _ => String::from("Couldn't do it could I"),
        };

        sender
            .send_privmsg(command.channel, output)
            .with_context(|| "Failed to send message")
    }
}

pub struct SporklikeCommand;
impl Command for SporklikeCommand {
    fn execute(command: CommandMessage, sender: irc::client::Sender) -> CommandResult {
        let db = sporker::getdb()?;
        let s = sporker::Spork::new(db);

        // Get all our cmdline args
        let words: Vec<&str> = command.text.split_whitespace().collect();

        // Fewer than 2 args? Go away.
        if words.is_empty() {
            return sender
                .send_privmsg(command.channel, String::from("Talking about nobody is it"))
                .with_context(|| format!("Failed to send message"));
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
                words.insert(0, prepend_nick(&command.nick));
                words.join(" ")
            }
            _ => String::from("Couldn't do it could I"),
        };

        sender
            .send_privmsg(command.channel, output)
            .with_context(|| "Failed to send message")
    }
}
