// 2022 Day 3
// https://adventofcode.com/2022/day/3
// --- Day 3: Rucksack Reorganization ---
// Find the duplicate char in a string

use anyhow::Result;

use crate::puzzle::Puzzle;
use crate::utils::utils::*;

pub struct Day03 {
    rucksacks: Vec<(String, String)>,
    groups: Vec<(String, String, String)>,
}

/// Get value of a-Z chars
fn find_char_score(c: &char) -> u32 {
    let value = match c {
        'a'..='z' => (*c as u32) - 96,
        _ => (*c as u32) - 64 + 26, // Uppercase are worth more than lower case
    };
    value
}

impl Puzzle for Day03 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut day = Day03 {
            rucksacks: vec![],
            groups: vec![],
        };

        for line in input.lines() {
            // Each input rucksack is split in two
            let len = line.len() / 2;
            let compartment1 = &line[..len];
            let compartment2 = &line[len..];
            day.rucksacks
                .push((compartment1.to_string(), compartment2.to_string()));
        }

        // Or the lines are groups of 3
        let mut lines = input.lines();
        while let (Some(a), Some(b), Some(c)) = (lines.next(), lines.next(), lines.next()) {
            day.groups
                .push((a.to_string(), b.to_string(), c.to_string()));
        }

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        let mut score = 0;
        // Find the char in both compartments of rucksack. Get the score. Sum the scores.
        for (a, b) in self.rucksacks.iter() {
            for c in a.chars() {
                if char_in_string(&c, b) {
                    score += find_char_score(&c);
                    break;
                }
            }
        }

        Ok(score.to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(157.to_string()),
            false => Some(7674.to_string()),
        }
    }

    fn solve_part2(&mut self) -> Result<String> {
        let mut score = 0;
        // Find the char in each of the group of 3.
        // Get the char value and sume the values.
        for (a, b, c) in self.groups.iter() {
            for chr in a.chars() {
                if char_in_string(&chr, b) && char_in_string(&chr, c){
                    score += find_char_score(&chr);
                    break;
                }
            }
        }

        Ok(score.to_string())
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(70.to_string()),
            false => Some(2805.to_string()),
        }
    }
}
