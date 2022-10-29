macro_rules! create_bot_command {
    ($name: ident, $f: stmt, $p:ident) => {

        #[derive(Debug)]
        #[allow(dead_code)]
        pub struct $name {
            pub params: CommandParams,
        }

        impl Command for $name {
            fn execute(&$p) -> CommandResult {
                $f
            }
        }
    };
}

macro_rules! bot_commands {

    ($_self:expr, $params:ident, [$($cmd: pat => $handler:expr,)*]) => {
        match $_self {
            $(
                $cmd => Some(Box::new($handler)),
            )*
            _ => None,
        }
    }
}

pub(crate) use bot_commands;
pub(crate) use create_bot_command;
