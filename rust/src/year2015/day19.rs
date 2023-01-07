// 2015 Day 19
// https://adventofcode.com/2015/day/19
// --- Day 19: Medicine for Rudolph ---

use anyhow::Result;

use crate::puzzle::Puzzle;

#[allow(unused_imports)]
use crate::utils::utils::*;

use std::collections::HashMap;
#[allow(unused_imports)]
use std::collections::VecDeque;

pub struct Day19 {
    molecule: String,
    replacements: Vec<Replacement>,
    starts: Vec<String>,
}

#[derive(Debug)]
struct Replacement {
    start: String,
    end: String,
}

fn count_replacements(day: &Day19) -> usize {
    let mut count: HashMap<String, bool> = HashMap::new();

    for replacement in day.replacements.iter() {
        for (i, _) in day.molecule.match_indices(&replacement.start) {
            let splits = day.molecule.split_at(i);
            let mut one = splits.0.to_string();
            let two = splits.1.replacen(&replacement.start, &replacement.end, 1);
            one.push_str(&two);
            count.insert(one, true);
        }
    }

    log::debug!("{:#?}", count);

    count.len()
}

impl Puzzle for Day19 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        let split: Vec<&str> = input.trim().split("\n\n").collect();

        #[allow(unused_mut)]
        let mut day = Day19 {
            molecule: split[1].to_string(),
            replacements: vec![],
            starts: vec![],
        };

        for line in split[0].lines() {
            let molecules: Vec<&str> = line.split(" ").collect();
            if line.starts_with("e") {
                day.starts.push(molecules[2].to_string());
            } else {
                day.replacements.push(Replacement {
                    start: molecules[0].to_string(),
                    end: molecules[2].to_string(),
                });
            }
        }

        log::debug!("{}", day.molecule);
        log::debug!("{:#?}", day.replacements);

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        let answer = count_replacements(self);
        Ok(answer.to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(7.to_string()),
            false => Some(509.to_string()),
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
