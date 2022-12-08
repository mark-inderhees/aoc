use anyhow::{bail, Result};
use clap::Parser;
use puzzle::Puzzle;
use regex::Regex;
use std::fs;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
// __BOOTSTRAP_MOD__
mod file_system;
mod puzzle;
mod rock_paper_scissors;
mod utils;

/// Runner for Advent of Code
#[derive(Parser, Debug)]
struct Args {
    /// Which day's code to run
    #[arg(
        long,
        short,
        default_value_t = 8, // __BOOTSTRAP_DAY__
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

    /// Validate all parts and input for the currently selected day
    #[arg(long, short)]
    validate: bool,

    /// Bootstrap a new day
    #[arg(long, short, value_name = "DAY")]
    bootstrap: Option<u32>,

    /// Set the log level
    #[arg(
        long,
        short,
        value_name = "LEVEL",
        default_value_t = log::LevelFilter::Info,
    )]
    logs: log::LevelFilter,
}

fn run_day<DayType: puzzle::Puzzle>(part: u32, input: String, test: bool) -> Result<()> {
    let raw_input = fs::read_to_string(input).expect("Input file error");
    let mut day: DayType = Puzzle::from_input(&raw_input)?;
    let output = match part {
        1 => day.solve_part1()?,
        2 => day.solve_part2()?,
        _ => bail!("Invalid part"),
    };
    let expect = match part {
        1 => day.answer_part1(test),
        2 => day.answer_part2(test),
        _ => bail!("Invalid part"),
    };
    match expect {
        Some(expected_val) => {
            println!(
                "Solution: {} == {} is {}\n",
                output,
                expected_val,
                output == expected_val
            );
            assert_eq!(output, expected_val);
        }
        _ => println!("Solution: {}\n", output),
    }

    Ok(())
}

fn bootstrap(day: u32) -> Result<()> {
    println!("Bootstrapping day {}", day);
    let source = ["src/dayXX.rs", "input/dayXX.test", "input/dayXX.input"];
    let dest = [
        format!("src/day{day:02}.rs"),
        format!("input/day{day:02}.test"),
        format!("input/day{day:02}.input"),
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
    main = re_run.replace(&main, format!("${{1}}{day} => run_day::<day{day:02}::Day{day:02}>(part, input, test)?,\r\n${{1}}${{2}}")).to_string();
    fs::write(main_rs, main)?;

    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();

    env_logger::Builder::from_default_env()
        .filter(None, args.logs)
        .init();

    match args.bootstrap {
        Some(day) => {
            bootstrap(day)?;
            return Ok(());
        }
        None => (),
    };

    let day = args.day;
    let part = args.part;
    let test = args.test;
    let runs = match args.validate {
        false => vec![(day, part, test)],
        true => vec![
            (day, 1, true),
            (day, 1, false),
            (day, 2, true),
            (day, 2, false),
        ],
    };

    for run in runs {
        let day = run.0;
        let part = run.1;
        let test = run.2;
        println!("\nRunning day={} part={} test={} ...", day, part, test);
        let input_type = match test {
            true => "test",
            false => "input",
        };
        let input = format!("input/day{:02}.{}", day, input_type);

        match day {
            1 => run_day::<day01::Day01>(part, input, test)?,
            2 => run_day::<day02::Day02>(part, input, test)?,
            3 => run_day::<day03::Day03>(part, input, test)?,
            4 => run_day::<day04::Day04>(part, input, test)?,
            5 => run_day::<day05::Day05>(part, input, test)?,
            6 => run_day::<day06::Day06>(part, input, test)?,
            7 => run_day::<day07::Day07>(part, input, test)?,
            8 => run_day::<day08::Day08>(part, input, test)?,
            // __BOOTSTRAP_RUN__
            _ => bail!("Day {} not found", day),
        }
    }

    Ok(())
}
