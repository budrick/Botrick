use clap::Parser;
use std::io::BufRead;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    pub file: Option<std::path::PathBuf>,
}

fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;
    let args = Args::parse();
    let file = std::fs::canonicalize(args.file.unwrap())?;

    let lines = open_file_lines(file)?;
    // let mut processed_total = 0u128;
    // for _ in lines {
    //     processed_total += 1;
    // }
    // println!("{}", processed_total);
    println!("{}", lines.count());
    Ok(())
}

fn open_file_lines<P: AsRef<std::path::Path>>(
    filename: P,
) -> std::io::Result<std::io::Lines<std::io::BufReader<std::fs::File>>> {
    let file = std::fs::File::open(filename)?;
    Ok(std::io::BufReader::new(file).lines())
}
