// 2016 Day 3
// https://adventofcode.com/2016/day/3

use anyhow::Result;

use crate::puzzle::Puzzle;

#[allow(unused_imports)]
use crate::utils::utils::*;

#[allow(unused_imports)]
use std::collections::VecDeque;

#[derive(Debug)]
struct Triangle {
    sides: Vec<u32>,
    valid: bool,
}

pub struct Day03 {
    triangles: Vec<Triangle>,
}

impl Puzzle for Day03 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut day = Day03 { triangles: vec![] };

        for line in input.trim().split("\n") {
            let mut vals = find_vals(line);
            vals.sort();
            let triangle = Triangle {
                valid: vals[0] + vals[1] > vals[2],
                sides: vals,
            };
            day.triangles.push(triangle);
        }
        log::debug!("Triangle {:?}", day.triangles);

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        let count = self.triangles.iter().fold(0u32, |acc, x| match x.valid {
            true => 1 + acc,
            false => acc,
        });
        Ok(count.to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(1.to_string()),
            false => Some(869.to_string()),
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
