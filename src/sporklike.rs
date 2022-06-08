use std::env;

mod sporker;

fn main() {
    // Spin up the database, and a Spork to use it.
    let db = sporker::getdb();
    let s = sporker::Spork::new(db);

    // Get all our cmdline args
    let args: Vec<String> = env::args().collect();
    
    // Fewer than 2 args? Go away.
    if args.len() < 2 {
        println!("Talking about nobody is it");
        ()
    }

    let saidby = &args[1];
    
    // If we have more than one arg, take the first one and run with it.
    // Otherwise, find out own start word. With blackjack. And hookers.
    let startw = match args.len() {
        2 => s.start(Some(&args[1])),
        _ => s.start_with_word(&args[2], Some(&args[1]))
    };
    println!("{:?}", startw);
    // If we have a start word, go with it. Otherwise, error out stupidly.
    match startw {
        Some(word) => {
            let words = sporker::build_words(word, &s, Some(saidby));
            println!("{}", words.join(" "));
        }
        _ => {
            println!("Talking about nobody and nothing is it");
        }
    }
}