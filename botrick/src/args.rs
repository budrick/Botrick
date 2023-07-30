use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, value_name = "DIR", default_value = ".")]
    /// Set Botrick's root directory
    pub dir: Option<PathBuf>,
}

pub fn parse() -> Args {
    Args::parse()
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Create a default configuration and data files
    Init,

    /// Run the bot
    Run,
}
