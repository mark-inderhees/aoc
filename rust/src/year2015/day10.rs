// 2015 Day 10
// https://adventofcode.com/2015/day/10
// --- Day 10: Elves Look, Elves Say ---

use anyhow::Result;

use crate::puzzle::Puzzle;

#[allow(unused_imports)]
use crate::utils::utils::*;

#[allow(unused_imports)]
use std::collections::VecDeque;

pub struct Day10 {
    input: String,
}

fn look_and_say(input: &str) -> String {
    log::debug!("Look and say for {input}");
    let len = input.chars().count();
    let mut output = String::with_capacity(len * 2);
    let mut i = 0;
    let chars: Vec<char> = input.chars().collect();
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
            count += 1;
            j += 1;
            i += 1;
        }

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
