use color_eyre::eyre::Result;
use std::env;

fn main() -> Result<()> {
    // Spin up the database, and a Spork to use it.
    let db = sporker::getdb()?;
    let s = sporker::Spork::new(db);

    // Get all our cmdline args
    let args: Vec<String> = env::args().collect();

    // If we have more than one arg, take the first one and run with it.
    // Otherwise, find out own start word. With blackjack. And hookers.
    let startw = match args.len() {
        1 => s.start(),
        _ => s.start_with_word(&args[1]),
    };

    // If we have a start word, go with it. Otherwise, error out stupidly.
    match startw {
        Some(word) => {
            let words = sporker::build_words(word, &s);
            println!("{}", words.join(" "));
        }
        _ => {
            println!("Couldn't do it could I");
        }
    }
    Ok(())
}
