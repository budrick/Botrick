use irc::proto::Message;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

/// Convenience type for Tokio channels passing `irc::proto::Message`
pub type IrcMessageChannelizer = (UnboundedSender<Message>, UnboundedReceiver<Message>);
pub type StringChannelizer = (UnboundedSender<String>, UnboundedReceiver<String>);
