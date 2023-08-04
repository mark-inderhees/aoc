// 2016 Day 1
// https://adventofcode.com/2016/day/1
// --- Day 1: No Time for a Taxicab ---
// Taxicab geometry around Easter Bunny HQ

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
    revisit: Vec<Point<i32>>, // Grid locations that have been revisited, in order
}

impl Puzzle for Day01 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut day = Day01 {
            grid: Board::new(),
            revisit: vec![],
        };

        // Create a board that is big enough
        let width = 1000i32;
        for _ in 0..width {
            day.grid.push_row(vec!['0'; width as usize]);
        }

        // Add a player and start point in the middle
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

        // Can move in 4 directions, commands are rotations
        let mut direction_index = 0i32;
        let directions = vec![
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ];

        // Input looks like R5, which means turn right and move 5 steps
        for command in input.split(",") {
            let command2 = command.trim();

            // For a rotation command, move to next direction
            match command2.chars().next().unwrap() {
                'R' => direction_index += 1,
                'L' => direction_index -= 1,
                _ => panic!("Unsupported rotation"),
            }
            let direction = directions[direction_index.rem_euclid(4) as usize];
            log::debug!("Direction is {:?} for command {}", direction, command2);

            // Now move the number of steps
            let mut steps_str = command2.chars();
            steps_str.next();
            let steps = find_val(steps_str.as_str());
            for _ in 0..steps {
                day.grid.step_player(p1, direction);

                // Check if we have been at this location already
                let loc = day.grid.player_location(p1);
                let value = day.grid.value_at(loc);
                if value != '0' {
                    log::debug!("Part 2 done at {:?}", loc);
                    day.revisit.push(loc)
                }
                day.grid.set_at(loc, '1');
            }
        }

        Ok(day)
    }

    // Find the taxicab distanced between start and end point
    fn solve_part1(&mut self) -> Result<String> {
        let end = self.grid.player_location(0);
        let start = self.grid.player_location(1);
        let diff = end - start;
        let answer = diff.x.abs() + diff.y.abs();

        Ok(answer.to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(8.to_string()),
            false => Some(273.to_string()),
        }
    }

    // Find the taxicab distance between start and first spot revisited
    fn solve_part2(&mut self) -> Result<String> {
        let start = self.grid.player_location(1);
        let revisit = self.revisit[0];
        let diff = revisit - start;
        let answer = diff.x.abs() + diff.y.abs();
        Ok(answer.to_string())
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(4.to_string()),
            false => Some(115.to_string()),
        }
    }
}
