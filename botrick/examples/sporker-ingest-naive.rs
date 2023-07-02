use clap::Parser;
use color_eyre::{eyre::Result};
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

    let mut lines = open_file_lines(file)?;
    let mut processed_total = 0u128;

    while let Ok(processed) = process_lines(&mut db, &mut lines) {
        if processed == 0 {
            break
        }
        processed_total += processed;
        println!("Processed: {}", processed_total);
    }
    //_total Create indexes and report
    println!("Creating indexes");
    sporker::create_indexes(&db)?;
    println!("Done");
    Ok(())
}

fn process_lines<T: Iterator<Item = Result<String, std::io::Error>>>(db: &mut rusqlite::Connection, lines: &mut T) -> Result<u128, rusqlite::Error> {
    let tx = db.transaction()?;
    let mut stmt = sporker::get_log_stmt(&tx);

    let mut processed = 0u128;

    for oline in lines {
        processed += 1;
        if oline.is_err() {
            continue
        }
        let line = oline.unwrap();

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

        if processed % 10_000 == 0 {
            break
        }
    }
    drop(stmt);
    tx.commit()?;

    Ok(processed)
}

fn open_file_lines<P: AsRef<Path>>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
