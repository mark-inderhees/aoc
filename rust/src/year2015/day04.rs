// 2015 Day 4
// https://adventofcode.com/2015/day/4
// --- Day 4: The Ideal Stocking Stuffer ---

use anyhow::Result;
use md5;
use std::io::Write;

use crate::puzzle::Puzzle;

#[allow(unused_imports)]
use crate::utils::utils::*;

#[allow(unused_imports)]
use std::collections::VecDeque;

pub struct Day04 {
    input: String,
}

impl Puzzle for Day04 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut day = Day04 {
            input: input.lines().collect::<Vec<&str>>()[0].to_string(),
        };

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        let mut answer = 0;
        loop {
            let md5_input = format!("{}{}", self.input, answer);
            let digest = md5::compute(md5_input);
            let hash = format!("{:x}", digest);
            if hash.starts_with("00000") {
                break;
            }
            answer += 1;
        }
        Ok(answer.to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(1048970.to_string()),
            false => Some(254575.to_string()),
        }
    }

    fn solve_part2(&mut self) -> Result<String> {
        // Find what number needs to be appended to input to give hash starting with six zeros
        let mut number = 0;

        // Allocation hash input once
        let mut hash_input = Vec::new();

        // Only write question input into hash once
        write!(hash_input, "{}", self.input)?;
        let len = hash_input.len();

        // Brute force, increment number and keep trying
        loop {
            // Append number to hash input
            write!(hash_input, "{}", number)?;

            // Comput hash and check for 6 zeros
            let digest = md5::compute(hash_input.clone());
            if digest.0[0] == 0 && digest.0[1] == 0 && digest.0[2] == 0 {
                break;
            }

            // Try again, drop the number we appended and increment
            hash_input.truncate(len);
            number += 1;
        }
        Ok(number.to_string())
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(5714438.to_string()),
            false => Some(1038736.to_string()),
        }
    }
}
