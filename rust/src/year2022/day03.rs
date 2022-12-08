use anyhow::Result;

use crate::puzzle::Puzzle;
use crate::utils::utils::*;

pub struct Day03 {
    rucksacks: Vec<(String, String)>,
    groups: Vec<(String, String, String)>,
}

fn get_char_value(c: &char) -> u32 {
    let value = match c {
        'a'..='z' => (*c as u32) - 96,
        _ => (*c as u32) - 64 + 26,
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
            let len = line.len() / 2;
            let compartment1 = &line[..len];
            let compartment2 = &line[len..];
            day.rucksacks
                .push((compartment1.to_string(), compartment2.to_string()));
        }

        let mut lines = input.lines();
        while let (Some(a), Some(b), Some(c)) = (lines.next(), lines.next(), lines.next()) {
            day.groups
                .push((a.to_string(), b.to_string(), c.to_string()));
        }

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        let mut score = 0;
        for (a, b) in self.rucksacks.iter() {
            for c in a.chars() {
                if char_in_string(&c, b) {
                    score += get_char_value(&c);
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
        for (a, b, c) in self.groups.iter() {
            for chr in a.chars() {
                if char_in_string(&chr, b) && char_in_string(&chr, c){
                    score += get_char_value(&chr);
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
