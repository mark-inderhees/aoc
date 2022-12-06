use anyhow::{bail, Result};
use clap::Parser;
use puzzle::Puzzle;
use regex::Regex;
use std::fs;

mod day01;
mod day06;
mod day07;
// __BOOTSTRAP_MOD__
mod puzzle;

/// Runner for Advent of Code
#[derive(Parser, Debug)]
struct Args {
    /// Which day's code to run
    #[arg(
        long,
        short,
        default_value_t = 7, // __BOOTSTRAP_DAY__
    )]
    day: u32,

    /// Which part of the day's code to run
    #[arg(
        long,
        short,
        default_value_t = 1, // __BOOTSTRAP_PART__
    )]
    part: u32,

    /// Run test data instead of input
    #[arg(
        long,
        short,
        value_name = "BOOL",
        action = clap::ArgAction::Set,
        default_value_t = true, // __BOOTSTRAP_TEST__
    )]
    test: bool,

    /// Bootstrap a new day
    #[arg(long, short, value_name = "DAY")]
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
        format!("src/day{day:02}.rs"),
        format!("src/day{day:02}.test"),
        format!("src/day{day:02}.input"),
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
    let new_contents = contents.replace("DayXX", &format!("Day{day:02}"));
    fs::write(day_rs, new_contents)?;

    // Reset values in main.rs
    let main_rs = "src/main.rs";
    let mut main = fs::read_to_string(main_rs)?;
    let re_mod = Regex::new(r"(// __BOOTSTRAP_MOD__)")?;
    let re_day = Regex::new(r"\d+(, // __BOOTSTRAP_DAY__)")?;
    let re_part = Regex::new(r"\d+(, // __BOOTSTRAP_PART__)")?;
    let re_test = Regex::new(r"false(, // __BOOTSTRAP_TEST__)")?;
    let re_run = Regex::new(r"( +)(// __BOOTSTRAP_RUN__)")?;
    main = re_mod
        .replace(&main, format!("mod day{day:02};\r\n${{1}}"))
        .to_string();
    main = re_day.replace(&main, format!("{day}${{1}}")).to_string();
    main = re_part.replace(&main, "1${1}").to_string();
    main = re_test.replace(&main, "true${1}").to_string();
    main = re_run.replace(&main, format!("${{1}}{day} => run_day::<day{day:02}::Day{day:02}>(args.part, input)?,\r\n${{1}}${{2}}")).to_string();
    fs::write(main_rs, main)?;

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
        7 => run_day::<day07::Day07>(args.part, input)?,
        // __BOOTSTRAP_RUN__
        _ => bail!("Day {} not found", args.day),
    }

    Ok(())
}
