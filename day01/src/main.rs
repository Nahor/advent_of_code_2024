use clap::{Parser, Subcommand};
use common::read_input_str;
use day01::{part1, part2};
use miette::Result;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug)]
#[command(rename_all = "lower")]
enum Command {
    /// Part 1
    Part1 { file: Option<PathBuf> },

    /// Part 2
    Part2 { file: Option<PathBuf> },

    /// Part 2 (using parsing directly into a hashmap)
    Part2Map { file: Option<PathBuf> },

    /// Part 2 (using parsing into a vector+hashmap)
    Part2VecMap { file: Option<PathBuf> },
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Some(Command::Part1 { file }) => {
            println!("Result: {}", part1::run(&read_input_str!(file)?)?)
        }
        Some(Command::Part2 { file }) => {
            println!("Result: {}", part2::run(&read_input_str!(file)?)?)
        }
        Some(Command::Part2Map { file }) => {
            println!("Result: {}", part2::run_map(&read_input_str!(file)?)?)
        }
        Some(Command::Part2VecMap { file }) => {
            println!("Result: {}", part2::run_vecmap(&read_input_str!(file)?)?)
        }
        None => println!("Result: {}", part1::run(&read_input_str!(None)?)?),
    }

    Ok(())
}
