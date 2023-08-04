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
    commands: Vec<Vec<Direction>>,
}

impl Puzzle for Day02 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut day = Day02 { commands: vec![] };

        for line in input.trim().split('\n') {
            let mut command = vec![];
            for direction_char in line.chars() {
                let direction = match direction_char {
                    'U' => Direction::Up,
                    'D' => Direction::Down,
                    'L' => Direction::Left,
                    'R' => Direction::Right,
                    _ => panic!("Unknown direction char"),
                };
                command.push(direction)
            }
            day.commands.push(command);
        }

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        let mut grid = Board::new();
        let mut values = vec![];
        grid.push_row(vec!['1', '2', '3']);
        grid.push_row(vec!['4', '5', '6']);
        grid.push_row(vec!['7', '8', '9']);
        let middle = BoardPoint { x: 1, y: 1 };
        let p1 = grid.add_player(middle, 'x');

        for command in self.commands.iter() {
            for direction in command {
                grid.step_player(p1, *direction);
            }
            let value = grid.player_value(p1);
            log::debug!("Value is {}", value);
            values.push(value.to_string());
        }
        Ok(values.join(""))
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some("1985".to_string()),
            false => Some("76792".to_string()),
        }
    }

    fn solve_part2(&mut self) -> Result<String> {
        let mut grid = Board::new();
        let mut values = vec![];
        grid.push_row(vec![' ', ' ', '1', ' ', ' ']);
        grid.push_row(vec![' ', '2', '3', '4', ' ']);
        grid.push_row(vec!['5', '6', '7', '8', '9']);
        grid.push_row(vec![' ', 'A', 'B', 'C', ' ']);
        grid.push_row(vec![' ', ' ', 'D', ' ', ' ']);
        grid.add_wall(' ');
        let middle = BoardPoint { x: 0, y: 2 };
        let p1 = grid.add_player(middle, 'x');

        for command in self.commands.iter() {
            for direction in command {
                grid.step_player(p1, *direction);
            }
            let value = grid.player_value(p1);
            log::debug!("Value is {}", value);
            values.push(value.to_string());
        }
        Ok(values.join(""))
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some("5DB3".to_string()),
            false => None,
        }
    }
}
