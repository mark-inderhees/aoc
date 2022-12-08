use anyhow::Result;
use std::ops::RangeInclusive;

use crate::puzzle::Puzzle;

pub struct Day04 {
    groups: Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>,
}

impl Puzzle for Day04 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut day = Day04 { groups: vec![] };

        for line in input.lines() {
            let elves: Vec<&str> = line.split(",").collect();
            let r1: Vec<&str> = elves[0].split("-").collect();
            let r2: Vec<&str> = elves[1].split("-").collect();
            let r1_0: u32 = r1[0].parse()?;
            let r1_1: u32 = r1[1].parse()?;
            let r2_0: u32 = r2[0].parse()?;
            let r2_1: u32 = r2[1].parse()?;
            day.groups.push((r1_0..=r1_1, r2_0..=r2_1));
        }

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        let mut count = 0;
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
