// 2016 Day 2
// https://adventofcode.com/2016/day/2

use anyhow::Result;

use crate::puzzle::Puzzle;
use crate::utils::board::*;

#[allow(unused_imports)]
use crate::utils::utils::*;

#[allow(unused_imports)]
use std::collections::VecDeque;

pub struct Day02 {
    grid: Board<u32>,
    values: Vec<String>,
}

impl Puzzle for Day02 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut day = Day02 {
            grid: Board::new(),
            values: vec![],
        };

        day.grid.push_row(vec![1, 2, 3]);
        day.grid.push_row(vec![4, 5, 6]);
        day.grid.push_row(vec![7, 8, 9]);
        let middle = BoardPoint { x: 1, y: 1 };
        let p1 = day.grid.add_player(middle, 0);

        for line in input.trim().split('\n') {
            // day.grid.set_player_location(p1, middle);
            for direction_char in line.chars() {
                let direction = match direction_char {
                    'U' => Direction::Up,
                    'D' => Direction::Down,
                    'L' => Direction::Left,
                    'R' => Direction::Right,
                    _ => panic!("Unknown direction char"),
                };
                day.grid.step_player(p1, direction);
            }
            let value = day.grid.player_value(p1);
            log::debug!("Value is {}", value);
            day.values.push(value.to_string());
        }

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        Ok(self.values.join(""))
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some("1985".to_string()),
            false => Some("76792".to_string()),
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
