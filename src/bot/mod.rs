use crate::sporker;
use anyhow::{anyhow, Context};
use irc::{client::Sender, proto::Command::PRIVMSG};
use lazy_static::lazy_static;
use regex::Regex;

pub fn parse_command(message: &irc::proto::Message) -> Option<CommandMessage> {
    if let PRIVMSG(ref _channel, ref text) = message.command {
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
                params: String::from(""),
            });
        }

        // Try and regex out a "normal" command. `cmd` becomes the command itself, `spaces` any following whitespace
        let maybe_cmd = COMMAND_RE.captures(text);
        let (cmd, spaces) = match maybe_cmd {
            Some(matches) => (
                matches.get(1).unwrap().as_str(),
                matches.get(2).unwrap().as_str(),
            ),
            // No (probable) commands? Make an attempt at getting links, and dispatch a special builtin
            _ => {
                ("default", "")    
            },
        };

        // Command text is the rest of the line minus the command + space prefix.
        // Default command gets special treatment - the whole privmsg is passed as
        // the params since there's no prefix to strip.
        let cmd_text = match cmd {
            "default" => text.as_str(),
            _ => {
                text
                    .strip_prefix(format!("%{}{}", cmd, spaces).as_str())
                    .map_or("", |v| v)
            }
        };

        // let cmd_text = text
        //     .strip_prefix(format!("%{}{}", cmd, spaces).as_str())
        //     .map_or("", |v| v);

        // If there is a valid command, create a CommandMessage to pass around to handlers.
        match cmd {
            "" => None,
            _ => Some(CommandMessage {
                nick: String::from(responsenick),
                channel: String::from(responsetarget),
                command: String::from(cmd),
                params: String::from(cmd_text),
            }),
        }
    } else {
        None
    }
}

pub fn mention_nick(nick: &str) -> String {
    format!("{}:", nick)
}

pub fn get_urls(message: &str)->Vec<linkify::Link> {
    let mut linkfinder = linkify::LinkFinder::new();
    linkfinder.kinds(&[linkify::LinkKind::Url]);
    linkfinder.links(message).collect()
}

pub async fn get_url_title(url: &str) -> Option<String> {
    if url.is_empty() {
        return None;
    }
    let response_o = reqwest::get(url).await;
    if response_o.is_err() {
        return None;
    }
    let response = response_o.unwrap();
    let content_type = response.headers().get("content-type");
    // if content_type.is_none() {
    //     return None;
    // }
    content_type?; // weeeeeeird
    if content_type.unwrap().to_str().unwrap_or_default().starts_with("text/html") {
        let body = response.text().await.unwrap_or_default();
        let doc = select::document::Document::from(body.as_str());
        let f:Vec<_> = doc.find(select::predicate::Name("title")).take(1).collect();
        if f.is_empty() {
            return None;
        }
        Some(f[0].text())
    } else {
        None
    }
}

fn get_command_handler(command: CommandMessage, sender: Sender) -> Option<Box<dyn Command>> {
    match command.command.as_str() {
        "default" => Some(Box::new(DefaultCommand { command, sender })),
        ".bots" => Some(Box::new(BotsCommand { command, sender })),
        "spork" => Some(Box::new(SporkCommand { command, sender })),
        "sporklike" => Some(Box::new(SporklikeCommand { command, sender })),
        _ => None,
    }
}

// Dispatch handlers for BotCommands
pub fn handle_command_message(command: CommandMessage, sender: Sender) -> CommandResult {
    let handler = get_command_handler(command.clone(), sender);
    if let Some(handler) = handler {
        handler.execute()
    } else {
        Err(anyhow!(
            "Could not find command handler for `{}`",
            command.command
        ))
    }
}

pub type CommandResult = anyhow::Result<()>;
// type CommandHandlerResult = anyhow::Result<Box<dyn Command>>;

pub trait Command {
    fn execute(&self) -> CommandResult;
}
#[derive(Debug, Clone)]
pub struct CommandMessage {
    pub command: String,
    pub params: String,
    pub channel: String,
    pub nick: String,
}

pub struct DefaultCommand {
    sender: Sender,
    command: CommandMessage,
}
impl Command for DefaultCommand {
    fn execute(&self) -> CommandResult {
        println!("{:#?}", self.command);
        let urls = get_urls(self.command.params.as_str());
        if urls.is_empty() {
            return Ok(());
        }

        self.sender
            .send_privmsg(
                &self.command.channel,
                format!("We found at least one URL: {}", urls[0].as_str()),
            )
            .with_context(|| "Failed to send message")
    }
}

pub struct BotsCommand {
    sender: Sender,
    command: CommandMessage,
}
impl Command for BotsCommand {
    fn execute(&self) -> CommandResult {
        self.sender
            .send_privmsg(
                &self.command.channel,
                String::from("Reporting in! [Rust] just %spork or %sporklike, yo."),
            )
            .with_context(|| "Failed to send message")
    }
}

pub struct SporkCommand {
    sender: Sender,
    command: CommandMessage,
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
    sender: Sender,
    command: CommandMessage,
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
