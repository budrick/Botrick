use anyhow::Result;
use futures::prelude::*;
use irc::client::prelude::*;
use tokio::sync::mpsc::unbounded_channel;

use botrick::{parse_command, sporker, BotCommand, Channelizer};

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load("config.toml")?;
    // println!("{:?}", config);

    // Logger thread
    let (ltx, mut lrx): Channelizer = unbounded_channel();
    let _logger = tokio::spawn(async move {
        let db = sporker::getdb().unwrap();
        let s = sporker::Spork::new(db);

        while let Some(line) = lrx.recv().await {
            let nick = line.source_nickname();
            let cmd = line.command.clone();
            if let Command::PRIVMSG(_, text) = cmd {
                if !text.starts_with('\u{001}') || text.starts_with("\u{001}ACTION") {
                    // println!("Line: {:?}, {:?}", nick, text);
                    if let Some(n) = nick {
                        s.log_message(n, text.as_str());
                    }
                }
            }
        }
    });

    // Main thread stuff resumes here
    let db = sporker::getdb()?;
    let s = sporker::Spork::new(db);

    let mut client = Client::from_config(config).await?;
    client.identify()?;

    let mut stream = client.stream()?;
    let sender = client.sender();

    while let Some(message) = stream.next().await.transpose()? {
        if let Command::PRIVMSG(ref _channel, ref text) = message.command {
            // if text.contains(&*client.current_nickname()) {
            //     // send_privmsg comes from ClientExt
            //     // sender.send_privmsg(&channel, "beep boop").unwrap();
            // }
            let responseplace = message.response_target().unwrap();
            let responsenick = match message.source_nickname() {
                Some(nick) => {
                    format!("{}:", nick)
                }
                _ => "".to_string(),
            }
            .to_string();

            // println!("source_nickname: {:?} response_target {:?} message {:?}", message.source_nickname(), message.response_target(), message);

            let cmd = parse_command(text);
            match cmd {
                Some(BotCommand::Bots) => {
                    sender.send_privmsg(
                        responseplace,
                        "Reporting in! [Rust] just %spork or %sporklike, yo.",
                    )?;
                }
                Some(BotCommand::Spork(cmd_text)) => {
                    let words: Vec<&str> = cmd_text.split_whitespace().collect();
                    let startw = if !words.is_empty() {
                        println!("{} sporked {:?}", responsenick, words);
                        s.start_with_word(words[0])
                    } else {
                        println!("{} sporked no words", responsenick);
                        s.start()
                    };

                    match startw {
                        Some(word) => {
                            let mut words = sporker::build_words(word, &s);
                            words.insert(0, responsenick.to_string());
                            sender.send_privmsg(responseplace, words.join(" "))?;
                        }
                        _ => {
                            sender.send_privmsg(responseplace, "Couldn't do it could I")?;
                        }
                    }
                }
                Some(BotCommand::Sporklike(cmd_text)) => {
                    // Get all our cmdline args
                    let words: Vec<&str> = cmd_text.split_whitespace().collect();

                    // Fewer than 2 args? Go away.
                    if words.is_empty() {
                        sender
                            .send_privmsg(responseplace, "Talking about nobody is it")
                            .unwrap();
                        continue;
                        // Ok(())
                    }

                    let saidby = words[0];

                    // If we have more than one arg, take the first one and run with it.
                    // Otherwise, find out own start word. With blackjack. And hookers.
                    let startw = match words.len() {
                        1 => {
                            println!("{} sporkliked {}", responsenick, saidby);
                            s.start_like(saidby)
                        }
                        _ => {
                            println!("{} sporkliked {} {:?}", responsenick, saidby, words);
                            s.start_with_word_like(words[1], saidby)
                        }
                    };

                    match startw {
                        Some(word) => {
                            let mut words = sporker::build_words_like(word, &s, saidby);
                            words.insert(0, responsenick.to_string());
                            sender.send_privmsg(responseplace, words.join(" ")).unwrap();
                        }
                        _ => {
                            sender
                                .send_privmsg(responseplace, "Couldn't do it could I")
                                .unwrap();
                        }
                    }
                }
                _ => {
                    ltx.send(message.clone())?; // Log the message if it isn't a valid command to us
                }
            }
        }
    }

    Ok(())
}
