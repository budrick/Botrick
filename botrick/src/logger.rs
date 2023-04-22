use sporker::{getdb, Spork};
use tokio::sync::mpsc;

#[derive(Debug)]
struct LogActor {
    receiver: mpsc::UnboundedReceiver<LogActorMessage>,
    spork: Spork,
}
#[derive(Debug)]
enum LogActorMessage {
    Log { message: irc::proto::Message },
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
                let nick = message.source_nickname();
                let cmd = message.command.clone();
                if let irc::proto::Command::PRIVMSG(_, text) = cmd {
                    if !text.starts_with('\u{001}') || text.starts_with("\u{001}ACTION") {
                        if let Some(n) = nick {
                            self.spork.log_message(n, text.as_str());
                        }
                    }
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

    pub fn log(&self, message: irc::proto::Message) {
        let msg = LogActorMessage::Log { message };

        let _ = self.sender.send(msg);
    }
}

impl Default for LogActorHandle {
    fn default() -> Self {
        Self::new()
    }
}
