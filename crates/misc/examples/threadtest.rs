use std::sync::mpsc::{Receiver, Sender, channel};
use std::thread;

use sporker::Spork;
use sporker::getdb;

// #[tokio::main]
fn main() {
    let (tx, rx): (Sender<&str>, Receiver<&str>) = channel();
    let th = thread::spawn(move || {
        if let Ok(db) = getdb() {
            let spork = Spork::new(db);
            println!("Threaded with spork, {:?}", spork);
        }
        while let Ok(thing) = rx.recv() {
            println!("Received {}", thing);
            if thing == "Fleb" {
                break;
            }
        }
    });
    tx.send("Fleb").expect("Failed to send???");
    th.join().expect("Thready deady");
    println!("Done");
}
