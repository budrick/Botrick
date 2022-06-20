use std::env;
extern crate botrick;
use crate::botrick::sporker;
use anyhow::Result;

fn main() -> Result<()> {
    // Spin up the database, and a Spork to use it.
    let db = sporker::getdb()?;
    let s = sporker::Spork::new(db);

    // Get all our cmdline args
    let args: Vec<String> = env::args().collect();

    // Fewer than 2 args? Go away.
    if args.len() < 2 {
        println!("Talking about nobody is it");
        return Ok(());
    }

    let saidby = &args[1];

    // If we have more than one arg, take the first one and run with it.
    // Otherwise, find out own start word. With blackjack. And hookers.
    let startw = match args.len() {
        2 => s.start_like(saidby),
        _ => s.start_with_word_like(&args[2], saidby),
    };

    // If we have a start word, go with it. Otherwise, error out stupidly.
    match startw {
        Some(word) => {
            let words = sporker::build_words_like(word, &s, saidby);
            println!("{}", words.join(" "));
        }
        _ => {
            println!("Talking about nobody and nothing is it");
        }
    }
    Ok(())
}
