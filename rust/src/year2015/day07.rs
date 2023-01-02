// 2015 Day 7
// https://adventofcode.com/2015/day/7
// --- Day 7: Some Assembly Required ---

use anyhow::Result;

use crate::puzzle::Puzzle;

#[allow(unused_imports)]
use crate::utils::utils::*;

#[allow(unused_imports)]
use std::collections::VecDeque;

pub struct Day07 {
    operations: Vec<Operation>,
}

enum Operators {
    Set(u16),
    And,
    Or,
    Not,
    LShift(u16),
    RShift(u16),
}

struct Operation {
    name: String,
    result: u16,
    operator: Operators,
    lhs: String,
    rhs: String,
}

impl Puzzle for Day07 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut day = Day07 { operations: vec![] };

        for line in input.lines() {
            let mut operator =if line.contains("AND") {
                Operators::And
            }
            else if line.contains("OR") {
                Operators::Or}
                else if line.contains("Not") {
                    Operators::Not
            } else if line.contains("LSHIFT") {
                Operators::LShift(find_val(line))
            } else if line.contains("RSHIFT"){
                Operators::RShift(find_val(line))
            } else {
                Operators::Set(find_val(line))
            };
        }

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        Ok("to do".to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => None,
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
