use irc::client::prelude::*;
use futures::prelude::*;

#[tokio::main]
async fn main() -> irc::error::Result<()> {

    let config = Config::load("config.toml")?;

    let mut client = Client::from_config(config).await?;
    client.identify()?;

    let mut stream = client.stream()?;

    while let Some(message) = stream.next().await.transpose()? {
        if let Command::PRIVMSG(channel, message) = message.command {
            if message.contains(&*client.current_nickname()) {
                // send_privmsg comes from ClientExt
                client.send_privmsg(&channel, "beep boop").unwrap();
            }
        }
    }

    Ok(())
}
