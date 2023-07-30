use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, value_name = "DIR", default_value = ".")]
    /// Set Botrick's root directory
    pub dir: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Commands>,
}

pub fn parse() -> Args {
    Args::parse()
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Create a default configuration and data files
    Init,

    /// Run the bot
    Run,
}
