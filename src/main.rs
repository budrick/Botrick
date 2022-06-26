use anyhow::Result;
use futures::prelude::*;
use irc::client::prelude::*;
use tokio::sync::mpsc::unbounded_channel;

use botrick::{handle_command, parse_command, sporker, Channelizer};

#[tokio::main]
async fn main() -> Result<()> {
    // Logger task
    let (ltx, mut lrx): Channelizer = unbounded_channel();
    let _logger = tokio::spawn(async move {
        let db = sporker::getdb().unwrap();
        let s = sporker::Spork::new(db);

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
    let mut client = Client::from_config(Config::load("config.toml")?).await?;
    client.identify()?;

    let mut stream = client.stream()?;
    let sender = client.sender();

    while let Some(message) = stream.next().await.transpose()? {
        if let Command::PRIVMSG(ref _channel, ref text) = message.command {
            // Left for demonstrative purposes: Quick and dirty example of listening for bot's own nick
            // if text.contains(&*client.current_nickname()) {
            //     // send_privmsg comes from ClientExt
            //     // sender.send_privmsg(&channel, "beep boop").unwrap();
            // }

            // Determine where to send responses, and who to ping if applicable.
            let responseplace = message.response_target().unwrap();
            let responsenick = match message.source_nickname() {
                Some(nick) => {
                    format!("{}: ", nick)
                }
                _ => "".to_string(),
            }
            .to_string();

            // Attempt to parse out a valid command from the line.
            // If it's a valid bot command, call a handler and send back the response.
            // If it's just a regular PRIVMSG, send it to the logger task.
            let cmd = parse_command(text);
            match cmd {
                Some(command) => {
                    println!("{}{:?}", responsenick, command);
                    let result = match handle_command(command) {
                        Ok(message) => {
                            format!("{}{}", responsenick, message)
                        }
                        Err(message) => {
                            message.to_string()
                        }
                    };

                    sender.send_privmsg(responseplace, result)?;
                }
                _ => {
                    ltx.send(message.clone())?; // Log the message if it isn't a valid command to us
                }
            }
        }
    }

    Ok(())
}
