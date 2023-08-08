// 2016 Day 8
// https://adventofcode.com/2016/day/8
// --- Day 8: Two-Factor Authentication ---
// Run some simple commands to display pixels on a screen

use anyhow::Result;
use regex::Regex;
use rusttype::Point;

use crate::{puzzle::Puzzle, utils::screen::Screen};

#[allow(unused_imports)]
use crate::utils::utils::*;

#[allow(unused_imports)]
use std::collections::VecDeque;

struct Rotate {
    target: usize,
    count: usize,
}

// There are just three commands
enum Command {
    DrawRect(Point<usize>),
    RotateRow(Rotate),
    RotateColumn(Rotate),
}

pub struct Day08 {
    screen: Screen,
    commands: Vec<Command>,
}

fn run_commands(commands: &Vec<Command>, screen: &mut Screen) {
    for command in commands.iter() {
        // Run the commands on the screen
        match command {
            Command::DrawRect(point) => screen.set_pixels(Point { x: 0, y: 0 }, point.x, point.y),
            Command::RotateRow(rotate) => screen.rotate_row(rotate.target, rotate.count),
            Command::RotateColumn(rotate) => screen.rotate_column(rotate.target, rotate.count),
        }
    }
}

impl Puzzle for Day08 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut day = Day08 {
            screen: Screen::new(50, 6),
            commands: vec![],
        };

        for line in input.lines() {
            log::debug!("Line {}", line);
            // Parse the three command types to command enum
            if line.starts_with("rect") {
                let re = Regex::new(r"rect (\d*)x(\d*)").unwrap();
                let matches = re.captures(line).unwrap();
                let point = Point {
                    x: find_val(matches.get(1).unwrap().as_str()),
                    y: find_val(matches.get(2).unwrap().as_str()),
                };
                day.commands.push(Command::DrawRect(point));
            } else if line.starts_with("rotate row") {
                let re = Regex::new(r"rotate row y=(\d*) by (\d*)").unwrap();
                let matches = re.captures(line).unwrap();
                let rotate = Rotate {
                    target: find_val(matches.get(1).unwrap().as_str()),
                    count: find_val(matches.get(2).unwrap().as_str()),
                };
                day.commands.push(Command::RotateRow(rotate));
            } else if line.starts_with("rotate column") {
                let re = Regex::new(r"rotate column x=(\d*) by (\d*)").unwrap();
                let matches = re.captures(line).unwrap();
                let rotate = Rotate {
                    target: find_val(matches.get(1).unwrap().as_str()),
                    count: find_val(matches.get(2).unwrap().as_str()),
                };
                day.commands.push(Command::RotateColumn(rotate));
            }
        }

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        // Find out how many pixels are set after all commands are run
        run_commands(&self.commands, &mut self.screen);
        let answer = self.screen.count_set_pixels();
        Ok(answer.to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(6.to_string()),
            false => Some(128.to_string()),
        }
    }

    fn solve_part2(&mut self) -> Result<String> {
        // Look at final screen debug print from part 1 and physically read in the display
        run_commands(&self.commands, &mut self.screen);
        let answer = self.screen.to_string();
        Ok(answer.to_string())
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            // Test data does not actually display stuff
            true => Some("\n.#..#.#...........................................\n#.#...............................................\n..................................................\n..................................................\n..................................................\n.#................................................\n".to_string()),
            // Real data reads like EOARGPHYAO
            false => Some("\n####..##...##..###...##..###..#..#.#...#.##...##..\n#....#..#.#..#.#..#.#..#.#..#.#..#.#...##..#.#..#.\n###..#..#.#..#.#..#.#....#..#.####..#.#.#..#.#..#.\n#....#..#.####.###..#.##.###..#..#...#..####.#..#.\n#....#..#.#..#.#.#..#..#.#....#..#...#..#..#.#..#.\n####..##..#..#.#..#..###.#....#..#...#..#..#..##..\n".to_string()),
        }
    }
}
