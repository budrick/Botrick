use tokio::sync::mpsc;
use werdle::{self, GuessResult};

use crate::color::{Color, colorize};
use crate::irc::CommandMessage;

struct WerdleActor {
    receiver: mpsc::UnboundedReceiver<ActorMessage>,
    sender: irc::client::Sender,
    game: werdle::Game,
}

#[derive(Debug)]
enum ActorMessage {
    Guess {
        // word: String,
        command: CommandMessage,
    },
    GetState {
        command: CommandMessage,
    },
    // GetWord {
    //     command: crate::bot::CommandMessage,
    // },
}

impl WerdleActor {
    fn new(receiver: mpsc::UnboundedReceiver<ActorMessage>, sender: irc::client::Sender) -> Self {
        let game = werdle::Game::new();
        WerdleActor {
            receiver,
            sender,
            game,
        }
    }
    fn handle_message(&mut self, msg: ActorMessage) {
        println!("Got message: {:?}", msg);
        match msg {
            ActorMessage::Guess { command } => {
                println!("Guessed {}", command.params);
                let res = self.game.guess(command.params.as_str());
                match res {
                    Ok(result) => {
                        if self.game.is_correct() {
                            let _ = self.sender.send_privmsg(
                                command.respond_to,
                                format!("You did it. It was {}. Good job Team.", self.game.werd()),
                            );
                            self.game = werdle::Game::new();
                        } else if self.game.is_finished() {
                            let _ = self.sender.send_privmsg(command.respond_to, format!("Sorry, nobody got it. Better luck next time or something. It was {} btw.", self.game.werd()));
                        } else {
                            let _ = self.sender.send_privmsg(
                                command.respond_to,
                                format!(
                                    "NO, try again. {}, remaining letters: {}. {} tries left.",
                                    color_result(result),
                                    self.game.unguessed_letters(),
                                    self.game.guesses_left()
                                ),
                            );
                        }
                    }
                    Err(_) => {
                        let _ = self
                            .sender
                            .send_privmsg(command.respond_to, "Guess correctly pls");
                    }
                }

                if self.game.is_finished() {
                    self.game = werdle::Game::new()
                }
                // The `let _ =` ignores any errors when sending.
                //
                // This can happen if the `select!` macro is used
                // to cancel waiting for the response.
                // let _ = respond_to.send(self.next_id);
            }
            ActorMessage::GetState { command } => {
                let last_guess = self.game.last_guess();
                let letters = self.game.unguessed_letters();
                let guess = match last_guess {
                    Some(g) => color_result(g),
                    None => String::from("_____"),
                };
                let message = format!(
                    "{}, remaining letters: {}. {} tries left.",
                    guess,
                    letters,
                    self.game.guesses_left()
                );
                let _ = self.sender.send_privmsg(command.respond_to, message);
            }
        }
    }
}

async fn run_my_actor(mut actor: WerdleActor) {
    while let Some(msg) = actor.receiver.recv().await {
        actor.handle_message(msg);
    }
}

fn color_result(state: GuessResult) -> String {
    let mut colored_result = String::new();
    for guess_result in state.result {
        let output = match guess_result.1 {
            werdle::GuessCharState::WrongChar => String::from("_"),
            werdle::GuessCharState::WrongPlace => {
                colorize(Color::Yellow, None, String::from(guess_result.0).as_str())
            }
            werdle::GuessCharState::RightChar => {
                colorize(Color::Green, None, String::from(guess_result.0).as_str())
            }
        };

        colored_result.push_str(output.as_str());
    }
    colored_result
}

#[derive(Clone)]
pub struct WerdleActorHandle {
    sender: mpsc::UnboundedSender<ActorMessage>,
}

impl WerdleActorHandle {
    pub fn new(ircsender: irc::client::Sender) -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();
        let actor = WerdleActor::new(receiver, ircsender);
        tokio::spawn(run_my_actor(actor));

        Self { sender }
    }

    // pub fn get_word(&self, command: CommandMessage) {
    //     println!("Get word");
    //     let _ = self.sender.send(ActorMessage::GetWord { command });
    // }
}

impl super::api::Actor for WerdleActorHandle {
    fn process(&self, message: CommandMessage) {
        tracing::debug!("Werdle Actor Handle received: {:?}", message);
        let wm = if message.params.trim().is_empty() {
            ActorMessage::GetState { command: message }
        } else {
            ActorMessage::Guess { command: message }
        };
        let _ = self.sender.send(wm);
    }
}
