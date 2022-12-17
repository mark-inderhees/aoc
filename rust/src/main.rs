use anyhow::{bail, Result};
use clap::Parser;
use puzzle::Puzzle;
use regex::Regex;
use std::fs;

mod puzzle;
mod utils;
mod year2022;

/// Runner for Advent of Code
#[derive(Parser, Debug)]
struct Args {
    /// Which day's code to run
    #[arg(
        long,
        short,
        default_value_t = 17, // __BOOTSTRAP_DAY__
    )]
    day: u32,

    /// Which part of the day's code to run
    #[arg(
        long,
        short,
        default_value_t = 1, // __BOOTSTRAP_PART__
    )]
    part: u32,

    /// Which year to run for
    #[arg(long, short, default_value_t = 2022)]
    year: u32,

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

    /// Validate all parts and input for all days for the currently selected year
    #[arg(long, short)]
    comprehensive: bool,

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
            log::info!(
                "Solution: {} == {} is {}\n\n",
                output,
                expected_val,
                output == expected_val
            );
            assert_eq!(output, expected_val);
        }
        _ => log::info!("Solution: {}\n\n", output),
    }

    Ok(())
}

fn bootstrap(day: u32, year: u32) -> Result<()> {
    println!("Bootstrapping day {}", day);
    let source = ["src/dayXX.rs", "input/dayXX.test", "input/dayXX.input"];
    let dest = [
        format!("src/year{year}/day{day:02}.rs"),
        format!("input/{year}/day{day:02}.test"),
        format!("input/{year}/day{day:02}.input"),
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
    let re_day = Regex::new(r"\d+(, // __BOOTSTRAP_DAY__)")?;
    let re_part = Regex::new(r"\d+(, // __BOOTSTRAP_PART__)")?;
    let re_test = Regex::new(r"false(, // __BOOTSTRAP_TEST__)")?;
    let re_run = Regex::new(r"( +)(// __BOOTSTRAP_RUN__)")?;
    main = re_day.replace(&main, format!("{day}${{1}}")).to_string();
    main = re_part.replace(&main, "1${1}").to_string();
    main = re_test.replace(&main, "true${1}").to_string();
    main = re_run.replace(&main, format!("${{1}}{day} => run_day::<year{year}::day{day:02}::Day{day:02}>(part, input, test)?,\n${{1}}${{2}}")).to_string();
    fs::write(main_rs, main)?;

    // Add a line to year's mod.rs
    let mod_rs = &format!("src/year{year}/mod.rs");
    let mut mod_text = fs::read_to_string(mod_rs)?;
    mod_text += &format!("\npub mod day{day:02};");
    fs::write(mod_rs, mod_text)?;

    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();

    env_logger::Builder::from_default_env()
        .filter(None, args.logs)
        .format(|_, record| {println!("{}", record.args()); Ok(())})
        .filter_level(log::LevelFilter::Warn)
        .init();

    match args.bootstrap {
        Some(day) => {
            bootstrap(day, args.year)?;
            return Ok(());
        }
        None => (),
    };

    let years = vec![args.year];
    let mut days = vec![args.day];
    let mut parts = vec![args.part];
    let mut tests = vec![args.test];
    if args.validate {
        parts = (1..=2).collect();
        tests = vec![true, false];
    } else if args.comprehensive {
        days = (1..=25).collect();
        parts = (1..=2).collect();
        tests = vec![true, false];
    }

    let mut runs = vec![];
    {
        for year in &years {
            for day in &days {
                for part in &parts {
                    for test in &tests {
                        runs.push((*day, *year, *part, *test));
                    }
                }
            }
        }
    }

    println!("\n"); // Empty line
    for run in runs {
        let day = run.0;
        let year = run.1;
        let part = run.2;
        let test = run.3;
        println!("Running day={} part={} test={} ...", day, part, test);
        let input_type = match test {
            true => "test",
            false => "input",
        };
        let input = format!("input/{year}/day{day:02}.{input_type}");

        match year {
            2022 => match day {
                1 => run_day::<year2022::day01::Day01>(part, input, test)?,
                2 => run_day::<year2022::day02::Day02>(part, input, test)?,
                3 => run_day::<year2022::day03::Day03>(part, input, test)?,
                4 => run_day::<year2022::day04::Day04>(part, input, test)?,
                5 => run_day::<year2022::day05::Day05>(part, input, test)?,
                6 => run_day::<year2022::day06::Day06>(part, input, test)?,
                7 => run_day::<year2022::day07::Day07>(part, input, test)?,
                8 => run_day::<year2022::day08::Day08>(part, input, test)?,
                9 => run_day::<year2022::day09::Day09>(part, input, test)?,
                10 => run_day::<year2022::day10::Day10>(part, input, test)?,
                11 => run_day::<year2022::day11::Day11>(part, input, test)?,
                12 => run_day::<year2022::day12::Day12>(part, input, test)?,
                13 => run_day::<year2022::day13::Day13>(part, input, test)?,
                14 => run_day::<year2022::day14::Day14>(part, input, test)?,
                15 => run_day::<year2022::day15::Day15>(part, input, test)?,
                16 => run_day::<year2022::day16::Day16>(part, input, test)?,
                17 => run_day::<year2022::day17::Day17>(part, input, test)?,
                // __BOOTSTRAP_RUN__
                _ => {
                    println!("Day {} not found, goodbye!", day);
                    break;
                }
            },
            _ => bail!("Year {} not found", year),
        }
    }

    Ok(())
}
