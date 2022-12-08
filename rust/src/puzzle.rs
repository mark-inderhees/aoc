use anyhow::Result;

pub trait Puzzle: Sized {
    fn from_input(input: &str) -> Result<Self>;
    fn solve_part1(&mut self) -> Result<String>;
    fn solve_part2(&mut self) -> Result<String>;
    fn answer_part1(&mut self, test: bool) -> Option<String>;
    fn answer_part2(&mut self, test: bool) -> Option<String>;
}
