use clap::Parser;
use color_eyre::eyre::Result;
use std::{
    fs::File,
    io::{self, BufRead},
    path::{Path, PathBuf},
};

// List of blocked initial words
// static BLOCKLIST: [&str; 2] = ["!speak", "!talklike"];

// Command line arg definitions
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    pub file: Option<PathBuf>,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Args::parse();
    let file = std::fs::canonicalize(args.file.unwrap())?;

    let mut lines = open_file_lines(file)?;
    let mut processed_total = 0u128;

    while let Ok(processed) = process_lines(&mut lines) {
        if processed == 0 {
            break;
        }
        processed_total += processed;
        println!("Processed {} lines", processed_total);
    }
    println!("Done");
    Ok(())
}

fn process_lines<T: Iterator<Item = Result<String, std::io::Error>>>(
    lines: &mut T,
) -> Result<u128, rusqlite::Error> {
    let mut processed = 0u128;

    for oline in lines {
        processed += 1;
        if oline.is_err() {
            println!("Error at {}: {:?}", processed, oline);
            continue;
        }

        if processed % 10_000 == 0 {
            break;
        }
    }

    Ok(processed)
}

fn open_file_lines<P: AsRef<Path>>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
