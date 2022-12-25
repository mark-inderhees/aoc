// 2022 Day 6
// https://adventofcode.com/2022/day/6
// --- Day 6: Tuning Trouble ---
// Scan data for packet types. They key is to use rusts window iterator.

use anyhow::Result;

use crate::puzzle::Puzzle;

pub struct Day06 {
    code: String,
}

/// Is every char in this string unique?
fn all_unique(chars: &[char]) -> bool {
    for char in chars {
        let count = chars.iter().filter(|x| *x==char).count();
        if count > 1 {
            return false;
        }
    }

    true
}

impl Puzzle for Day06 {
    fn from_input(input: &str) -> Result<Self> {
        let day = Day06 {
            code: input.to_string(),
        };
        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        let chars = self.code.chars().collect::<Vec<char>>();
        // Find first window of 4 chars that is unique
        let mut index = 4;
        for window in chars.windows(4) {
            if all_unique(window) {
                break;
            }
            index += 1;
        }
        Ok(index.to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(5.to_string()),
            false => Some(1238.to_string()),
        }
    }

    fn solve_part2(&mut self) -> Result<String> {
        let chars = self.code.chars().collect::<Vec<char>>();
        // Find first window of 14 chars that is unique
        let mut index = 14;
        for window in chars.windows(14) {
            if all_unique(window) {
                break;
            }
            index += 1;
        }
        Ok(index.to_string())
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(23.to_string()),
            false => Some(3037.to_string()),
        }
    }
}
