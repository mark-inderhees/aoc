// 2015 Day 10
// https://adventofcode.com/2015/day/10
// --- Day 10: Elves Look, Elves Say ---
// Brute force look and say game

use anyhow::Result;

use crate::puzzle::Puzzle;

#[allow(unused_imports)]
use crate::utils::utils::*;

#[allow(unused_imports)]
use std::collections::VecDeque;

pub struct Day10 {
    input: String,
}

// For a given input, play one round of look and say and give output
// https://en.wikipedia.org/wiki/Look-and-say_sequence
fn look_and_say(input: &str) -> String {
    log::debug!("Look and say for {input}");
    let chars: Vec<char> = input.chars().collect();
    let len = chars.len();
    let mut output = String::with_capacity(len * 2); // Give size so no reallocs
    let mut i = 0;
    while i < len {
        let char = chars[i];

        // Count how many of this char there are
        let mut count = 1;
        let mut j = i + 1;
        while j < len {
            let next_char = chars[j];
            if next_char != char {
                break;
            }
            // Found a matching char, increment counts
            count += 1;
            j += 1;
            i += 1;
        }

        // Done with this look and say section, build the output string
        log::debug!("There are {count} of {char}");
        output.push_str(&count.to_string());
        output.push(char);
        i += 1;
    }

    log::debug!("Look and say for {input} became {output}");
    output
}

impl Puzzle for Day10 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut day = Day10 {
            input: input.trim().to_string(),
        };

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        // Run it 40 times
        let mut input = self.input.clone();
        for i in 0..40 {
            input = look_and_say(&input);
            log::info!("Round {i} is len {}", input.chars().count());
        }
        let answer = input.chars().count();
        Ok(answer.to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(82350.to_string()),
            false => Some(492982.to_string()),
        }
    }

    fn solve_part2(&mut self) -> Result<String> {
        // Run it 50 times
        let mut input = self.input.clone();
        for i in 0..50 {
            input = look_and_say(&input);
            log::info!("Round {i} is len {}", input.chars().count());
        }
        let answer = input.chars().count();
        Ok(answer.to_string())
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(1166642.to_string()),
            false => Some(6989950.to_string()),
        }
    }
}
