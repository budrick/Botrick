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
                let startw = self.spork.start();
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
            ActorMessage::Sporklike { message } => todo!(),
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
        let _ = match &message.command[1..] {
            "spork" => self.sender.send(ActorMessage::Spork { message }),
            "sporklike" => self.sender.send(ActorMessage::Spork { message }),
            _ => return,
        };
    }
}
