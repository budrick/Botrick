use std::{char, collections::HashMap, sync::Arc};

use regex::RegexSet;
use tokio::sync::mpsc;

use crate::irc::CommandMessage;

// use crate::bot::CommandMessage;

// use crate::bot::CommandMessage;
// use crate::color::{colorize, Color};

struct IrcActor {
    receiver: mpsc::UnboundedReceiver<ActorMessage>,
    _sender: irc::client::Sender,
    handlers: HashMap<String, Box<dyn super::Actor>>,
    regex_regexes: Vec<String>,
    regex_prefixes: Vec<Option<char>>,
    regex_handlers: Vec<Arc<dyn super::Actor>>,
    regexes: RegexSet,
    default_handler: Arc<dyn super::Actor>,
}

#[derive(Debug)]
enum ActorMessage {
    Register {
        command: String,
        handler: Box<dyn super::Actor>,
    },
    RegisterRegex {
        prefix: Option<char>,
        regexes: Vec<String>,
        handler: Arc<dyn super::Actor>,
    },
    RefreshRegexes,
    Process {
        message: Box<CommandMessage>,
    },
}

impl IrcActor {
    fn new(
        receiver: mpsc::UnboundedReceiver<ActorMessage>,
        sender: irc::client::Sender,
        default_handler: Arc<dyn super::Actor>,
    ) -> Self {
        IrcActor {
            receiver,
            _sender: sender,
            handlers: HashMap::new(),
            regex_regexes: Vec::new(),
            regex_prefixes: Vec::new(),
            regex_handlers: Vec::new(),
            regexes: RegexSet::default(),
            default_handler,
        }
    }
    fn handle_message(&mut self, msg: ActorMessage) {
        tracing::debug!("Got message: {:?}", msg);
        match msg {
            ActorMessage::Register { command, handler } => {
                self.handlers.insert(command, handler);
            }
            ActorMessage::RegisterRegex {
                prefix,
                regexes,
                handler,
            } => {
                for reg in regexes {
                    self.regex_regexes.push(reg);
                    self.regex_prefixes.push(prefix);
                    self.regex_handlers.push(handler.clone());
                }
            }
            ActorMessage::RefreshRegexes => {
                self.regexes = RegexSet::new(self.regex_regexes.clone()).unwrap_or_default();
            }
            ActorMessage::Process { message } => {
                let matches: Vec<_> = self
                    .regexes
                    .matches(message.full_text.as_str())
                    .into_iter()
                    .collect();
                tracing::debug!("{:?}", matches);
                if matches.is_empty() {
                    self.default_handler.process(*message);
                } else {
                    let handler = self.regex_handlers.get(matches[0]).unwrap();
                    handler.process(*message);
                }

                // // TODO: Log anything that doesn't match a command.
                // if let Some(h) = self.handlers.get(&message.command) {
                //     tracing::debug!("Passing to handler: {:?}", message);
                //     h.process(*message);
                // }
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
    pub fn new(ircsender: irc::client::Sender, default_handler: Arc<dyn super::Actor>) -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();
        let actor = IrcActor::new(receiver, ircsender, default_handler);
        tokio::spawn(run_my_actor(actor));

        Self { sender }
    }

    pub fn process(&self, message: irc::proto::Message) {
        tracing::debug!("Received: {}", message);
        let _ = self.sender.send(ActorMessage::Process {
            message: Box::new(message.into()),
        });
    }

    pub fn register<S>(&self, command: S, handler: Box<dyn super::Actor>)
    where
        S: AsRef<str>,
    {
        let _ = self.sender.send(ActorMessage::Register {
            command: command.as_ref().to_string(),
            handler,
        });
    }

    pub fn register_regex<I, S>(
        &self,
        regexes: I,
        handler: Arc<dyn super::Actor>,
        prefix: Option<char>,
    ) where
        S: AsRef<str>,
        I: IntoIterator<Item = S>,
    {
        let mut res: Vec<String> = Vec::new();
        for r in regexes {
            res.push(r.as_ref().to_string());
        }
        let _ = self.sender.send(ActorMessage::RegisterRegex {
            prefix: None,
            regexes: res,
            handler,
        });
    }

    pub fn register_prefixed<I, S>(&self, prefix: char, commands: I, handler: Arc<dyn super::Actor>)
    where
        S: AsRef<str>,
        I: IntoIterator<Item = S>,
    {
        let mut res: Vec<String> = Vec::new();
        for c in commands {
            res.push(c.as_ref().to_string());
        }
        let cmds = self.create_prefixed_regex(prefix, res);
        self.register_regex(cmds, handler, Some(prefix));
    }

    pub fn create_prefixed_regex<I: IntoIterator<Item = S>, S: ToString>(
        &self,
        prefix: char,
        regexes: I,
    ) -> Vec<String> {
        let mut res: Vec<String> = Vec::new();
        for r in regexes {
            res.push(format!(r"^{}{}\b", prefix, r.to_string()));
        }
        res
    }

    pub fn refresh_regexes(&self) {
        let _ = self.sender.send(ActorMessage::RefreshRegexes);
    }
}
