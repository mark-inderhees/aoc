// 2015 Day 25
// https://adventofcode.com/2015/day/25
// --- Day 25: Let It Snow ---
// Simply move through a grid diagonally.

use anyhow::Result;

use crate::puzzle::Puzzle;
use crate::utils::utils::*;

pub struct Day25 {
    x_target: u32,
    y_target: u32,
}

impl Puzzle for Day25 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        // Input is simply the x,y target
        let vals: Vec<u32> = find_vals(input.trim());
        #[allow(unused_mut)]
        let mut day = Day25 {
            x_target: vals[1],
            y_target: vals[0],
        };

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        // There is no patterns as we are using big prime numbers, need to brute force
        let mut current = 20151125u64;
        let mut x = 1;
        let mut y = 1;
        let mut y_max = y;
        loop {
            if x == self.x_target && y == self.y_target {
                log::debug!("Found {current}");
                break;
            }

            let next = (current * 252533u64) % 33554393u64;

            // Do the diagonal move, this is the main key part of the day
            if y == 1 {
                // Start new diagonal
                x = 1;
                y = y_max + 1;
                y_max = y;
            } else {
                // Move diagonally
                x += 1;
                y -= 1;
            }

            current = next;
        }

        Ok(current.to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(10600672.to_string()),
            false => Some(9132360.to_string()),
        }
    }

    fn solve_part2(&mut self) -> Result<String> {
        // There is no part 2!
        Ok(12.to_string())
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(12.to_string()),
            false => Some(12.to_string()),
        }
    }
}
