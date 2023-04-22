use tokio::sync::mpsc;

use crate::irc::CommandMessage;

struct TestActor {
    receiver: mpsc::UnboundedReceiver<ActorMessage>,
    sender: irc::client::Sender,
}

#[derive(Debug)]
enum ActorMessage {
    Test { message: CommandMessage },
}

impl TestActor {
    fn new(receiver: mpsc::UnboundedReceiver<ActorMessage>, sender: irc::client::Sender) -> Self {
        TestActor { receiver, sender }
    }
    fn handle_message(&mut self, msg: ActorMessage) {
        tracing::debug!("Got message {:?}", msg);
        match msg {
            ActorMessage::Test { message } => {
                tracing::debug!("Test is handling {:?}", message);
            }
        };
    }
}

async fn run_my_actor(mut actor: TestActor) {
    while let Some(msg) = actor.receiver.recv().await {
        actor.handle_message(msg);
    }
}

#[derive(Clone)]
pub struct TestActorHandle {
    sender: mpsc::UnboundedSender<ActorMessage>,
}

impl TestActorHandle {
    pub fn new(ircsender: irc::client::Sender) -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();
        let actor = TestActor::new(receiver, ircsender);
        tokio::spawn(run_my_actor(actor));

        Self { sender }
    }
}

impl super::api::Actor for TestActorHandle {
    fn process(&self, message: CommandMessage) {
        tracing::debug!("Test Actor Handle received: {:?}", message);
        let _ = self.sender.send(ActorMessage::Test { message });
    }
}
