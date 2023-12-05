mod days;

extern crate clap;

use clap::Parser;
use std::io;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CLI {
    /// The day to run
    #[clap(short, long)]
    day: u8,

    #[clap(short, long)]
    part: Option<u8>,
}

fn main() -> io::Result<()> {
    let cli = CLI::parse();

    match cli.day {
        1 => days::day_one::run(cli.part.unwrap_or(1))?,
        _ => println!("Day not implemented"),
    }

    Ok(())
}
