use anyhow::{bail, Result};
use clap::Parser;
use puzzle::Puzzle;
use std::fs;

mod day01;
mod day06;
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

    /// Bootstrap a new day
    #[arg(long)]
    bootstrap: Option<u32>,
}

fn run_day<DayType: puzzle::Puzzle>(part: u32, input: String) -> Result<()> {
    let raw_input = fs::read_to_string(input).expect("Input file error");
    let mut day: DayType = Puzzle::from_input(&raw_input)?;
    let output = match part {
        1 => day.solve_part1()?,
        2 => day.solve_part2()?,
        _ => bail!("Invalid part"),
    };
    println!("\n\nSolution: {}\n\n", output);

    Ok(())
}

fn bootstrap(day: u32) -> Result<()> {
    println!("Bootstrapping day {}", day);
    let source = ["src/dayXX.rs", "src/dayXX.test", "src/dayXX.input"];
    let dest = [
        format!("src/day{:02}.rs", day),
        format!("src/day{:02}.test", day),
        format!("src/day{:02}.input", day),
    ];
    for (s, d) in source.iter().zip(dest.iter()) {
        if fs::metadata(d).is_ok() {
            bail!("Already exists");
        }
        fs::copy(s, d)?;
    }

    // Change DayXX to a real number like Day01
    let day_rs = &dest[0];
    let contents = fs::read_to_string(day_rs)?;
    let new_contents = contents.replace("DayXX", &format!("Day{:02}", day));
    fs::write(day_rs, new_contents)?;

    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.bootstrap {
        Some(day) => {
            bootstrap(day)?;
            return Ok(());
        }
        None => (),
    };

    println!(
        "\n\nRunning day={} part={} test={} ...",
        args.day, args.part, args.test
    );
    let input_type = match args.test {
        true => "test",
        false => "input",
    };
    let input = format!("src/day{:02}.{}", args.day, input_type);
    match args.day {
        1 => run_day::<day01::Day01>(args.part, input)?,
        6 => run_day::<day06::Day06>(args.part, input)?,
        _ => bail!("Day {} not found", args.day),
    }

    Ok(())
}
