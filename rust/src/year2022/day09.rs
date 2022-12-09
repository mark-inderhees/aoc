use anyhow::Result;

use crate::puzzle::Puzzle;
use crate::utils::board::*;

pub struct Day09 {
    board: Board<char>,
    visited: Board<char>,
    commands: Vec<(Direction, u32)>,
}

impl Puzzle for Day09 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut day = Day09 {
            board: Board::new(),
            visited: Board::new(),
            commands: vec![],
        };

        for line in input.lines() {
            let l: Vec<&str> = line.split(" ").collect();
            let dir_char = l[0];
            let step_count = l[1].parse::<u32>().unwrap();
            let direction = match dir_char {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => panic!("Did not understand dir char"),
            };
            day.commands.push((direction, step_count));
        }

        let dim = 1000;
        for d in 0..dim {
            day.board.push_row(vec!['.'; dim]);
            day.visited.push_row(vec!['.'; dim]);
        }

        day.board.set_location(dim as i32 / 2, dim as i32 / 2);

        for (direction, step_count) in &day.commands {
            for _ in 0..*step_count {
                day.board.step(*direction);
            }
        }

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        Ok("to do".to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => None,
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
