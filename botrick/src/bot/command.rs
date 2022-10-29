macro_rules! create_bot_command {
    ($name: ident, $f: stmt, $p:ident) => {

        #[derive(Debug)]
        #[allow(dead_code)]
        pub struct $name {
            sender: irc::client::Sender,
            command: crate::bot::CommandMessage,
            config: crate::config::Config,
        }

        impl Command for $name {
            fn execute(&$p) -> CommandResult {
                $f
            }
        }
    };
}

macro_rules! bot_command {
    ($name: ident, $params:ident) => {
        $name { $params }
    };
}

pub(crate) use bot_command;
pub(crate) use create_bot_command;
