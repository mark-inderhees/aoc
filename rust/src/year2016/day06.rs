// 2016 Day 6
// https://adventofcode.com/2016/day/6

use anyhow::Result;

use crate::puzzle::Puzzle;

#[allow(unused_imports)]
use crate::utils::utils::*;

#[allow(unused_imports)]
use std::collections::VecDeque;

pub struct Day06 {
    messages: Vec<String>,
}

impl Puzzle for Day06 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut day = Day06 { messages: vec![] };

        for line in input.trim().split("\n") {
            day.messages.push(line.to_string());
        }

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        let mut answer = String::new();
        for i in 0..self.messages[0].len() {
            let mut letter_count = vec![0; 26];
            for message in &self.messages {
                let index = message.chars().nth(i).unwrap() as usize - 'a' as usize;
                letter_count[index] += 1;
            }
            let mut max_count = 0;
            let mut max_index = 0;
            for (index, count) in letter_count.iter().enumerate() {
                if *count > max_count {
                    max_count = *count;
                    max_index = index;
                }
            }
            let char = char::from_u32(max_index as u32 + 'a' as u32).unwrap();
            answer += &char.to_string();
        }

        Ok(answer)
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some("easter".to_string()),
            false => Some("cyxeoccr".to_string()),
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
