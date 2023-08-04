// 2016 Day 1
// https://adventofcode.com/2016/day/1

use anyhow::Result;
use rusttype::Point;

use crate::{
    puzzle::Puzzle,
    utils::board::{Board, BoardPoint, Direction},
};

#[allow(unused_imports)]
use crate::utils::utils::*;

#[allow(unused_imports)]
use std::collections::VecDeque;

pub struct Day01 {
    grid: Board<char>,
}

impl Puzzle for Day01 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut day = Day01 { grid: Board::new() };

        let width = 1000i32;
        for _ in 0..width {
            day.grid.push_row(vec!['.'; width as usize]);
        }
        let p1 = day.grid.add_player(
            BoardPoint {
                x: width / 2,
                y: width / 2,
            },
            'x',
        );
        let start = day.grid.add_player(
            BoardPoint {
                x: width / 2,
                y: width / 2,
            },
            's',
        );

        let mut direction_index = 0i32;
        let directions = vec![
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ];
        for command in input.split(",") {
            let command2 = command.trim();
            match command2.chars().next().unwrap() {
                'R' => direction_index += 1,
                'L' => direction_index -= 1,
                _ => panic!("Unsupported rotation"),
            }
            let direction = directions[direction_index.rem_euclid(4) as usize];
            log::debug!("Direction is {:?} for command {}", direction, command2);
            let mut bob = command2.chars();
            bob.next();
            let steps = find_val(bob.as_str());
            for _ in 0..steps {
                day.grid.step_player(p1, direction);
            }
        }

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        let point1 = self.grid.player_location(0);
        let start1 = self.grid.player_location(1);
        let x = point1.x - start1.x;
        let y = point1.y - start1.y;

        Ok((x.abs() + y.abs()).to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(12.to_string()),
            false => Some(273.to_string()),
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
