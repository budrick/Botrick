/*
So let's sketch out how this should go:
    - Most services / commands should be handled by actors.
        - We get lots of concurrency
        - We get isolation
        - Database can be shared AND safely accessed across tasks/threads - because it's in its own one.
    - IRC loop / actor is the controlling factor.
        - We can probably simply spin this up and feed an Actor its messages.
        - Said Actor will handle message parsing, handoff to the Logging actor, etc.
 */

use color_eyre::eyre::Result;
use futures::StreamExt;
use tracing::debug;
use botrick::actors::IrcActorHandle;
use irc::client::prelude as irc;
use botrick::config as botconfig;

#[tokio::main]
async fn main() -> Result<()> {
    // Error formatting, good tracing
    color_eyre::install()?;
    tracing_subscriber::fmt::init();

    // Parse command-line args, and set the working directory. Let it fail fast.
    let args = botrick::args::parse();
    let dir = std::fs::canonicalize(args.dir.unwrap())?;
    std::env::set_current_dir(dir)?;
    
    // Load configuration file or die trying
    // let bot_config: botconfig::Config = confy::load_path(std::path::Path::new("botrick.toml"))?;
    
    
    // Spin up IRC loop
    let user_config = irc::Config::load("irc.toml")?;
    let config = irc::Config {
        version: Some(botrick::VERSION_STR.to_string()),
        ..user_config
    };
    let mut client = irc::Client::from_config(config).await?;
    client.identify()?;
    let mut stream = client.stream()?;
    let sender = client.sender();
    
    let irc_handler = IrcActorHandle::new(sender.clone());

    while let Some(message) = stream.next().await.transpose()? {
        if let irc::Command::PRIVMSG(ref _channel, ref text) = message.command {

            // Reject CTCP - handled by the `irc` crate on its own
            if text.starts_with('\u{1}') {
                continue;
            }
            irc_handler.process(message);
        }
    }



    Ok(())
}