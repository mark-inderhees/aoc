// 2016 Day 5
// https://adventofcode.com/2016/day/5

use anyhow::Result;
use md5;
use std::io::Write;

use crate::puzzle::Puzzle;

#[allow(unused_imports)]
use crate::utils::utils::*;

#[allow(unused_imports)]
use std::collections::VecDeque;

pub struct Day05 {
    input: String,
}

/// Find what needs to be appended to input to create hash with leading zeros.
/// To make this as fast as possible, reduce logic in the main loop like
/// allocations and formatting. Iterations means find the nth iteration.
fn hash_for_zeros(input: &str, five_zeros: bool, iterations: u32) -> String {
    // Look for 5 or 6 zeros
    let mask = match five_zeros {
        true => 0xF0,
        false => 0xFF,
    };

    // The number of valid hashes found
    let mut iteration = 0;

    // Find what number needs to be appended to input to give hash starting
    // with five or six zeros.
    let mut number = 0;

    // Allocation hash input once
    let mut hash_input = Vec::new();

    // Only write question input into hash once
    write!(hash_input, "{}", input).expect("Write failure");
    let len = hash_input.len();

    let mut answer = "".to_string();

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
            iteration += 1;
            let s = format!("{:x}", digest.0[2]);
            answer += &s;
            log::debug!(
                "MARK!! {:?} {} {:?} {} {}",
                hash_input,
                number,
                digest,
                digest.0[2],
                s
            );
            if iteration == iterations {
                break;
            }
        }

        // Try again
        number += 1;
    }

    answer
}

impl Puzzle for Day05 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut day = Day05 {
            input: input.trim().to_string(),
        };

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        let bob = hash_for_zeros(&self.input, true, 8);

        Ok(bob.to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some("18f47a30".to_string()),
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
