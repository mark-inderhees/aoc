// 2015 Day 6
// https://adventofcode.com/2015/day/6
// --- Day 6: Probably a Fire Hazard ---
// Turn on, off, or toggle lights based on commands for areas of lights

use anyhow::Result;

use crate::puzzle::Puzzle;

#[allow(unused_imports)]
use crate::utils::utils::*;

#[allow(unused_imports)]
use std::collections::VecDeque;

pub struct Day06 {
    instructions: Vec<Instruction>,

    /// Part 1, are lights on or off?
    grid: Vec<Vec<bool>>,

    /// Part 2, how bright are the lights?
    grid_brightness: Vec<Vec<u32>>,
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
            grid: vec![vec![false; 1000]; 1000],
            grid_brightness: vec![vec![0; 1000]; 1000],
        };

        for line in input.lines() {
            // Command is either loggle, turn on, or turn off
            let command = if line.starts_with("toggle") {
                Command::Toggle
            } else if line.starts_with("turn on") {
                Command::TurnOn
            } else if line.starts_with("turn off") {
                Command::TurnOff
            } else {
                panic!("Unexpected command")
            };

            // Get start and end coordinates for area that command applies to.
            // These coordinates are inclusive.
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
                    // Do work for part 1, turning lights on and off
                    let x_ = x as usize;
                    let y_ = y as usize;
                    let current_value = day.grid[x_][y_];
                    let new_value = match instruction.command {
                        Command::Toggle => !current_value,
                        Command::TurnOff => false,
                        Command::TurnOn => true,
                    };
                    day.grid[x_][y_] = new_value;

                    // Do work for part 2, changing brightness of lights
                    let current_brightness = day.grid_brightness[x_][y_];
                    let new_brightness = match instruction.command {
                        Command::Toggle => current_brightness + 2,
                        Command::TurnOff => current_brightness.checked_sub(1).unwrap_or(0),
                        Command::TurnOn => current_brightness + 1,
                    };
                    day.grid_brightness[x_][y_] = new_brightness;
                }
            }
        }

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        // Count how many lights are on
        let mut lights_on = 0;
        for x in 0..1000 {
            for y in 0..1000 {
                if self.grid[x][y] {
                    lights_on += 1;
                }
            }
        }
        Ok(lights_on.to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some((1_000_000 - 1000 - 4).to_string()),
            false => Some(400410.to_string()),
        }
    }

    fn solve_part2(&mut self) -> Result<String> {
        // Calculate total brightness
        let mut brightness = 0;
        for x in 0..1000 {
            for y in 0..1000 {
                brightness += self.grid_brightness[x][y];
            }
        }
        Ok(brightness.to_string())
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some((1_000_000 + 2000 - 4).to_string()),
            false => Some(15343601.to_string()),
        }
    }
}
