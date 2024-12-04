use clap::{Parser, Subcommand};
use day03::{input::read_input, part1, part2};
use miette::Result;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Part 1
    Part1 { file: Option<PathBuf> },
    /// Part 1
    Part1Winnow { file: Option<PathBuf> },

    /// Part 2
    Part2 { file: Option<PathBuf> },

    /// Part 2
    Part2Winnow { file: Option<PathBuf> },
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Some(Command::Part1 { file }) => println!("Result: {}", part1::run(&read_input(file)?)?),
        Some(Command::Part1Winnow { file }) => {
            println!("Result: {}", part1::run_winnow(&read_input(file)?)?)
        }
        Some(Command::Part2 { file }) => println!("Result: {}", part2::run(&read_input(file)?)?),
        Some(Command::Part2Winnow { file }) => {
            println!("Result: {}", part2::run_winnow(&read_input(file)?)?)
        }
        None => println!("Result: {}", part1::run(&read_input(None)?)?),
    }

    Ok(())
}