use crate::{color::colorize, irc::CommandMessage};
use rand::prelude::*;
use tokio::sync::mpsc;
use crate::data::{DWORD_ROOTS, DWORDS};

struct MiscActor {
    receiver: mpsc::UnboundedReceiver<ActorMessage>,
    _sender: irc::client::Sender,
}

#[derive(Debug)]
enum ActorMessage {
    Isit { message: CommandMessage },
    Dword { message: CommandMessage },
}

impl MiscActor {
    fn new(receiver: mpsc::UnboundedReceiver<ActorMessage>, sender: irc::client::Sender) -> Self {
        MiscActor {
            receiver,
            _sender: sender,
        }
    }
    fn handle_message(&mut self, msg: ActorMessage) {
        tracing::debug!("Got message {:?}", msg);
        match msg {
            ActorMessage::Isit { message } => {
                tracing::debug!("Isit");

                let isit: bool = random();
                let output = if isit {
                    colorize(crate::color::Color::Red, None, "It is")
                } else {
                    colorize(crate::color::Color::Red, None, "Just isn't")
                };

                let _ = self._sender.send_privmsg(message.respond_to, output);
            },
            ActorMessage::Dword { message } => {
                tracing::debug!("Dword");

                let mut rng = rand::thread_rng();
                let num_elements = rng.gen_range(1..25);

                let root = DWORD_ROOTS[rand::thread_rng().gen_range(0..DWORD_ROOTS.len())];
                let mut root = root.to_owned();
                for _ in 1..num_elements {
                    root.push('\\');
                    root.push_str(DWORDS[rand::thread_rng().gen_range(0..DWORDS.len())]);
                }

                let _ = self._sender.send_privmsg(message.respond_to, root);
            },
        };
    }
}

async fn run_my_actor(mut actor: MiscActor) {
    while let Some(msg) = actor.receiver.recv().await {
        actor.handle_message(msg);
    }
}

#[derive(Clone)]
pub struct MiscActorHandle {
    sender: mpsc::UnboundedSender<ActorMessage>,
}

impl MiscActorHandle {
    pub fn new(ircsender: irc::client::Sender) -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();
        let actor = MiscActor::new(receiver, ircsender);
        tokio::spawn(run_my_actor(actor));

        Self { sender }
    }
}

impl super::api::Actor for MiscActorHandle {
    fn process(&self, message: CommandMessage) {
        tracing::debug!("Misc Actor Handle received: {:?}", message);
        let _ = match &message.command[1..] {
            "isit" => self.sender.send(ActorMessage::Isit { message }),
            "dword" => self.sender.send(ActorMessage::Dword { message }),
            _ => Ok(()),
        };
    }
}
