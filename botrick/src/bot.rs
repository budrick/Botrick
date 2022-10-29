use crate::color::{colorize, Color};
use crate::config::Config;
use anyhow::{anyhow, Context};
use command::{create_bot_command, bot_commands};
use commands::*;
use irc::{client::Sender, proto::Command::PRIVMSG};
use lazy_static::lazy_static;
use regex::Regex;
mod command;
mod commands;

pub fn parse_command(message: &irc::proto::Message) -> Option<CommandMessage> {
    if let PRIVMSG(ref _channel, ref text) = message.command {
        lazy_static! {
            static ref COMMAND_RE: Regex = Regex::new(r"^%(\S+)(\s*)").unwrap();
        }

        // CTCP messages are ignored.
        if text.starts_with('\u{001}') {
            return None;
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
            _ => ("default", ""),
        };

        // Command text is the rest of the line minus the command + space prefix.
        // Default command gets special treatment - the whole privmsg is passed as
        // the params since there's no prefix to strip.
        let cmd_text = match cmd {
            "default" => text.as_str(),
            _ => text
                .strip_prefix(format!("%{}{}", cmd, spaces).as_str())
                .map_or("", |v| v),
        };

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

pub fn get_urls(message: &str) -> Vec<linkify::Link> {
    let mut linkfinder = linkify::LinkFinder::new();
    linkfinder.kinds(&[linkify::LinkKind::Url]);
    linkfinder.links(message).collect()
}

pub fn get_url_title(url: &str) -> Option<String> {
    if url.is_empty() {
        return None;
    }
    let response_o = reqwest::blocking::get(url);
    if response_o.is_err() {
        return None;
    }
    let response = response_o.unwrap();
    let content_type = response.headers().get("content-type");
    // if content_type.is_none() {
    //     return None;
    // }
    content_type?; // weeeeeeird
    if content_type
        .unwrap()
        .to_str()
        .unwrap_or_default()
        .starts_with("text/html")
    {
        let body = response.text().unwrap_or_default();
        let doc = select::document::Document::from(body.as_str());
        let f: Vec<_> = doc.find(select::predicate::Name("title")).take(1).collect();
        if f.is_empty() {
            return None;
        }
        Some(f[0].text())
    } else {
        None
    }
}

fn get_command_handler(
    command: CommandMessage,
    sender: Sender,
    config: Config,
) -> Option<Box<dyn Command>> {
    let params = CommandParams {command, sender, config };
    bot_commands!(
        params.command.command.as_str(),
        params,
        [
            "test" => TestCommand { params },
            ".bots" => BotsCommand { params },
            "spork" => crate::bot::commands::SporkTestCommand { params},
        ]
    )
    // match command.command.as_str() {
    //     "default" => Some(Box::new(DefaultCommand {
    //         command,
    //         sender,
    //         config,
    //     })),
    //     ".bots" => Some(Box::new(BotsCommand { command, sender })),
    //     "spork" => Some(Box::new(SporkCommand { command, sender })),
    //     "sporklike" => Some(Box::new(SporklikeCommand { command, sender })),
    //     "colors" => Some(Box::new(ColorsCommand { command, sender })),
    //     "sleep" => Some(Box::new(SleepCommand { command, sender })),
    //     "test" => Some(Box::new(TestCommand {
    //         command,
    //         sender,
    //         config,
    //     })),
    //     _ => None,
    // }
}

// This is some weird voodoo.
create_bot_command![
    TestCommand,
    {
        self.params.sender
            .send_privmsg(&self.params.command.channel, "testtest")
            .with_context(|| "Couldn't send message")
    },
    self
];

// This is some weird voodoo.
create_bot_command![
    BotsCommand,
    {
        self.params.sender
            .send_privmsg(
                &self.params.command.channel,
                String::from("Reporting in! [Rust] just %spork or %sporklike, yo."),
            )
            .with_context(|| "Failed to send message")
    },
    self
];

// Dispatch handlers for BotCommands
pub fn handle_command_message(
    command: CommandMessage,
    sender: Sender,
    config: Config,
) -> CommandResult {
    let handler = get_command_handler(command.clone(), sender, config);
    if let Some(handler) = handler {
        handler.execute()
    } else {
        Err(anyhow!(
            "Could not find command handler for `{}`",
            command.command
        ))
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct CommandParams {
    command: CommandMessage,
    sender: Sender,
    config: Config,
}

pub struct DefaultCommand {
    sender: Sender,
    command: CommandMessage,
    config: Config,
}
impl Command for DefaultCommand {
    fn execute(&self) -> CommandResult {
        let urls = get_urls(self.command.params.as_str());
        if !self.config.inspect_urls || urls.is_empty() {
            return Ok(());
        }
        let title = get_url_title(urls[0].as_str());
        if title.is_none() {
            return Ok(());
        }
        let colbit = colorize(Color::Green, None, "LINK >>");
        self.sender
            .send_privmsg(
                &self.command.channel,
                format!("{} {}", colbit, title.unwrap()),
            )
            .with_context(|| "Failed to send message")
    }
}
