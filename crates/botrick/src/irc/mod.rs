#[derive(Debug, Clone)]
pub struct CommandMessage {
    pub command: String,
    pub sent_by: String,
    pub respond_to: String,
    pub params: String,
    pub full_text: String,
}
impl From<irc::proto::Message> for CommandMessage {
    fn from(item: irc::proto::Message) -> CommandMessage {
        if let irc::proto::Command::PRIVMSG(ref _channel, ref text) = item.command {
            let command = text.split(' ').next().unwrap_or("").to_string();
            let sent_by = item.source_nickname().unwrap_or("").to_string();
            let respond_to = item.response_target().unwrap_or("").to_string();
            let params = text
                .strip_prefix(&command)
                .unwrap_or("")
                .strip_prefix(' ')
                .unwrap_or("")
                .to_string();
            let full_text = text.to_string();
            CommandMessage {
                command,
                sent_by,
                respond_to,
                params,
                full_text,
            }
        } else {
            CommandMessage {
                command: "".to_string(),
                sent_by: "".to_string(),
                respond_to: "".to_string(),
                params: "".to_string(),
                full_text: "".to_string(),
            }
        }
    }
}
