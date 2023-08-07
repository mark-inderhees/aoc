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
    part1: String,
    part2: String,
}

/// Find what needs to be appended to input to create hash with leading zeros.
/// To make this as fast as possible, reduce logic in the main loop like
/// allocations and formatting. Iterations means find the nth iteration.
fn hash_for_zeros(input: &str, five_zeros: bool, iterations: u32) -> (String, String) {
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
    let mut answer2 = vec!["".to_string(); 8];
    let mut answer2_map = vec![false; 8];

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
            let digit = digest.0[2];
            let s = format!("{:x}", digit);
            if iteration <= iterations {
                answer += &s;
            }
            log::debug!(
                "MARK!! {:?} {} {:?} {} {} {}",
                hash_input,
                number,
                digest,
                digest.0[2],
                s,
                digest.0[3],
            );

            if digit < 8 {
                if !answer2_map[digit as usize] {
                    let digit2 = digest.0[3] >> 4;
                    let s = format!("{:x}", digit2);
                    answer2[digit as usize] = s;
                    answer2_map[digit as usize] = true;
                }
            }

            if answer2_map.iter().all(|&x| x) {
                break;
            }
        }

        // Try again
        number += 1;
    }

    (answer, answer2.join(""))
}

impl Puzzle for Day05 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let (part1, part2) = hash_for_zeros(input.trim(), true, 8);
        let mut day = Day05 { part1, part2 };

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        Ok(self.part1.to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some("18f47a30".to_string()),
            false => Some("2414bc77".to_string()),
        }
    }

    fn solve_part2(&mut self) -> Result<String> {
        Ok(self.part2.to_string())
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some("05ace8e3".to_string()),
            false => Some("437e60fc".to_string()),
        }
    }
}
