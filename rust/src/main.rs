use anyhow::{bail, Result};
use clap::Parser;
use puzzle::Puzzle;
use regex::Regex;
use std::collections::VecDeque;
use std::fs;
use std::path::PathBuf;
use std::time::Instant;

mod puzzle;
mod utils;
mod year2015;
mod year2016;
mod year2022;
mod year2024;

/// Runner for Advent of Code
#[derive(Parser, Debug)]
struct Args {
    /// Which day's code to run
    #[arg(
        long,
        short,
        default_value_t = 1, // __BOOTSTRAP_DAY__
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
    #[arg(long, short, default_value_t = 2024)]
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

    /// Validate all parts and input for all days and years
    #[arg(long, short)]
    exhaustive: bool,

    /// Bootstrap a new day
    #[arg(long, short, value_name = "DAY")]
    bootstrap: Option<u32>,

    /// Set the log level
    #[arg(
        long,
        short,
        value_name = "LEVEL",
        default_value_t = log::LevelFilter::Warn,
    )]
    logs: log::LevelFilter,

    /// Save a comma seperated value (CSV) file with perf to a file
    #[arg(long, short)]
    sheet: Option<PathBuf>,
}

fn run_day<DayType: puzzle::Puzzle>(
    part: u32,
    input: String,
    test: bool,
    duration: &mut f64,
) -> Result<()> {
    let start = Instant::now();
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
    *duration = start.elapsed().as_millis() as f64 / 1000f64;
    match expect {
        Some(expected_val) => {
            println!(
                "Solution: {} == {} is {} in {:.3} seconds\n\n",
                output,
                expected_val,
                output == expected_val,
                duration
            );
            assert_eq!(output, expected_val);
        }
        _ => println!("Solution: {} in {:.3} seconds\n\n", output, duration),
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
    let mut contents = fs::read_to_string(day_rs)?;
    contents = contents.replace("DayXX", &format!("Day{day:02}"));

    // Fix header in dayXX.rs
    let url = format!("// https://adventofcode.com/{year}/day/{day}");
    let header = format!("// {year} Day {day}");
    let mut lines: VecDeque<&str> = contents.lines().collect();
    lines.pop_front();
    lines.pop_front();
    lines.push_front(&url);
    lines.push_front(&header);
    let lines: Vec<&str> = lines.into();
    fs::write(day_rs, lines.join("\n"))?;

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
    main = re_run.replace(&main, format!("${{1}}{day} => run_day::<year{year}::day{day:02}::Day{day:02}>(part, input, test, &mut duration)?,\n${{1}}${{2}}")).to_string();
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
        .format(|_, record| {
            println!("{}", record.args());
            Ok(())
        })
        .init();

    match args.bootstrap {
        Some(day) => {
            bootstrap(day, args.year)?;
            return Ok(());
        }
        None => (),
    };

    let mut years = vec![args.year];
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
    } else if args.exhaustive {
        days = (1..=25).collect();
        parts = (1..=2).collect();
        tests = vec![true, false];
        years = vec![2015, 2016, 2022];
    }

    struct Run {
        day: u32,
        year: u32,
        part: u32,
        test: bool,
        duration: f64,
    }

    let mut runs = vec![];
    {
        for year in &years {
            for day in &days {
                for part in &parts {
                    for test in &tests {
                        runs.push(Run {
                            day: *day,
                            year: *year,
                            part: *part,
                            test: *test,
                            duration: 0f64,
                        });
                    }
                }
            }
        }
    }

    println!("\n"); // Empty line
    let start = Instant::now();
    let mut year_filter = vec![];
    for run in runs.iter_mut() {
        let day = run.day;
        let year = run.year;
        let part = run.part;
        let test = run.test;
        let mut duration = 0f64;

        if year_filter.contains(&year) {
            continue;
        }

        println!("Running {year} day={day} part={part} test={test} ...");
        let input_type = match test {
            true => "test",
            false => "input",
        };
        let input = format!("input/{year}/day{day:02}.{input_type}");

        match year {
            2015 => match day {
                1 => run_day::<year2015::day01::Day01>(part, input, test, &mut duration)?,
                2 => run_day::<year2015::day02::Day02>(part, input, test, &mut duration)?,
                3 => run_day::<year2015::day03::Day03>(part, input, test, &mut duration)?,
                4 => run_day::<year2015::day04::Day04>(part, input, test, &mut duration)?,
                5 => run_day::<year2015::day05::Day05>(part, input, test, &mut duration)?,
                6 => run_day::<year2015::day06::Day06>(part, input, test, &mut duration)?,
                7 => run_day::<year2015::day07::Day07>(part, input, test, &mut duration)?,
                8 => run_day::<year2015::day08::Day08>(part, input, test, &mut duration)?,
                9 => run_day::<year2015::day09::Day09>(part, input, test, &mut duration)?,
                10 => run_day::<year2015::day10::Day10>(part, input, test, &mut duration)?,
                11 => run_day::<year2015::day11::Day11>(part, input, test, &mut duration)?,
                12 => run_day::<year2015::day12::Day12>(part, input, test, &mut duration)?,
                13 => run_day::<year2015::day13::Day13>(part, input, test, &mut duration)?,
                14 => run_day::<year2015::day14::Day14>(part, input, test, &mut duration)?,
                15 => run_day::<year2015::day15::Day15>(part, input, test, &mut duration)?,
                16 => run_day::<year2015::day16::Day16>(part, input, test, &mut duration)?,
                17 => run_day::<year2015::day17::Day17>(part, input, test, &mut duration)?,
                18 => run_day::<year2015::day18::Day18>(part, input, test, &mut duration)?,
                19 => run_day::<year2015::day19::Day19>(part, input, test, &mut duration)?,
                20 => run_day::<year2015::day20::Day20>(part, input, test, &mut duration)?,
                21 => run_day::<year2015::day21::Day21>(part, input, test, &mut duration)?,
                22 => run_day::<year2015::day22::Day22>(part, input, test, &mut duration)?,
                23 => run_day::<year2015::day23::Day23>(part, input, test, &mut duration)?,
                24 => run_day::<year2015::day24::Day24>(part, input, test, &mut duration)?,
                25 => run_day::<year2015::day25::Day25>(part, input, test, &mut duration)?,
                _ => {
                    println!("Day {} not found, goodbye!\n", day);
                    year_filter.push(year);
                }
            },
            2016 => match day {
                1 => run_day::<year2016::day01::Day01>(part, input, test, &mut duration)?,
                2 => run_day::<year2016::day02::Day02>(part, input, test, &mut duration)?,
                3 => run_day::<year2016::day03::Day03>(part, input, test, &mut duration)?,
                4 => run_day::<year2016::day04::Day04>(part, input, test, &mut duration)?,
                5 => run_day::<year2016::day05::Day05>(part, input, test, &mut duration)?,
                6 => run_day::<year2016::day06::Day06>(part, input, test, &mut duration)?,
                7 => run_day::<year2016::day07::Day07>(part, input, test, &mut duration)?,
                8 => run_day::<year2016::day08::Day08>(part, input, test, &mut duration)?,
                9 => run_day::<year2016::day09::Day09>(part, input, test, &mut duration)?,
                10 => run_day::<year2016::day10::Day10>(part, input, test, &mut duration)?,
                _ => {
                    println!("Day {} not found, goodbye!\n", day);
                    year_filter.push(year);
                }
            },
            2022 => match day {
                1 => run_day::<year2022::day01::Day01>(part, input, test, &mut duration)?,
                2 => run_day::<year2022::day02::Day02>(part, input, test, &mut duration)?,
                3 => run_day::<year2022::day03::Day03>(part, input, test, &mut duration)?,
                4 => run_day::<year2022::day04::Day04>(part, input, test, &mut duration)?,
                5 => run_day::<year2022::day05::Day05>(part, input, test, &mut duration)?,
                6 => run_day::<year2022::day06::Day06>(part, input, test, &mut duration)?,
                7 => run_day::<year2022::day07::Day07>(part, input, test, &mut duration)?,
                8 => run_day::<year2022::day08::Day08>(part, input, test, &mut duration)?,
                9 => run_day::<year2022::day09::Day09>(part, input, test, &mut duration)?,
                10 => run_day::<year2022::day10::Day10>(part, input, test, &mut duration)?,
                11 => run_day::<year2022::day11::Day11>(part, input, test, &mut duration)?,
                12 => run_day::<year2022::day12::Day12>(part, input, test, &mut duration)?,
                13 => run_day::<year2022::day13::Day13>(part, input, test, &mut duration)?,
                14 => run_day::<year2022::day14::Day14>(part, input, test, &mut duration)?,
                15 => run_day::<year2022::day15::Day15>(part, input, test, &mut duration)?,
                16 => run_day::<year2022::day16::Day16>(part, input, test, &mut duration)?,
                17 => run_day::<year2022::day17::Day17>(part, input, test, &mut duration)?,
                18 => run_day::<year2022::day18::Day18>(part, input, test, &mut duration)?,
                19 => run_day::<year2022::day19::Day19>(part, input, test, &mut duration)?,
                20 => run_day::<year2022::day20::Day20>(part, input, test, &mut duration)?,
                21 => run_day::<year2022::day21::Day21>(part, input, test, &mut duration)?,
                22 => run_day::<year2022::day22::Day22>(part, input, test, &mut duration)?,
                23 => run_day::<year2022::day23::Day23>(part, input, test, &mut duration)?,
                24 => run_day::<year2022::day24::Day24>(part, input, test, &mut duration)?,
                25 => run_day::<year2022::day25::Day25>(part, input, test, &mut duration)?,
                _ => {
                    println!("Day {} not found, goodbye!\n", day);
                    year_filter.push(year);
                }
            },
            2024 => match day {
                1 => run_day::<year2024::day01::Day01>(part, input, test, &mut duration)?,
                // __BOOTSTRAP_RUN__
                _ => {
                    println!("Day {} not found, goodbye!\n", day);
                    year_filter.push(year);
                }
            },
            _ => bail!("Year {} not found", year),
        }
        run.duration = duration;
    }
    if args.validate || args.comprehensive || args.exhaustive {
        println!(
            "Full run took {:.3} seconds",
            start.elapsed().as_millis() as f64 / 1000f64
        );
    }

    if let Some(csv_path) = args.sheet {
        let mut csv = vec![];
        for run in runs {
            csv.push(format!(
                "{},{},{},{},{}",
                run.year, run.day, run.part, run.test, run.duration
            ));
        }
        println!("Wrote perf log to {}", csv_path.to_str().unwrap());
        fs::write(csv_path, csv.join("\n"))?;
    }

    Ok(())
}
