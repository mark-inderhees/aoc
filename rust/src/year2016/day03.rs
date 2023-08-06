// 2016 Day 3
// https://adventofcode.com/2016/day/3
// --- Day 3: Squares With Three Sides ---
// Determine if lenghts of triangle reveal valid triangles

use anyhow::Result;

use crate::puzzle::Puzzle;

#[allow(unused_imports)]
use crate::utils::utils::*;

#[allow(unused_imports)]
use std::collections::VecDeque;
use std::vec;

pub struct Day03 {
    triangles: Vec<Vec<u32>>,
}

// Given a triangle with three sides, determine if it is a valid triangle.
// Return the number of valid triangles.
fn count_valid_triangles(triangles: &Vec<Vec<u32>>) -> u32 {
    triangles.iter().fold(0u32, |acc, x| {
        let mut sides = x.clone();
        sides.sort();
        match sides[0] + sides[1] > sides[2] {
            true => 1 + acc,
            false => acc,
        }
    })
}

impl Puzzle for Day03 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut day = Day03 { triangles: vec![] };

        for line in input.trim().split("\n") {
            // Each line has three lengths of the triangle
            let vals = find_vals(line);
            day.triangles.push(vals);
        }
        log::debug!("Triangle {:?}", day.triangles);

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        // How many triangles are valid?
        let count = count_valid_triangles(&self.triangles);
        Ok(count.to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(3.to_string()),
            false => Some(869.to_string()),
        }
    }

    fn solve_part2(&mut self) -> Result<String> {
        // Triangles are actually listed in columns, so read in chuncks of 3
        // and build real triangles.
        let mut triangles = vec![];
        let chunks: Vec<&[Vec<u32>]> = self.triangles.chunks(3).collect();
        for chunk in chunks {
            for i in 0..3 {
                triangles.push(vec![chunk[0][i], chunk[1][i], chunk[2][i]])
            }
        }

        // Now how many of these real triangles are valid?
        let count = count_valid_triangles(&triangles);
        Ok(count.to_string())
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(6.to_string()),
            false => Some(1544.to_string()),
        }
    }
}
