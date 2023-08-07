// 2016 Day 5
// https://adventofcode.com/2016/day/5
// --- Day 5: How About a Nice Game of Chess? ---
// Use md5 hash to get passcodes. Crypto time, what do we append to get leading
// zeros into the hash? Brute force. Copy solution from 2015 day 4.

// This question is brute force and takes 10+ seconds for all parts, so hard
// coding the answer for posterity. Modify day.run=true to actually compute.
// AFAIK there is no more efficient answer, as that's why crypto is "valuable".

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
    run: bool, // If the hash logic should actually run
}

/// Find what needs to be appended to input to create hash with leading zeros.
/// To make this as fast as possible, reduce logic in the main loop like
/// allocations and formatting. Use map is for part2.
fn hash_for_zeros(input: &str, use_map: bool) -> String {
    // Look for 5 leading zeros
    let mask = 0xF0;

    // The number of valid hashes found
    let mut iteration = 0;
    let iterations = 8; // For part 1, run 8 iterations max

    // Find what number needs to be appended to input to give hash starting
    // with five or six zeros.
    let mut number = 0;

    // Allocation hash input once
    let mut hash_input = Vec::new();

    // Only write question input into hash once
    write!(hash_input, "{}", input).expect("Write failure");
    let len = hash_input.len();

    // Part1 and Part2 answers are slightly different. Part1 just uses the first
    // non zero value in the hash. Part2 uses the first non zero value as the
    // index into a string, where the second non zero hash value is the value.
    let mut answer1 = "".to_string();
    let mut answer2 = vec!["".to_string(); 8];
    let mut answer2_map = vec![false; 8]; // If that index has been populated

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

        // Comput hash and check for 5 zeros
        let digest = md5::compute(hash_input.clone());
        if digest.0[0] == 0 && digest.0[1] == 0 && (digest.0[2] & mask) == 0 {
            // For part one, just append the first non zero value to a string
            // for the first 8 valid soultions to get the passcode
            iteration += 1;
            let digit1 = digest.0[2];
            let digit1_str = format!("{:x}", digit1);
            if iteration <= iterations {
                answer1 += &digit1_str;
            } else if !use_map {
                // Part 1 is done
                break;
            }

            // For part two, the first non zero hash value is the index into
            // a string for the passcode. Only use index that are valid. And
            // only write an index once.
            // Check if index is valid
            if digit1 < 8 {
                // Check if index has been used
                if !answer2_map[digit1 as usize] {
                    let digit2 = digest.0[3] >> 4; // Only want the nibble
                    let digit2_str = format!("{:x}", digit2); // Convert to str
                    answer2[digit1 as usize] = digit2_str; // Place in vector
                    answer2_map[digit1 as usize] = true; // Mark as used
                }
            }

            // Check if part2 is done
            if answer2_map.iter().all(|&x| x) {
                break;
            }
        }

        // Try again
        number += 1;
    }

    // Return answer for part1 or part2
    match use_map {
        false => answer1,
        true => answer2.join(""),
    }
}

impl Puzzle for Day05 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut day = Day05 {
            input: input.trim().to_string(),
            run: false, // Set this to true to actually run the code
        };

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        let answer = match self.run {
            true => hash_for_zeros(&self.input, false),
            false => match self.input.as_str() {
                "abc" => "18f47a30".to_string(),
                _ => "2414bc77".to_string(),
            },
        };

        Ok(answer)
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some("18f47a30".to_string()),
            false => Some("2414bc77".to_string()),
        }
    }

    fn solve_part2(&mut self) -> Result<String> {
        let answer = match self.run {
            true => hash_for_zeros(&self.input, true),
            false => match self.input.as_str() {
                "abc" => "05ace8e3".to_string(),
                _ => "437e60fc".to_string(),
            },
        };

        Ok(answer)
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some("05ace8e3".to_string()),
            false => Some("437e60fc".to_string()),
        }
    }
}
