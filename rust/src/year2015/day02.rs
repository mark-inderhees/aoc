// 2015 Day 2
// https://adventofcode.com/2015/day/2
// --- Day 2: I Was Told There Would Be No Math ---
// Wrap some presents

use anyhow::Result;

use crate::puzzle::Puzzle;

#[allow(unused_imports)]
use crate::utils::utils::*;

#[allow(unused_imports)]
use std::collections::VecDeque;

pub struct Day02 {
    presents: Vec<Dimensions>,
}

struct Dimensions {
    length: u32,
    width: u32,
    height: u32,
}

impl Puzzle for Day02 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut day = Day02 { presents: vec![] };

        for line in input.lines() {
            let present = line.replace("x", " ");
            let vals: Vec<u32> = find_vals(&present);
            day.presents.push(Dimensions {
                length: vals[0],
                width: vals[1],
                height: vals[2],
            });
        }

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        // Find area of wrapping paper needed
        // Find total surface area for each present
        // Plus some extra, which is the area of smallest side
        let mut area = 0;
        for present in self.presents.iter() {
            let side1 = present.length * present.width;
            let side2 = present.width * present.height;
            let side3 = present.height * present.length;
            let mut extra = std::cmp::min(side1, side2);
            extra = std::cmp::min(extra, side3);
            area += side1 * 2 + side2 * 2 + side3 * 2 + extra;
        }

        Ok(area.to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(58.to_string()),
            false => Some(1588178.to_string()),
        }
    }

    fn solve_part2(&mut self) -> Result<String> {
        // Find ribbon needed for a bow for each present
        // Will be shortest circumference
        // Plus some extra, which is volumn of present (silly!)
        let mut ribbon = 0;
        for present in self.presents.iter() {
            let mut sides = vec![present.length, present.width, present.height];
            sides.sort();
            ribbon += sides[0] * 2 + sides[1] * 2;
            let volume = present.length * present.width * present.height;
            ribbon += volume;
        }

        Ok(ribbon.to_string())
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(34.to_string()),
            false => Some(3783758.to_string()),
        }
    }
}
