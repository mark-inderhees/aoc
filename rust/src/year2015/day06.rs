// 2015 Day 6
// https://adventofcode.com/2015/day/6
// --- Day 6: Probably a Fire Hazard ---

use anyhow::Result;

use crate::puzzle::Puzzle;

#[allow(unused_imports)]
use crate::utils::utils::*;

use std::collections::HashMap;
#[allow(unused_imports)]
use std::collections::VecDeque;

pub struct Day06 {
    instructions: Vec<Instruction>,
    grid: HashMap<UtilsPoint, bool>,
}

enum Command {
    Toggle,
    TurnOn,
    TurnOff,
}

struct Instruction {
    command: Command,
    start: UtilsPoint,
    end: UtilsPoint,
}

impl Puzzle for Day06 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut day = Day06 {
            instructions: vec![],
            grid: HashMap::new(),
        };

        for line in input.lines() {
            let command = if line.starts_with("toggle") {
                Command::Toggle
            } else if line.starts_with("turn on") {
                Command::TurnOn
            } else if line.starts_with("turn off") {
                Command::TurnOff
            } else {
                panic!("Unexpected command")
            };

            let vals: Vec<i32> = find_vals(line);
            let start = UtilsPoint {
                x: vals[0],
                y: vals[1],
            };
            let end = UtilsPoint {
                x: vals[2],
                y: vals[3],
            };
            day.instructions.push(Instruction {
                command,
                start,
                end,
            });
        }

        for instruction in day.instructions.iter() {
            for x in instruction.start.x..=instruction.end.x {
                for y in instruction.start.y..=instruction.end.y {
                    let point = UtilsPoint { x, y };
                    let current_value = day.grid.get(&point).unwrap_or(&false);
                    let new_value = match instruction.command {
                        Command::Toggle => !current_value,
                        Command::TurnOff => false,
                        Command::TurnOn => true,
                    };
                    day.grid.insert(point, new_value);
                }
            }
        }

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        let mut lights_on = 0;
        for x in 0..1000 {
            for y in 0..1000 {
                let point = UtilsPoint { x, y };
                let current_value = self.grid.get(&point).unwrap_or(&false);
                if *current_value {
                    lights_on += 1;
                }
            }
        }
        Ok(lights_on.to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some((1_000_000 - 1000 - 4).to_string()),
            false => None,
        }
    }

    fn solve_part2(&mut self) -> Result<String> {
        Ok("to do".to_string())
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => None,
            false => None,
        }
    }
}
