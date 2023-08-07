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

// Given a list of messages, find either the most common or least common char
// at each index, then put those together into the answer.
fn decipher(messages: &Vec<String>, most_common: bool) -> String {
    let mut answer = String::new();

    // Loop through each char in each message
    for i in 0..messages[0].len() {
        // Find the count of each letter at this char index
        let mut letter_count = vec![0; 26];
        for message in messages {
            let index = message.chars().nth(i).unwrap() as usize - 'a' as usize;
            letter_count[index] += 1;
        }

        // Find max and min count char at this index
        let mut max_count = 0;
        let mut max_index = 0;
        let mut min_count = u32::MAX;
        let mut min_index = 0;
        for (index, count) in letter_count.iter().enumerate() {
            if *count > max_count {
                max_count = *count;
                max_index = index;
            }

            if *count < min_count && *count != 0 {
                min_count = *count;
                min_index = index;
            }
        }
        let index = match most_common {
            true => max_index,
            false => min_index,
        };
        let char = char::from_u32(index as u32 + 'a' as u32).unwrap();
        answer += &char.to_string();
    }

    answer
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
        let answer = decipher(&self.messages, true);
        Ok(answer)
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some("easter".to_string()),
            false => Some("cyxeoccr".to_string()),
        }
    }

    fn solve_part2(&mut self) -> Result<String> {
        let answer = decipher(&self.messages, false);
        Ok(answer)
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some("advent".to_string()),
            false => Some("batwpask".to_string()),
        }
    }
}
