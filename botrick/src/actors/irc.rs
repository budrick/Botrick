use std::collections::HashMap;

use tokio::sync::mpsc;

use crate::irc::CommandMessage;

// use crate::bot::CommandMessage;

// use crate::bot::CommandMessage;
// use crate::color::{colorize, Color};

struct IrcActor {
    receiver: mpsc::UnboundedReceiver<ActorMessage>,
    sender: irc::client::Sender,
    handlers: HashMap<String, Box<dyn super::Actor>>,
}

#[derive(Debug)]
enum ActorMessage {
    Register {
        command: String,
        handler: Box<dyn super::Actor>,
    },
    Process {
        message: Box<CommandMessage>,
    },
}

impl IrcActor {
    fn new(receiver: mpsc::UnboundedReceiver<ActorMessage>, sender: irc::client::Sender) -> Self {
        IrcActor {
            receiver,
            sender,
            handlers: HashMap::new(),
        }
    }
    fn handle_message(&mut self, msg: ActorMessage) {
        tracing::debug!("Got message: {:?}", msg);
        match msg {
            ActorMessage::Register { command, handler } => {
                self.handlers.insert(command, handler);
            }
            ActorMessage::Process { message } => {
                // TODO: Log anything that doesn't match a command.
                if let Some(h) = self.handlers.get(&message.command) {
                    tracing::debug!("Passing to handler: {:?}", message);
                    h.process(*message);
                }
            }
        }
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
        let _ = self.sender.send(ActorMessage::Process {
            message: Box::new(message.into()),
        });
    }

    pub fn register(&self, command: String, handler: Box<dyn super::Actor>) {
        let _ = self
            .sender
            .send(ActorMessage::Register { command, handler });
    }
}
