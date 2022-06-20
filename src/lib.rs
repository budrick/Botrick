pub mod sporker;

use irc::proto::Message;
use lazy_static::lazy_static;
use regex::Regex;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

pub type Channelizer = (UnboundedSender<Message>, UnboundedReceiver<Message>);
pub enum BotCommand {
    Spork(String),
    Sporklike(String),
    Bots,
}

pub fn parse_command(text: &str) -> Option<BotCommand> {
    lazy_static! {
        static ref COMMAND_RE: Regex = Regex::new(r"^%(\S+)(\s*)").unwrap();
    }

    if text.starts_with(".bots") {
        return Some(BotCommand::Bots);
    }

    let maybe_cmd = COMMAND_RE.captures(text);
    let (cmd, spaces) = match maybe_cmd {
        Some(matches) => (
            matches.get(1).unwrap().as_str(),
            matches.get(2).unwrap().as_str(),
        ),
        _ => ("", ""),
    };

    let cmd_text = text
        .strip_prefix(format!("%{}{}", cmd, spaces).as_str())
        .map_or("", |v| v);

    match cmd {
        "spork" => Some(BotCommand::Spork(cmd_text.to_string())),
        "sporklike" => Some(BotCommand::Sporklike(cmd_text.to_string())),
        _ => None,
    }
}

pub fn should_log(msg: String) -> bool {
    !msg.starts_with('\u{001}') || msg.starts_with("\u{001}ACTION")
}
