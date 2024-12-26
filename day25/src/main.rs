use clap::{Parser, Subcommand};
use common::read_input_u8;
use day25::{part1, part1_logic_sum};
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
    /// Part 1
    Part1LogicSum { file: Option<PathBuf> },

    /// Part 2
    Part2 { file: Option<PathBuf> },
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Some(Command::Part1 { file }) => {
            println!("Result: {}", part1::run(&read_input_u8!(file)?)?)
        }
        Some(Command::Part1LogicSum { file }) => {
            println!("Result: {}", part1_logic_sum::run(&read_input_u8!(file)?)?)
        }
        Some(Command::Part2 { file: _ }) => {
            println!("There is no part 2 on the last day")
        }
        None => println!("Result: {}", part1::run(&read_input_u8!(None)?)?),
    }

    Ok(())
}
