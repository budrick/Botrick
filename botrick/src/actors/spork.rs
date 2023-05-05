use tokio::sync::mpsc;

use crate::irc::CommandMessage;

struct SporkActor {
    receiver: mpsc::UnboundedReceiver<ActorMessage>,
    spork: sporker::Spork,
    _sender: irc::client::Sender,
}

#[derive(Debug)]
enum ActorMessage {
    Spork { message: CommandMessage },
    Sporklike { message: CommandMessage },
}

impl SporkActor {
    fn new(receiver: mpsc::UnboundedReceiver<ActorMessage>, sender: irc::client::Sender) -> Self {
        SporkActor {
            receiver,
            spork: sporker::Spork::new(sporker::getdb().unwrap()),
            _sender: sender,
        }
    }
    fn handle_message(&mut self, msg: ActorMessage) {
        tracing::debug!("Got message {:?}", msg);
        match msg {
            ActorMessage::Spork { message } => {
                tracing::debug!("Sporked {:?}", message);

                let words: Vec<&str> = message.params.split_whitespace().collect();
                let startw = if !words.is_empty() {
                    self.spork.start_with_word(words[0])
                } else {
                    self.spork.start()
                };

                let output: String = match startw {
                    Some(word) => {
                        let mut words = sporker::build_words(word, &self.spork);
                        words.insert(0, format!("{}:", &message.sent_by));
                        words.join(" ")
                    }
                    _ => String::from("Couldn't do it could I"),
                };

                let _ = self._sender.send_privmsg(&message.respond_to, output);
            }
            ActorMessage::Sporklike { message } => {
                tracing::debug!("Sporkliked {:?}", message);

                let words: Vec<&str> = message.params.split_whitespace().collect();

                if words.is_empty() {
                    let _ = self._sender.send_privmsg(&message.respond_to, "Talking about nobody is it");
                    return;
                }

                let startw = match words.len() {
                    1 => self.spork.start_like(words[0]),
                    _ => self.spork.start_with_word_like(words[1], words[0]),
                };

                let output: String = match startw {
                    Some(word) => {
                        let mut words = sporker::build_words_like(word, &self.spork, words[0]);
                        words.insert(0, format!("{}:", &message.sent_by));
                        words.join(" ")
                    }
                    _ => String::from("Couldn't do it could I"),
                };

                let _ = self._sender.send_privmsg(&message.respond_to, output);
            }
        };
    }
}

async fn run_my_actor(mut actor: SporkActor) {
    while let Some(msg) = actor.receiver.recv().await {
        actor.handle_message(msg);
    }
}

#[derive(Clone)]
pub struct SporkActorHandle {
    sender: mpsc::UnboundedSender<ActorMessage>,
}

impl SporkActorHandle {
    pub fn new(ircsender: irc::client::Sender) -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();
        let actor = SporkActor::new(receiver, ircsender);
        tokio::spawn(run_my_actor(actor));

        Self { sender }
    }
}

impl super::api::Actor for SporkActorHandle {
    fn process(&self, message: CommandMessage) {
        tracing::debug!("Spork Actor Handle received: {:?}", message);
        if &message.command == "7" {
            let _ = self.sender.send(ActorMessage::Spork {
                message: CommandMessage {
                    params: "7".to_string(),
                    ..message
                },
            });
            return;
        }
        let _ = match &message.command[1..] {
            "spork" => self.sender.send(ActorMessage::Spork { message }),
            "sporklike" => self.sender.send(ActorMessage::Sporklike { message }),
            _ => Ok(()),
        };
    }
}
