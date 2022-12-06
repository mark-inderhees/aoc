use anyhow::Result;

use crate::puzzle::Puzzle;

#[derive(Debug)]
pub struct Day01 {
    elves: Vec<u32>,
}

impl Puzzle for Day01 {
    fn from_input(input: &str) -> Result<Self> {
        let mut day = Day01 { elves: vec![] };

        for elf in input.split("\r\n\r\n") {
            let calories:u32 = elf.lines().map(|calorie| calorie.parse::<u32>().expect("")).sum();
            day.elves.push(calories);
        }

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        Ok(self.elves.iter().max().expect("err").to_string())
    }

    fn solve_part2(&mut self) -> Result<String> {
        self.elves.sort();
        self.elves.reverse();
        Ok(self.elves[0..3].iter().sum::<u32>().to_string())
    }
}
