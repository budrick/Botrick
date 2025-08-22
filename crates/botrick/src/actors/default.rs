use std::sync::Arc;

use irc::proto::FormattedStringExt;
use tokio::sync::mpsc;

use crate::{
    actors::logger::LogActorHandle,
    color::{Color, colorize},
    config::Config,
    irc::CommandMessage,
};

struct DefaultActor {
    receiver: mpsc::UnboundedReceiver<ActorMessage>,
    sender: irc::client::Sender,
    config: Arc<Config>,
    logger: crate::actors::LogActorHandle,
}

#[derive(Debug)]
enum ActorMessage {
    Default { message: CommandMessage },
    Bots { message: CommandMessage },
}

impl DefaultActor {
    fn new(
        receiver: mpsc::UnboundedReceiver<ActorMessage>,
        sender: irc::client::Sender,
        config: Arc<Config>,
        logger: crate::actors::LogActorHandle,
    ) -> Self {
        DefaultActor {
            receiver,
            sender,
            config,
            logger,
        }
    }
    fn handle_message(&mut self, msg: ActorMessage) {
        tracing::debug!("Got message {:?}", msg);
        match msg {
            ActorMessage::Default { message } => {
                tokio::spawn(scan_urls(
                    message.clone(),
                    self.config.clone(),
                    self.sender.clone(),
                ));
                self.logger.log(message);
            }
            ActorMessage::Bots { message } => {
                let _ = self.sender.send_privmsg(
                    message.respond_to,
                    format!(
                        "Reporting in! [Rust 🦀] just %spork or %sporklike, yo. v{}",
                        crate::PKG_VERSION
                    ),
                );
            }
        };
    }
}

async fn run_my_actor(mut actor: DefaultActor) {
    while let Some(msg) = actor.receiver.recv().await {
        actor.handle_message(msg);
    }
}

#[derive(Clone)]
pub struct DefaultActorHandle {
    sender: mpsc::UnboundedSender<ActorMessage>,
}

impl DefaultActorHandle {
    pub fn new(ircsender: irc::client::Sender, config: Arc<Config>) -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();
        let actor = DefaultActor::new(receiver, ircsender, config, LogActorHandle::new());
        tokio::spawn(run_my_actor(actor));

        Self { sender }
    }
}

impl super::api::Actor for DefaultActorHandle {
    fn process(&self, message: CommandMessage) {
        tracing::debug!("Default Actor Handle received: {:?}", message);
        if message.command == ".bots" {
            let _ = self.sender.send(ActorMessage::Bots { message });
        } else {
            let _ = self.sender.send(ActorMessage::Default { message });
        }
    }
}

async fn scan_urls(message: CommandMessage, config: Arc<Config>, sender: irc::client::Sender) {
    let stripped = message.full_text.as_str().strip_formatting();
    for rej in &config.inspect_rejects {
        if stripped.contains(rej) {
            return;
        }
    }
    let urls = get_urls(message.full_text.as_str());
    if !config.inspect_urls || urls.is_empty() {
        return;
    }
    let title = get_url_title(urls[0].as_str()).await;
    if title.is_none() {
        return;
    }
    let colbit = colorize(Color::Green, None, "LINK >>");
    let _ = sender.send_privmsg(
        &message.respond_to,
        format!("{} {}", colbit, title.unwrap()),
    );
}

pub fn get_urls(message: &'_ str) -> Vec<linkify::Link<'_>> {
    let mut linkfinder = linkify::LinkFinder::new();
    linkfinder.kinds(&[linkify::LinkKind::Url]);
    linkfinder.links(message).collect()
}

async fn get_url_title(url: &str) -> Option<String> {
    if url.is_empty() {
        return None;
    }
    let response_o = reqwest::get(url).await;
    if response_o.is_err() {
        return None;
    }
    let response = response_o.unwrap();
    let content_type = response.headers().get("content-type");
    // if content_type.is_none() {
    //     return None;
    // }
    content_type?; // weeeeeeird
    if content_type
        .unwrap()
        .to_str()
        .unwrap_or_default()
        .starts_with("text/html")
    {
        let body = response.text().await.unwrap_or_default();
        let doc = select::document::Document::from(body.as_str());
        let f: Vec<_> = doc.find(select::predicate::Name("title")).take(1).collect();
        if f.is_empty() {
            return None;
        }
        Some(f[0].text())
    } else {
        None
    }
}
