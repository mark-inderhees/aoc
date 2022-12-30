// 2015 Day 3
// https://adventofcode.com/2015/day/3
// --- Day 3: Perfectly Spherical Houses in a Vacuum ---
// Count how many houses santa visits from wacky instructions

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

/// Santa (or robot santa) visit a house. Grid and location state are updated.
fn visit_a_house(day: &mut Day03, player_id: PlayerId, direction: Direction) {
    // Move santa, ensure the move actually happened with unwrap
    let _ = day.board.step_player(player_id, direction).unwrap();

    // Update location state, incrementing how many presents have been sent to this house
    let location = day.board.player_location(player_id);
    let count = day.locations.get(&location);
    let count = match count {
        Some(x) => *x,
        None => 0,
    };
    day.locations.insert(location, count + 1);
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

        // Santa moves on a grid, up down left or right
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

        // Grid needs to be really big
        let width = 1000;
        let height = 1000;
        let row = vec![0; width];
        for _ in 0..height {
            day.board.push_row(row.clone());
        }

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        // How many unique houses does santa visit?
        let player_id = self.board.add_player(
            BoardPoint {
                x: (self.board.width() / 2) as i32,
                y: (self.board.height() / 2) as i32,
            },
            0,
        );

        // Use a hash map to track unique houses
        self.locations
            .insert(self.board.player_location(player_id), 1);
        for direction in self.commands.clone().iter() {
            visit_a_house(self, player_id, *direction);
        }
        let houses = self.locations.len();
        Ok(houses.to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(4.to_string()),
            false => Some(2572.to_string()),
        }
    }

    fn solve_part2(&mut self) -> Result<String> {
        // How many unique houses do santa and robot santa visit?
        // They start at the same spot and alternate commands
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

        // Both give a present at the start
        self.locations.insert(self.board.player_location(santa), 2);

        for direction in self.commands.clone().iter().step_by(2) {
            visit_a_house(self, santa, *direction);
        }

        for direction in self.commands.clone().iter().skip(1).step_by(2) {
            visit_a_house(self, robot, *direction);
        }

        let houses = self.locations.len();
        Ok(houses.to_string())
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(3.to_string()),
            false => Some(2631.to_string()),
        }
    }
}
