pub mod sporker;

use irc::{proto::Message, client::Sender};
use lazy_static::lazy_static;
use regex::Regex;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

pub type Channelizer = (UnboundedSender<Message>, UnboundedReceiver<Message>);

// Valid bot command types and their parameters
#[derive(Debug)]
pub enum BotCommand {
    Spork(String),
    Sporklike(String),
    Bots,
}

// pub struct BotCommandMessage {
//     command: BotCommand,
//     message: Message,
//     place: String,
//     nick: String,
// }

// Convenient Result type. Commands always give back a String for output, or an error.
type BotrickResult = Result<String, Box<dyn std::error::Error>>;

// Take a string representing the text of an IRC PRIVMSG, and figure out if it has a command in it.
// Run the command if it exists
pub fn parse_command(text: &str) -> Option<BotCommand> {
    lazy_static! {
        static ref COMMAND_RE: Regex = Regex::new(r"^%(\S+)(\s*)").unwrap();
    }

    // Short-circuit command parsing for `.bots`
    // .bots is a special case that doesn't use our regular prefix
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

// Given a string, check whether it should be logged.
// Currently, CTCP messages other than ACTION are not logged.
pub fn should_log(msg: String) -> bool {
    !msg.starts_with('\u{001}') || msg.starts_with("\u{001}ACTION")
}

// Dispatch handlers for BotCommands
pub fn handle_command(cmd: BotCommand, place: &str, sender: Sender) -> BotrickResult {
    match cmd {
        BotCommand::Bots => handle_bots(place, sender),
        BotCommand::Spork(text) => handle_spork(text, place, sender),
        BotCommand::Sporklike(text) => handle_sporklike(text, place, sender),
    }
}

// Handle the `spork` command
pub fn handle_spork(text: String, _place: &str, _sender: Sender) -> BotrickResult {
    let db = sporker::getdb()?;
    let s = sporker::Spork::new(db);

    let words: Vec<&str> = text.split_whitespace().collect();
    let startw = if !words.is_empty() {
        s.start_with_word(words[0])
    } else {
        s.start()
    };

    match startw {
        Some(word) => {
            let words = sporker::build_words(word, &s);
            Ok(words.join(" "))
        }
        _ => Err("Couldn't do it could I".into()),
    }
}

// handle the `sporklike` commands
pub fn handle_sporklike(text: String, _place: &str, _sender: Sender) -> BotrickResult {
    let db = sporker::getdb()?;
    let s = sporker::Spork::new(db);

    // Get all our cmdline args
    let words: Vec<&str> = text.split_whitespace().collect();

    // Fewer than 2 args? Go away.
    if words.is_empty() {
        return Err("Talking about nobody is it".into());
    }

    let saidby = words[0];

    // If we have more than one arg, take the first one and run with it.
    // Otherwise, find out own start word. With blackjack. And hookers.
    let startw = match words.len() {
        1 => {
            s.start_like(saidby)
        }
        _ => {
            s.start_with_word_like(words[1], saidby)
        }
    };

    match startw {
        Some(word) => {
            let words = sporker::build_words_like(word, &s, saidby);
            Ok(words.join(" "))
        }
        _ => Err("Couldn't do it could I".into()),
    }
}

// Handle the `.bots` command
fn handle_bots(_place: &str, _sender: Sender) -> BotrickResult {
    // match sender.send_privmsg(place, "I AM A SEND") {
    //     Ok(_) => println!("Sent"),
    //     Err(e) => println!("{:?}", e)
    // }
    Ok("Reporting in! [Rust] just %spork or %sporklike, yo.".to_string())
}
