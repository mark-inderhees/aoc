use anyhow::Result;

use crate::puzzle::Puzzle;

pub struct Day01 {
    elves: Vec<u32>,
}

impl Puzzle for Day01 {
    fn from_input(input: &str) -> Result<Self> {
        let mut day = Day01 { elves: vec![] };

        for elf in input.split("\r\n\r\n") {
            let calories: u32 = elf
                .lines()
                .map(|calorie| calorie.parse::<u32>().expect("Parse error"))
                .sum();
            day.elves.push(calories);
        }

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        Ok(self.elves.iter().max().expect("Max error").to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(24000.to_string()),
            false => Some(68787.to_string()),
        }
    }

    fn solve_part2(&mut self) -> Result<String> {
        self.elves.sort();
        self.elves.reverse();
        Ok(self.elves[0..3].iter().sum::<u32>().to_string())
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(45000.to_string()),
            false => Some(198041.to_string()),
        }
    }
}
