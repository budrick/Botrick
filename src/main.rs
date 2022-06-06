use irc::client::prelude::*;
use futures::prelude::*;

mod sporker;

fn some_words(word: Option<String>) {

}

#[tokio::main]
async fn main() -> irc::error::Result<()> {

    let config = Config::load("config.toml")?;

    let mut client = Client::from_config(config).await?;
    client.identify()?;

    let mut stream = client.stream()?;

    let db = sporker::getdb();
    let s = sporker::Spork::new(db);

    while let Some(message) = stream.next().await.transpose()? {
        let msgtarget = message.source_nickname();
        let target = match msgtarget {
            Some(nick) => {
                format!("{}:", nick.to_string())
            }
            _ => "".to_string()
        }.to_string();
        if let Command::PRIVMSG(ref channel, ref text) = message.command {
            if text.contains(&*client.current_nickname()) {
                // send_privmsg comes from ClientExt
                // client.send_privmsg(&channel, "beep boop").unwrap();
            }
            if text.starts_with(":spork ") {
                let words: Vec<&str> = text.strip_prefix(":spork ").unwrap().split_whitespace().collect();
                let startw;
                if words.len() > 0 {
                    println!("{} sporked {:?}", target, words);
                    startw = s.start_with_word(words[0]);
                } else {
                    println!("{} sporked no words", target);
                    startw = s.start()
                }

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
        }
    }

    Ok(())
}
