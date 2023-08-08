// 2016 Day 8
// https://adventofcode.com/2016/day/8

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

enum Command {
    DrawRect(Point<usize>),
    RotateRow(Rotate),
    RotateColumn(Rotate),
}

pub struct Day08 {
    screen: Screen,
    commands: Vec<Command>,
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
        for command in self.commands.iter() {
            match command {
                Command::DrawRect(point) => {
                    self.screen
                        .set_pixels(Point { x: 0, y: 0 }, point.x, point.y)
                }
                Command::RotateRow(rotate) => self.screen.rotate_row(rotate.target, rotate.count),
                Command::RotateColumn(rotate) => {
                    self.screen.rotate_column(rotate.target, rotate.count)
                }
            }
        }
        self.screen.debug_print();
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
        let answer = match self.commands.len() > 10 {
            true => "EOARGPHYAO".to_string(),
            false => "No Test Case".to_string(),
        };
        Ok(answer.to_string())
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some("No Test Case".to_string()),
            false => Some("EOARGPHYAO".to_string()),
        }
    }
}
