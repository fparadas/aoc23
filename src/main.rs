mod days;
extern crate clap;

use clap::Parser;
use std::io;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The day to run
    #[clap(short, long)]
    day: u8,

    #[clap(short, long)]
    part: Option<u8>,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    match cli.day {
        1 => days::day_one::run(cli.part.unwrap_or(1))?,
        2 => days::day_two::run(cli.part.unwrap_or(1))?,
        3 => days::day_three::run(cli.part.unwrap_or(1))?,
        4 => days::day_four::run(cli.part.unwrap_or(1))?,
        _ => println!("Day not implemented"),
    }

    Ok(())
}
