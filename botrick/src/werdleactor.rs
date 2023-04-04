use tokio::sync::mpsc;
use werdle::{self, GuessResult};

use crate::bot::CommandMessage;
use crate::color::{colorize, Color};

struct WerdleActor {
    receiver: mpsc::UnboundedReceiver<ActorMessage>,
    sender: irc::client::Sender,
    game: werdle::Game,
}

#[derive(Debug)]
enum ActorMessage {
    Guess {
        word: String,
        command: crate::bot::CommandMessage,
    },
    GetState {
        command: crate::bot::CommandMessage,
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
            ActorMessage::Guess { word, command } => {
                println!("Guessed {}", word);
                let res = self.game.guess(word.as_str());
                match res {
                    Ok(result) => {
                        if self.game.is_correct() {
                            let _ = self.sender.send_privmsg(
                                command.channel,
                                format!("You did it. It was {}. Good job Team.", self.game.werd()),
                            );
                            self.game = werdle::Game::new();
                        } else if self.game.is_finished() {
                            let _ = self.sender.send_privmsg(command.channel, format!("Sorry, nobody got it. Better luck next time or something. It was {} btw.", self.game.werd()));
                        } else {
                            let _ = self.sender.send_privmsg(
                                command.channel,
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
                            .send_privmsg(command.channel, "Guess correctly pls");
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
                let _ = self.sender.send_privmsg(command.channel, message);
            }
            // ActorMessage::GetWord { command } => {
            //     println!("GetWord called");
            //     let _ = self.sender.send_privmsg(command.channel, self.game.werd());
            // },
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

    pub fn guess(&self, guess: String, command: CommandMessage) {
        let msg = ActorMessage::Guess {
            word: guess,
            command,
        };

        // Ignore send errors. If this send fails, so does the
        // recv.await below. There's no reason to check the
        // failure twice.
        let _ = self.sender.send(msg);
    }

    pub fn get_state(&self, command: CommandMessage) {
        let _ = self.sender.send(ActorMessage::GetState { command });
    }

    // pub fn get_word(&self, command: CommandMessage) {
    //     println!("Get word");
    //     let _ = self.sender.send(ActorMessage::GetWord { command });
    // }
}
