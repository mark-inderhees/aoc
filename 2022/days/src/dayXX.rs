use anyhow::Result;

use crate::puzzle::Puzzle;

#[derive(Debug)]
pub struct DayXX {

}

impl Puzzle for DayXX {
    fn from_input(input: &str) -> Result<Self> {
        let mut day = DayXX { };

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        Ok("to do".to_string())
    }

    fn solve_part2(&mut self) -> Result<String> {
        Ok("to do".to_string())
    }
}
