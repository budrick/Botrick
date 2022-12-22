use anyhow::Context;
use irc::client::Sender;
use tokio::{
    spawn,
    sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
};
use werdle::*;

#[derive(Debug)]
pub struct WerdleMessage {
    pub guess: String,
    pub ircsender: Sender,
    pub irctarget: String,
}
pub type WerdleSender = UnboundedSender<WerdleMessage>;
pub type WerdleChannelizer = (WerdleSender, UnboundedReceiver<WerdleMessage>);

pub fn playwerdle() -> WerdleSender {
    let (wtx, mut wrx): WerdleChannelizer = unbounded_channel();
    let mut game = Game::new();
    let _ = spawn(async move {
        while let Some(guess) = wrx.recv().await {
            println!("{:#?}", game);
            println!("Guess: {:#?}", guess);
            if !guess.guess.is_empty() {
                let result = game.guess(&guess.guess);
                println!("{:#?}", result);
            }
            if game.is_finished() {
                println!("Finished!");
                let _ = guess
                    .ircsender
                    .send_privmsg(
                        guess.irctarget,
                        format!("Done! Word was {}", game.werd()).as_str(),
                    )
                    .with_context(|| "Failed to send message");
                game = Game::new();
            }
        }
    });
    wtx
}
