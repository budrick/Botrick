use irc::proto::Message;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

/// Convenience type for Tokio channels passing `irc::proto::Message`
pub type Channelizer = (UnboundedSender<Message>, UnboundedReceiver<Message>);