// 2015 Day 1
// https://adventofcode.com/2015/day/1
// --- Day 1: Not Quite Lisp ---
// Santa rides an elevator

use anyhow::Result;

use crate::puzzle::Puzzle;

#[allow(unused_imports)]
use crate::utils::utils::*;

#[allow(unused_imports)]
use std::collections::VecDeque;

enum Elevator {
    Up,
    Down,
}

pub struct Day01 {
    directions: Vec<Elevator>,
}

impl Puzzle for Day01 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut day = Day01 { directions: vec![] };

        for line in input.lines() {
            for char in line.chars() {
                day.directions.push(match char {
                    '(' => Elevator::Up,
                    ')' => Elevator::Down,
                    _ => panic!("Unexpected char"),
                });
            }
        }

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        // Simply calculate the final floor
        let mut floor: i32 = 0;
        for direction in self.directions.iter() {
            match direction {
                Elevator::Up => floor += 1,
                Elevator::Down => floor -= 1,
            }
        }
        Ok(floor.to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some((-1).to_string()),
            false => Some(232.to_string()),
        }
    }

    fn solve_part2(&mut self) -> Result<String> {
        // Find which index of the input results in floor -1
        let mut floor: i32 = 0;
        let mut index = 0;
        for direction in self.directions.iter() {
            match direction {
                Elevator::Up => floor += 1,
                Elevator::Down => floor -= 1,
            }

            index += 1;
            if floor == -1 {
                break;
            }
        }

        Ok(index.to_string())
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(5.to_string()),
            false => Some(1783.to_string()),
        }
    }
}
