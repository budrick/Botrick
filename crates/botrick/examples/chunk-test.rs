use clap::Parser;
use color_eyre::eyre::Result;
use itertools::Itertools;
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

    let lines = open_file_lines(file)?;
    let mut processed_total = 0u128;
    for mut chunk in &lines.chunks(10_000) {
        let run_count = run_batch(&mut chunk)?;
        processed_total += run_count;
        println!("Processed {} lines", processed_total);
    }
    println!("Done");
    Ok(())
}

fn run_batch<T: Iterator<Item = Result<String, std::io::Error>>>(
    lines: &mut T,
) -> Result<u128, rusqlite::Error> {
    let mut proc_count = 0u128;

    for line in lines {
        proc_count += 1;
        if line.is_err() {
            println!("Error at {}: {:?}", proc_count, line);
            break;
        }
    }

    Ok(proc_count)
}

fn open_file_lines<P: AsRef<Path>>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
