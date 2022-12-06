use anyhow::Result;

use crate::puzzle::Puzzle;

#[derive(Debug)]
pub struct Day06 {
    code: String,
}

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
        let mut index = 4;
        for window in chars.windows(4) {
            if all_unique(window) {
                break;
            }
            index += 1;
        }
        Ok(index.to_string())
    }

    fn solve_part2(&mut self) -> Result<String> {
        Ok("to do".to_string())
    }
}
