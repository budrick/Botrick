use clap::Parser;
use color_eyre::eyre::Result;
use itertools::Itertools;
use rusqlite::named_params;
use std::{
    fs::File,
    io::{self, BufRead},
    path::{Path, PathBuf},
};

// List of blocked initial words
static BLOCKLIST: [&str; 2] = ["!speak", "!talklike"];

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

    let mut db = sporker::opendb("werdz.sqlite")?;

    sporker::create_table(&db)?;

    let lines = open_file_lines(file)?;
    let mut processed_total = 0u128;
    for mut chunk in &lines.chunks(10_000) {
        let mut tx = db.transaction()?;
        let run_count = run_batch(&mut tx, &mut chunk)?;
        processed_total += run_count;
        tx.commit()?;
        println!("Processed {} lines", processed_total);
    }
    println!("Creating indexes");
    sporker::create_indexes(&db)?;
    println!("Done");
    Ok(())
}

fn run_batch<T: Iterator<Item = Result<String, std::io::Error>>>(
    tx: &mut rusqlite::Transaction<'_>,
    lines: &mut T,
) -> Result<u128, rusqlite::Error> {
    let mut proc_count = 0u128;
    let mut stmt = sporker::get_log_stmt(tx);

    while let Some(Ok(line)) = lines.next() {
        proc_count += 1;
        // irssi timestamp is 19 chars long, and we want to test the next 3
        let line_notime = &line[19..];

        if line_notime.len() < 5 {
            continue;
        }

        let mut nick_start = 0;
        let mut nick_end = 0;
        let mut words_offset = 0;

        // Handle regular lines
        if line_notime.chars().next().unwrap_or('-') == '<' {
            nick_start = 2;
            nick_end = line_notime.find('>').unwrap_or(0);
            words_offset = 2;
        }

        // Handle actions
        if line_notime.chars().nth(1).unwrap_or('-') == '*' {
            nick_start = 3;
            nick_end = nick_start + line_notime[nick_start..].find(' ').unwrap_or(0);
            words_offset = 1;
        }

        // Somehow we failed to find a nick? Just fail and continue.
        if nick_end == 0 {
            // println!("Skipped {}", line_notime);
            continue;
        }

        let nick = &line_notime[nick_start..nick_end];
        let words: Vec<&str> = line_notime[nick_end + words_offset..]
            .split_ascii_whitespace()
            .collect();

        if nick.is_empty() || words.is_empty() {
            continue;
        }

        // Do the database stuff
        let words_iter = words.iter().enumerate();

        // Single word? Don't even try.
        if words.len() < 2 {
            continue;
        }

        // If the first word is in our blocklist, ignore it.
        if BLOCKLIST.contains(&words[0]) {
            continue;
        }

        // Otherwise, LET'S LOGGING
        for (i, word) in words_iter {
            let mut prev = "";
            let mut next = "";
            if i != 0 {
                prev = words[i - 1];
            }
            if i != words.len() - 1 {
                next = words[i + 1];
            }
            let _res = stmt.execute(
                named_params! {":werd": word, ":nextwerd": next, ":prevwerd": prev, ":saidby": nick},
            );
        }
    }

    Ok(proc_count)
}

fn open_file_lines<P: AsRef<Path>>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
