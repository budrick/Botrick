use tokio::sync::mpsc;

use crate::bot::CommandMessage;
// use crate::color::{colorize, Color};

struct IrcActor {
    receiver: mpsc::UnboundedReceiver<ActorMessage>,
    sender: irc::client::Sender,
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

impl IrcActor {
    fn new(receiver: mpsc::UnboundedReceiver<ActorMessage>, sender: irc::client::Sender) -> Self {
        let game = werdle::Game::new();
        IrcActor {
            receiver,
            sender,
        }
    }
    fn handle_message(&mut self, msg: ActorMessage) {
        println!("Got message: {:?}", msg);
    }
}

async fn run_my_actor(mut actor: IrcActor) {
    while let Some(msg) = actor.receiver.recv().await {
        actor.handle_message(msg);
    }
}

#[derive(Clone)]
pub struct IrcActorHandle {
    sender: mpsc::UnboundedSender<ActorMessage>,
}

impl IrcActorHandle {
    pub fn new(ircsender: irc::client::Sender) -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();
        let actor = IrcActor::new(receiver, ircsender);
        tokio::spawn(run_my_actor(actor));

        Self { sender }
    }

    pub fn process(&self, message: irc::proto::Message) {
        tracing::debug!("Received: {}", message);
    }
}
