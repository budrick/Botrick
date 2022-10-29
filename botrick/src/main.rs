mod args;
mod bot;
mod channelizer;
mod color;
mod config;

use crate::{channelizer::Channelizer, config::Config as BotConfig};
use anyhow::Result;
use futures::prelude::*;
use irc::client::prelude::*;
use sporker::{getdb, Spork};
use std::fs;
use std::path::Path;
use tokio::sync::mpsc::unbounded_channel;

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command-line args, and set the working directory. Let it fail fast.
    let args = args::parse();
    let dir = fs::canonicalize(args.dir.unwrap())?;
    std::env::set_current_dir(dir)?;

    // Load configuration file or die trying
    let bot_config: BotConfig = confy::load_path(Path::new("botrick.toml"))?;
    // println!("{:?}", bot_config);
    // return Ok(());

    // Logger thread
    let (ltx, mut lrx): Channelizer = unbounded_channel();
    let _logger = tokio::spawn(async move {
        let db = getdb().unwrap();
        let s = Spork::new(db);

        while let Some(line) = lrx.recv().await {
            let nick = line.source_nickname();
            let cmd = line.command.clone();
            if let Command::PRIVMSG(_, text) = cmd {
                if !text.starts_with('\u{001}') || text.starts_with("\u{001}ACTION") {
                    if let Some(n) = nick {
                        s.log_message(n, text.as_str());
                    }
                }
            }
        }
    });

    // Spin up IRC loop
    let config = Config::load("irc.toml")?;
    let mut client = Client::from_config(config).await?;
    client.identify()?;

    let mut stream = client.stream()?;
    let sender = client.sender();

    while let Some(message) = stream.next().await.transpose()? {
        if let Command::PRIVMSG(ref _channel, ref _text) = message.command {
            // Left for demonstrative purposes: Quick and dirty example of listening for bot's own nick
            // if text.contains(&*client.current_nickname()) {
            //     // send_privmsg comes from ClientExt
            //     // sender.send_privmsg(&channel, "beep boop").unwrap();
            // }

            let cmd = bot::parse_command(&message);

            match cmd {
                Some(command) => {
                    if !command.command.eq("default") {
                        println!("{} {}", bot::mention_nick(&command.nick), command.command);
                    }
                    let sc = sender.clone();
                    let bcc = bot_config.clone();
                    if command.command.eq("default") {
                        ltx.send(message.clone())?; // Log the message if it isn't a valid command to us
                    }
                    tokio::task::spawn_blocking(move || {
                        _ = bot::handle_command_message(command, sc, bcc);
                    });
                    // match bot::handle_command_message(command, sender.clone()) {
                    //     _ => continue,
                    // }
                }
                _ => {
                    ltx.send(message.clone())?; // Log the message if it isn't a valid command to us
                }
            }
        }
    }

    Ok(())
}
