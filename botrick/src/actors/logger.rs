use sporker::{getdb, Spork};
use tokio::sync::mpsc;

#[derive(Debug)]
struct LogActor {
    receiver: mpsc::UnboundedReceiver<LogActorMessage>,
    spork: Spork,
}
#[derive(Debug)]
enum LogActorMessage {
    Log { message: crate::irc::CommandMessage },
}

impl LogActor {
    fn new(receiver: mpsc::UnboundedReceiver<LogActorMessage>) -> Self {
        LogActor {
            receiver,
            spork: Spork::new(getdb().unwrap()),
        }
    }
    fn handle_message(&mut self, msg: LogActorMessage) {
        match msg {
            LogActorMessage::Log { message } => {
                tracing::debug!("Logger got {:?}", message);
                let nick = message.sent_by;
                if !message.full_text.starts_with('\u{001}') || message.full_text.starts_with("\u{001}ACTION") {
                    self.spork.log_message(&nick, &message.full_text);
                }
            }
        }
    }
}

async fn run_my_actor(mut actor: LogActor) {
    while let Some(msg) = actor.receiver.recv().await {
        actor.handle_message(msg);
    }
}

#[derive(Clone, Debug)]
pub struct LogActorHandle {
    sender: mpsc::UnboundedSender<LogActorMessage>,
}

impl LogActorHandle {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();
        let actor = LogActor::new(receiver);
        tokio::spawn(run_my_actor(actor));

        Self { sender }
    }

    pub fn log(&self, message: crate::irc::CommandMessage) {
        let msg = LogActorMessage::Log { message };

        let _ = self.sender.send(msg);
    }
}

impl Default for LogActorHandle {
    fn default() -> Self {
        Self::new()
    }
}
