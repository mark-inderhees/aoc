use anyhow::Result;

use crate::puzzle::Puzzle;

#[allow(unused_imports)]
use crate::utils::utils::*;

pub struct Day20 {
    values: Vec<i32>,
}

impl Puzzle for Day20 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut day = Day20 { values: vec![] };

        for (i, line) in input.lines().enumerate() {
            day.values.push(get_val(line));
        }

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        let mut decoded: Vec<(usize, &i32)> = self.values.iter().enumerate().collect();
        for (original_index, value) in self.values.iter().enumerate() {
            let current_index = decoded
                .iter()
                .position(|r| r == &(original_index, value))
                .unwrap();
            decoded.remove(current_index);
            let mut new_index = value + current_index as i32;
            new_index = new_index.rem_euclid(decoded.len() as i32);
            if new_index == 0 {
                new_index = decoded.len() as i32;
            }
            decoded.insert(new_index as usize, (new_index as usize, value));
            log::debug!("{decoded:?}");
        }

        let zero = decoded.iter().position(|&r| *r.1 == 0).unwrap();
        let len = decoded.len();
        let one = decoded[(zero + 1000) % len].1;
        let two = decoded[(zero + 2000) % len].1;
        let three = decoded[(zero + 3000) % len].1;
        let answer = one + two + three;

        Ok(answer.to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(3.to_string()),
            false => None,
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
