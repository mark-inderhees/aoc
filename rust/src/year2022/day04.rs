// 2022 Day 4
// https://adventofcode.com/2022/day/4
// --- Day 4: Camp Cleanup ---
// For two ranges, find if they contain and overlap.

use anyhow::Result;
use std::ops::RangeInclusive;

use crate::puzzle::Puzzle;
use crate::utils::utils::*;

pub struct Day04 {
    groups: Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>,
}

impl Puzzle for Day04 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut day = Day04 { groups: vec![] };

        for line in input.lines() {
            // Get two ranges from input like
            // 5-7,7-9
            let line2 = line.replace("-", " "); // Remove '-' which could be a negative number
            let values = find_vals(&line2);
            day.groups.push((values[0]..=values[1], values[2]..=values[3]));
        }

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        let mut count = 0;
        // Count when one range fully contains the other range
        for (elf1, elf2) in self.groups.iter_mut() {
            if (elf1.contains(elf2.start()) && elf1.contains(elf2.end()))
                || (elf2.contains(elf1.start()) && elf2.contains(elf1.end()))
            {
                count += 1;
            }
        }

        Ok(count.to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(2.to_string()),
            false => Some(456.to_string()),
        }
    }

    fn solve_part2(&mut self) -> Result<String> {
        let mut count = 0;
        // Count number of overlaps
        for (elf1, elf2) in self.groups.iter_mut() {
            if elf1.contains(elf2.start())
                || elf1.contains(elf2.end())
                || elf2.contains(elf1.start())
                || elf2.contains(elf1.end())
            {
                count += 1;
            }
        }

        Ok(count.to_string())
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(4.to_string()),
            false => Some(808.to_string()),
        }
    }
}
