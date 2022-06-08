use irc::client::prelude::*;
use futures::prelude::*;
use regex::Regex;
use std::collections::VecDeque;

mod sporker;

#[tokio::main]
async fn main() -> irc::error::Result<()> {

    let config = Config::load("config.toml")?;

    let mut client = Client::from_config(config).await?;
    client.identify()?;

    let mut stream = client.stream()?;

    let db = sporker::getdb();
    let s = sporker::Spork::new(db);

    let command_re = Regex::new(r"^:(\S+)(\s*)").unwrap();

    while let Some(message) = stream.next().await.transpose()? {
        let msgtarget = message.source_nickname();
        let target = match msgtarget {
            Some(nick) => {
                format!("{}:", nick)
            }
            _ => "".to_string()
        }.to_string();
        if let Command::PRIVMSG(ref channel, ref text) = message.command {
            // if text.contains(&*client.current_nickname()) {
            //     // send_privmsg comes from ClientExt
            //     // client.send_privmsg(&channel, "beep boop").unwrap();
            // }

            let maybe_cmd = command_re.captures(text);
            let (cmd, spaces): (&str, &str) = match maybe_cmd {
                Some(matches) => {
                    (matches.get(1).unwrap().as_str(), matches.get(2).unwrap().as_str())
                }
                _ => continue
            };

            let cmd_text = text.strip_prefix(format!(":{}{}", cmd, spaces).as_str()).unwrap();

            match cmd {
                "spork" => {
                    let words: Vec<&str> = cmd_text.split_whitespace().collect();
                    let startw = if !words.is_empty() {
                        println!("{} sporked {:?}", target, words);
                        s.start_with_word(words[0], None)
                    } else {
                        println!("{} sporked no words", target);
                        s.start(None)
                    };

                    match startw {
                        Some(word) => {
                            let mut words = sporker::build_words(word, &s, None);
                            words.insert(0, target);
                            client.send_privmsg(&channel, words.join(" ")).unwrap();
                        }
                        _ => {
                            client.send_privmsg(&channel, "Couldn't do it could I").unwrap();
                        }
                    }
                },
                "sporklike" => {
                    let mut words: VecDeque<&str> = cmd_text.split_whitespace().collect();
                    if words.is_empty() {
                        client.send_privmsg(&channel, "Talking about nobody is it").unwrap();
                    }
                    let saidby = words.pop_front();
                    let startw = if !words.is_empty() {
                        println!("{} sporked {:?}", target, words);
                        s.start_with_word(words[0], saidby)
                    } else {
                        println!("{} sporked no words", target);
                        s.start(saidby)
                    };

                    match startw {
                        Some(word) => {
                            let mut words = sporker::build_words(word, &s, saidby);
                            words.insert(0, target);
                            client.send_privmsg(&channel, words.join(" ")).unwrap();
                        }
                        _ => {
                            client.send_privmsg(&channel, "Couldn't do it could I").unwrap();
                        }
                    }
                }
                _ => continue
            }

        }
    }

    Ok(())
}
