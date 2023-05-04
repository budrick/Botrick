use std::sync::Arc;

use tokio::sync::mpsc;
use irc::proto::FormattedStringExt;

use crate::{irc::CommandMessage, color::{colorize, Color}, config::Config};

struct DefaultActor {
    receiver: mpsc::UnboundedReceiver<ActorMessage>,
    sender: irc::client::Sender,
    config: Arc<Config>
}

#[derive(Debug)]
enum ActorMessage {
    Default { message: CommandMessage },
    Bots { message: CommandMessage },
}

impl DefaultActor {
    fn new(receiver: mpsc::UnboundedReceiver<ActorMessage>, sender: irc::client::Sender, config: Arc<Config>) -> Self {
        DefaultActor { receiver, sender, config }
    }
    fn handle_message(&mut self, msg: ActorMessage) {
        tracing::debug!("Got message {:?}", msg);
        match msg {
            ActorMessage::Default { message } => {
                let stripped = message.full_text.as_str().strip_formatting();
                for rej in &self.config.inspect_rejects {
                    if stripped.contains(rej) {
                        return;
                    }
                }
                let urls = get_urls(&message.full_text.as_str());
                if !self.config.inspect_urls || urls.is_empty() {
                    return;
                }
                let title = get_url_title(urls[0].as_str());
                if title.is_none() {
                    return;
                }
                let colbit = colorize(Color::Green, None, "LINK >>");
                let _ = self.sender
                    .send_privmsg(
                        &message.respond_to,
                        format!("{} {}", colbit, title.unwrap())
                    );
            },
            ActorMessage::Bots { message } => {
                let _ = self.sender.send_privmsg(message.respond_to, "Reporting in! [Rust🦀] just %spork or %sporklike, yo.".to_string());
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
        let actor = DefaultActor::new(receiver, ircsender, config);
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


pub fn get_urls(message: &str) -> Vec<linkify::Link> {
    let mut linkfinder = linkify::LinkFinder::new();
    linkfinder.kinds(&[linkify::LinkKind::Url]);
    linkfinder.links(message).collect()
}

pub fn get_url_title(url: &str) -> Option<String> {
    if url.is_empty() {
        return None;
    }
    let response_o = reqwest::blocking::get(url);
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
        let body = response.text().unwrap_or_default();
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