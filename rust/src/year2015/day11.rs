// 2015 Day 11
// https://adventofcode.com/2015/day/11
// --- Day 11: Corporate Policy ---

use anyhow::Result;

use crate::puzzle::Puzzle;

#[allow(unused_imports)]
use crate::utils::utils::*;

#[allow(unused_imports)]
use std::collections::VecDeque;

pub struct Day11 {
    current_password: String,
}

/// Incrementing like ay -> az -> ba until valid password found
fn increment_password(current_password: &str) -> String {
    let mut ascii: Vec<u8> = current_password.chars().map(|char| char as u8).collect();
    let mut i = ascii.len() - 1;
    let max = 'z' as u8;
    let min = 'a' as u8;

    // Increment with roll over
    loop {
        let value = ascii[i] + 1;
        if value <= max {
            ascii[i] = value;
            break;
        }

        // Keep going so next char is incremented
        ascii[i] = min;
        i -= 1;
    }

    let output: String = ascii.iter().map(|value| *value as char).collect();
    log::debug!("Increment {current_password} to {output}");
    output
}

fn find_next_password(current_password: &str) -> String {
    // Keep incrementing like ay -> az -> ba until valid password found
    // Must have one three char straight like "bcd"
    // Cannont contain i, o, or l
    // Must contain two unique pairs, like aa and jj
    let mut password = current_password.to_string();

    loop {
        password = increment_password(&password);
        let chars: Vec<char> = password.chars().collect();

        // Must have one three char straight like "bcd"
        let mut has_straight = false;
        for i in 0..chars.len() - 2 {
            let char1 = chars[i] as u8;
            let char2 = chars[i + 1] as u8;
            let char3 = chars[i + 2] as u8;
            if char1 == char2 - 1 && char1 == char3 - 2 {
                log::trace!("{password} has straight {}", char1 as char);
                has_straight = true;
                break;
            }
        }
        if !has_straight {
            log::debug!("{password} has no straigt");
            continue;
        }

        // Cannont contain i, o, or l
        if chars.contains(&'i') || chars.contains(&'o') || chars.contains(&'l') {
            log::debug!("{password} contains i, o, or l");
            continue;
        }

        // Must contain two unique pairs, like aa and jj
        let mut pair_count = 0;
        let mut first_pair = ' ';
        for i in 0..chars.len() - 1 {
            let char = chars[i];

            // If we already found this pair, skip
            if char == first_pair {
                continue;
            }

            // Check if this is a double pair
            let next_char = chars[i + 1];
            if char == next_char {
                pair_count += 1;

                // Check if this is second pair
                if pair_count == 2 {
                    log::debug!("{password} contains double pairs {first_pair} and {char}");
                    break;
                }
                log::debug!("{password} found first pair {char}");
                first_pair = char;
            }
        }
        if pair_count != 2 {
            log::debug!("{password} has no double pair");
            continue;
        }

        break;
    }

    password
}

impl Puzzle for Day11 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut day = Day11 {
            current_password: input.trim().to_string(),
        };

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        let answer = find_next_password(&self.current_password);
        Ok(answer)
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some("abcdffaa".to_string()),
            false => Some("hxbxxyzz".to_string()),
        }
    }

    fn solve_part2(&mut self) -> Result<String> {
        let mut answer = find_next_password(&self.current_password);
        answer = find_next_password(&answer);
        Ok(answer)
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some("abcdffbb".to_string()),
            false => Some("hxcaabcc".to_string()),
        }
    }
}
