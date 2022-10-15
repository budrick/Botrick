use std::path::PathBuf;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value = ".")]
    pub dir: Option<PathBuf>
}

pub fn parse() -> Args {
    Args::parse()
}