use anyhow::Result;

use crate::puzzle::Puzzle;

#[allow(unused_imports)]
use crate::utils::utils::*;

pub struct Day20 {
    values: Vec<i64>,
}

fn decode(values: &Vec<i64>, iterations: usize, magic: i64) -> Vec<i64> {
    // First, multiply by the magic
    let decrypt: Vec<i64> = values.iter().map(|r| *r as i64 * magic).collect();

    // Enumerate to keep the original index so numbers are uniquely searchable
    let mut decoded: Vec<(usize, &i64)> = decrypt.iter().enumerate().collect();

    // Run the requested iterations
    for _ in 0..iterations {
        // Loop over every value once
        for (original_index, value) in decrypt.iter().enumerate() {
            // Find where this value is in the currently list and remove it
            let current_index = decoded
                .iter()
                .position(|r| r == &(original_index, value))
                .unwrap();
            decoded.remove(current_index);

            // Use non negative remainder magic to find new index
            let mut new_index = value + current_index as i64;
            new_index = new_index.rem_euclid(decoded.len() as i64);
            if new_index == 0 {
                // Edge case, zero is actually end of list
                new_index = decoded.len() as i64;
            }
            decoded.insert(new_index as usize, (original_index, value));
            log::debug!("{decoded:?}");
        }
    }

    decoded.iter().map(|r| *r.1).collect::<Vec<i64>>()
}

fn get_answer(decoded: &Vec<i64>) -> i64 {
    let zero = decoded.iter().position(|&r| r == 0).unwrap();
    let len = decoded.len();
    let one = decoded[(zero + 1000) % len];
    let two = decoded[(zero + 2000) % len];
    let three = decoded[(zero + 3000) % len];
    one + two + three
}

impl Puzzle for Day20 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut day = Day20 { values: vec![] };

        for (i, line) in input.lines().enumerate() {
            day.values.push(get_val(line));
        }

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        let decoded = decode(&self.values, 1, 1);

        Ok(get_answer(&decoded).to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(3.to_string()),
            false => Some(6640.to_string()),
        }
    }

    fn solve_part2(&mut self) -> Result<String> {
        let decoded = decode(&self.values, 10, 811589153);

        Ok(get_answer(&decoded).to_string())
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(1623178306.to_string()),
            false => Some(11893839037215u64.to_string()),
        }
    }
}
