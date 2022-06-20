pub mod ircbot;
pub mod sporker;

use irc::proto::Message;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

pub type Channelizer = (UnboundedSender<Message>, UnboundedReceiver<Message>);
