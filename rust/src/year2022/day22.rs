use anyhow::Result;

use crate::puzzle::Puzzle;

#[allow(unused_imports)]
use crate::utils::utils::*;

#[allow(unused_imports)]
use std::collections::VecDeque;

pub struct Day22 {}

impl Puzzle for Day22 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut day = Day22 {};

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
