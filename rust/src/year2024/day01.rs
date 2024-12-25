// 2024 Day 1
// https://adventofcode.com/2024/day/1

use anyhow::Result;

use crate::puzzle::Puzzle;

#[allow(unused_imports)]
use crate::utils::utils::*;

#[allow(unused_imports)]
use std::collections::VecDeque;

pub struct Day01 {
    list1: Vec<u32>,
    list2: Vec<u32>,
}

impl Puzzle for Day01 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut day = Day01 {
            list1: vec![],
            list2: vec![],
        };

        for row in input.lines() {
            let values: Vec<u32> = find_vals(row);
            day.list1.push(values[0]);
            day.list2.push(values[1]);
        }

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        self.list1.sort();
        self.list2.sort();
        let mut distance = 0;
        for (a, b) in self.list1.iter().zip(self.list2.iter()) {
            if a > b {
                distance += a - b;
            } else {
                distance += b - a;
            }
        }
        Ok(distance.to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(11.to_string()),
            false => Some(1646452.to_string()),
        }
    }

    fn solve_part2(&mut self) -> Result<String> {
        let mut similarity = 0;
        for a in self.list1.iter() {
            let mut count = 0;
            for b in self.list2.iter() {
                if a == b {
                    count += 1;
                }
            }
            similarity += count * a;
        }

        Ok(similarity.to_string())
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(31.to_string()),
            false => None,
        }
    }
}
