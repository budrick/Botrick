use irc::client::prelude::*;
use futures::prelude::*;
use regex::Regex;

extern crate botrick;
use botrick::sporker;

#[tokio::main]
async fn main() -> irc::error::Result<()> {

    let config = Config::load("config.toml")?;

    let mut client = Client::from_config(config).await?;
    client.identify()?;

    let mut stream = client.stream()?;

    let db = sporker::getdb();
    let s = sporker::Spork::new(db);

    let command_re = Regex::new(r"^%(\S+)(\s*)").unwrap();

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

            let cmd_text = text.strip_prefix(format!("%{}{}", cmd, spaces).as_str()).unwrap();

            match cmd {
                "spork" => {
                    let words: Vec<&str> = cmd_text.split_whitespace().collect();
                    let startw = if !words.is_empty() {
                        println!("{} sporked {:?}", target, words);
                        s.start_with_word(words[0])
                    } else {
                        println!("{} sporked no words", target);
                        s.start()
                    };

                    match startw {
                        Some(word) => {
                            let mut words = sporker::build_words(word, &s);
                            words.insert(0, target);
                            client.send_privmsg(&channel, words.join(" ")).unwrap();
                        }
                        _ => {
                            client.send_privmsg(&channel, "Couldn't do it could I").unwrap();
                        }
                    }
                }
                "sporklike" => {

                    // Get all our cmdline args
                    let words: Vec<&str> = cmd_text.split_whitespace().collect();

                    // Fewer than 2 args? Go away.
                    if words.len() < 1 {
                        client.send_privmsg(&channel, "Talking about nobody is it").unwrap();
                        continue;
                        // Ok(())
                    }

                    let saidby = words[0];

                    // If we have more than one arg, take the first one and run with it.
                    // Otherwise, find out own start word. With blackjack. And hookers.
                    let startw = match words.len() {
                        1 => {
                        println!("{} sporkliked {}", target, saidby);
                            s.start_like(saidby)
                        },
                        _ => {
                            println!("{} sporkliked {} {:?}", target, saidby, words);
                            s.start_with_word_like(words[1], saidby)
                        }
                    };

                    match startw {
                        Some(word) => {
                            let mut words = sporker::build_words_like(word, &s, saidby);
                            words.insert(0, target);
                            client.send_privmsg(&channel, words.join(" ")).unwrap();
                        }
                        _ => {
                            client.send_privmsg(&channel, "Couldn't do it could I").unwrap();
                        }
                    }
                },
                _ => continue
            }

        }
    }

    Ok(())
}
