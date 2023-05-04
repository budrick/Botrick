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

use std::sync::Arc;

use botrick::{actors::{
    DefaultActorHandle, IrcActorHandle, SporkActorHandle, TestActorHandle, WerdleActorHandle,
}, bot, config::Config};
// use botrick::config as botconfig;
use color_eyre::eyre::Result;
use futures::StreamExt;
use irc::client::prelude as irc;
// use tracing_subscriber::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Error formatting, good tracing
    color_eyre::install()?;
    tracing_subscriber::fmt::init();

    // When we feel the need to use tokio-console - start here and comment the tracing_subscriber line above.
    // and the prelude import too.

    // let console_layer = console_subscriber::spawn();
    // tracing_subscriber::registry()
    //     .with(console_layer)
    //     .with(tracing_subscriber::fmt::layer())
    //     .init();

    // Parse command-line args, and set the working directory. Let it fail fast.
    let args = botrick::args::parse();
    let dir = std::fs::canonicalize(args.dir.unwrap())?;
    std::env::set_current_dir(dir)?;

    // Load configuration file or die trying
    let bot_config: Config = confy::load_path(std::path::Path::new("botrick.toml"))?;

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

    let default_handler = Arc::new(DefaultActorHandle::new(sender.clone(), Arc::new(bot_config)));

    let irc_handler = IrcActorHandle::new(
        sender.clone(),
        default_handler.clone(),
    );

    let werdle_handler = Arc::new(WerdleActorHandle::new(sender.clone()));
    irc_handler.register_prefixed('%', ["wordle", "werdle"], werdle_handler);

    let spork_handler = Arc::new(SporkActorHandle::new(sender.clone()));
    irc_handler.register_prefixed('%', ["spork", "sporklike"], spork_handler.clone());
    irc_handler.register_regex([r"^7$"], spork_handler.clone(), None);
    irc_handler.register_regex([r"^\.bots\b"], default_handler.clone(), None);

    irc_handler.refresh_regexes();

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
