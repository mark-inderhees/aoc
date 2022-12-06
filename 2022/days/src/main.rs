use anyhow::{bail, Result};
use clap::Parser;
use puzzle::Puzzle;
use std::fs;

mod day01;
mod puzzle;

/// Runner for Advent of Code
#[derive(Parser, Debug)]
struct Args {
    /// Which day's code to run
    #[arg(long, default_value_t = 1)]
    day: u32,

    /// Which part of the day's code to run
    #[arg(long, default_value_t = 1)]
    part: u32,

    /// Run test data instead of input
    #[arg(long, default_value_t = false)]
    test: bool,
}

fn run_day<DayType: puzzle::Puzzle>(part: u32, input: String) {
    let raw_input = fs::read_to_string(input).expect("read err");
    let mut day: DayType = Puzzle::from_input(&raw_input).expect("input err");
    let output = match part {
        1 => day.solve_part1().expect("part1 err"),
        2 => day.solve_part2().expect("part2 err"),
        _ => "invalid part".to_string(),
    };
    println!("SOLUTION: {}", output);
}

fn main() -> Result<()> {
    let args = Args::parse();
    println!("RUNNING DAY {} PART {} TEST={}...", args.day, args.part, args.test);
    let input_type = match args.test {
        true => "test",
        false => "input",
    };
    let input = format!("src/day{:02}.{}", args.day, input_type);
    match args.day {
        // 1 => run_day!(day1, day01::Day01, args.part, format!("day{:02}.input", args.day)),
        1 => run_day::<day01::Day01>(args.part, input),
        _ => bail!("day {} not found", args.day),
    }

    Ok(())
}
