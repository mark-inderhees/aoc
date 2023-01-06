// 2015 Day 16
// https://adventofcode.com/2015/day/16
// --- Day 16: Aunt Sue ---
// Find the correct aunt sue based on clues and partial known info

use anyhow::Result;
use std::collections::HashMap;

use crate::puzzle::Puzzle;
use crate::utils::utils::*;

pub struct Day16 {
    analysis: HashMap<String, u32>,
    sues: Vec<Sue>,
}

#[derive(Debug)]
struct Sue {
    number: u32,
    contents: HashMap<String, u32>,
}

/// Does this Sue match the analysis?
fn does_sue_match(sue: &Sue, day: &Day16) -> bool {
    for (key, value) in day.analysis.iter() {
        if sue.contents.contains_key(key) {
            if sue.contents[key] != *value {
                return false;
            }
        }
    }

    true
}

/// Does this Sue match the analysis with some funny rules?
fn does_sue_match_funny(sue: &Sue, day: &Day16) -> bool {
    for (key, value) in day.analysis.iter() {
        if sue.contents.contains_key(key) {
            if key == "cats" || key == "trees" {
                // cats and trees readings indicates that there are greater than that many
                if sue.contents[key] <= *value {
                    return false;
                }
            } else if key == "pomeranians" || key == "goldfish" {
                // pomeranians and goldfish readings indicate that there are fewer than that many
                if sue.contents[key] >= *value {
                    return false;
                }
            } else if sue.contents[key] != *value {
                // else compare for exact match like normal
                return false;
            }
        }
    }

    true
}

impl Puzzle for Day16 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        // Raw analysis info given in addition to the input
        let analysis = [
            ("children".to_string(), 3),
            ("cats".to_string(), 7),
            ("samoyeds".to_string(), 2),
            ("pomeranians".to_string(), 3),
            ("akitas".to_string(), 0),
            ("vizslas".to_string(), 0),
            ("goldfish".to_string(), 5),
            ("trees".to_string(), 3),
            ("cars".to_string(), 2),
            ("perfumes".to_string(), 1),
        ];

        #[allow(unused_mut)]
        let mut day = Day16 {
            analysis: analysis.into_iter().collect(),
            sues: vec![],
        };

        for line in input.lines() {
            // Line is like
            // Sue 1: goldfish: 6, trees: 9, pomeranians: 0
            let clean = line.replace(":", "");
            let splits: Vec<&str> = clean.split(" ").collect();
            let values: Vec<u32> = find_vals(line);
            let number = values[0];
            let mut sue = Sue {
                number,
                contents: HashMap::new(),
            };
            for (i, item) in splits.iter().skip(2).step_by(2).enumerate() {
                sue.contents.insert(item.to_string(), values[i + 1]);
            }
            day.sues.push(sue);
        }

        log::debug!("{:#?}", day.analysis);
        log::debug!("{:#?}", day.sues);

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        // Search for matching Sue
        let mut answer = 0;
        for sue in self.sues.iter() {
            if does_sue_match(sue, self) {
                answer = sue.number;
            }
        }
        Ok(answer.to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(2.to_string()),
            false => Some(103.to_string()),
        }
    }

    fn solve_part2(&mut self) -> Result<String> {
        // Search for matching Sue with funny rules
        let mut answer = 0;
        for sue in self.sues.iter() {
            if does_sue_match_funny(sue, self) {
                answer = sue.number;
            }
        }
        Ok(answer.to_string())
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(1.to_string()),
            false => Some(405.to_string()),
        }
    }
}
