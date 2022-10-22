use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

extern crate botrick;
use crate::botrick::sporker::getdb;
use crate::botrick::sporker::Spork;

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
