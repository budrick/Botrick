static BLOCKLIST: [&str; 2] = ["speak", "talklike"];

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        panic!("Wrong number of args. One only.");
    }

    println!("{:?}", BLOCKLIST.contains(&args[1].as_str()))
}
