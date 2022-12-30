// 2015 Day 3
// https://adventofcode.com/2015/day/3
// --- Day 3: Perfectly Spherical Houses in a Vacuum ---

use anyhow::Result;
use std::collections::HashMap;

use crate::puzzle::Puzzle;
use crate::utils::board::*;

#[allow(unused_imports)]
use crate::utils::utils::*;

#[allow(unused_imports)]
use std::collections::VecDeque;

pub struct Day03 {
    commands: Vec<Direction>,
    board: Board<u32>,
    locations: HashMap<BoardPoint, u32>,
}

impl Puzzle for Day03 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut day = Day03 {
            commands: vec![],
            board: Board::new(),
            locations: HashMap::new(),
        };

        let line = input.trim();
        for char in line.chars() {
            let direction = match char {
                '^' => Direction::Up,
                'v' => Direction::Down,
                '>' => Direction::Right,
                '<' => Direction::Left,
                _ => panic!("Unexpected char"),
            };
            day.commands.push(direction);
        }

        let width = 1000;
        let height = 1000;
        let row = vec![0; width];
        for _ in 0..height {
            day.board.push_row(row.clone());
        }

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        let player_id = self.board.add_player(
            BoardPoint {
                x: (self.board.width() / 2) as i32,
                y: (self.board.height() / 2) as i32,
            },
            0,
        );

        self.locations
            .insert(self.board.player_location(player_id), 1);
        for direction in self.commands.iter() {
            let _ = self.board.step_player(player_id, *direction).unwrap();

            let location = self.board.player_location(player_id);
            let count = self.locations.get(&location);
            let count = match count {
                Some(x) => *x,
                None => 0,
            };
            self.locations.insert(location, count + 1);
        }
        let houses = self.locations.len();
        Ok(houses.to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(4.to_string()),
            false => None,
        }
    }

    fn solve_part2(&mut self) -> Result<String> {
        let santa = self.board.add_player(
            BoardPoint {
                x: (self.board.width() / 2) as i32,
                y: (self.board.height() / 2) as i32,
            },
            0,
        );
        let robot = self.board.add_player(
            BoardPoint {
                x: (self.board.width() / 2) as i32,
                y: (self.board.height() / 2) as i32,
            },
            0,
        );

        self.locations.insert(self.board.player_location(santa), 2);
        for direction in self.commands.iter().step_by(2) {
            let _ = self.board.step_player(santa, *direction).unwrap();

            let location = self.board.player_location(santa);
            let count = self.locations.get(&location);
            let count = match count {
                Some(x) => *x,
                None => 0,
            };
            self.locations.insert(location, count + 1);
        }

        for direction in self.commands.iter().skip(1).step_by(2) {
            let _ = self.board.step_player(robot, *direction).unwrap();

            let location = self.board.player_location(robot);
            let count = self.locations.get(&location);
            let count = match count {
                Some(x) => *x,
                None => 0,
            };
            self.locations.insert(location, count + 1);
        }

        let houses = self.locations.len();
        Ok(houses.to_string())
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(3.to_string()),
            false => None,
        }
    }
}
