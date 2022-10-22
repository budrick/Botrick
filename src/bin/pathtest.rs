use std::path::PathBuf;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = ".")]
    dir: Option<PathBuf>
}

fn main() {
    let args = Args::parse();
    let cwd = std::env::current_dir().unwrap();
    println!("cwd: {}", cwd.display());
    std::env::set_current_dir(args.dir.unwrap().as_path()).unwrap();
    let cwd = std::env::current_dir().unwrap();
    println!("cwd: {}", cwd.display());
}