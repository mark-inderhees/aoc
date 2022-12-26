// 2022 Day 1
// https://adventofcode.com/2022/day/1
// --- Day 1: Calorie Counting ---
// Do some simple sums of inputs

use anyhow::Result;

use crate::puzzle::Puzzle;
use crate::utils::utils::*;

pub struct Day01 {
    elves: Vec<u32>,
}

impl Puzzle for Day01 {
    fn from_input(input: &str) -> Result<Self> {
        let mut day = Day01 { elves: vec![] };

        // Inputs are simple numbers split by whitespace for each elf
        for elf in input.split("\n\n") {
            let calories: u32 = elf.lines().map(|calorie| find_val::<u32>(calorie)).sum();
            day.elves.push(calories);
        }

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        // Find elf with most calories
        Ok(self.elves.iter().max().expect("Max error").to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(24000.to_string()),
            false => Some(68787.to_string()),
        }
    }

    fn solve_part2(&mut self) -> Result<String> {
        // Find sum of 3 elves with most calories
        self.elves.sort();
        self.elves.reverse();
        Ok(self.elves[0..3].iter().sum::<u32>().to_string())
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(45000.to_string()),
            false => Some(198041.to_string()),
        }
    }
}
