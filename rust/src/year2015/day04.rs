// 2015 Day 4
// https://adventofcode.com/2015/day/4
// --- Day 4: The Ideal Stocking Stuffer ---
// Crypto currency hash compute brute force.
// Note: the test input was slow, so I duplicated the real input to speed this up.

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

/// Find what needs to be appended to input to create hash with leading zeros.
/// To make this as fast as possible, reduce logic in the main loop like
/// allocations and formatting.
fn hash_for_zeros(input: &str, five_zeros: bool) -> u32 {
    // Look for 5 or 6 zeros
    let mask = match five_zeros {
        true => 0xF0,
        false => 0xFF,
    };

    // Find what number needs to be appended to input to give hash starting
    // with five or six zeros.
    let mut number = 0;

    // Allocation hash input once
    let mut hash_input = Vec::new();

    // Only write question input into hash once
    write!(hash_input, "{}", input).expect("Write failure");
    let len = hash_input.len();

    // Brute force, increment number and keep trying
    loop {
        // Append number to hash input
        if number % 10 == 0 {
            // Only run format logic on decimal roll over
            hash_input.truncate(len);
            write!(hash_input, "{}", number).expect("Write failure");
        } else {
            // Else, simply increment last value in vector
            *hash_input.last_mut().unwrap() += 1;
        }

        // Comput hash and check for 5 or 6 zeros
        let digest = md5::compute(hash_input.clone());
        if digest.0[0] == 0 && digest.0[1] == 0 && (digest.0[2] & mask) == 0 {
            break;
        }

        // Try again
        number += 1;
    }

    number
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
        // Look for 5 leading zeros
        let answer = hash_for_zeros(&self.input, true);
        Ok(answer.to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(254575.to_string()), // If using test input pqrstuv: Some(1048970.to_string()),
            false => Some(254575.to_string()),
        }
    }

    fn solve_part2(&mut self) -> Result<String> {
        // Look for 6 leading zeros
        let answer = hash_for_zeros(&self.input, false);
        Ok(answer.to_string())
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(1038736.to_string()), // If using test input pqrstuv: Some(5714438.to_string()),
            false => Some(1038736.to_string()),
        }
    }
}
