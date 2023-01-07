// 2015 Day 18
// https://adventofcode.com/2015/day/18
// --- Day 18: Like a GIF For Your Yard ---
// Toggle lights on a 100x100 grid

use anyhow::Result;

use crate::puzzle::Puzzle;
use crate::utils::board::*;

#[allow(unused_imports)]
use crate::utils::utils::*;

#[allow(unused_imports)]
use std::collections::VecDeque;

pub struct Day18 {
    board: Board<char>,
}

/// Toggle the lights in the grid.
/// A light which is on stays on when 2 or 3 neighbors are on, and turns off otherwise.
/// A light which is off turns on if exactly 3 neighbors are on, and stays off otherwise.
fn reset_lights(day: &mut Day18, corners_always_on: bool) {
    // Store the next state of the lights
    let mut next_values = VecDeque::new();

    // Look at all the lights and calculate the next state
    for y in 0..day.board.height() {
        for x in 0..day.board.width() {
            let point = BoardPoint { x, y };
            let value = day.board.value_at(point);
            let values = day.board.values_near_point(point);
            let neighbors_on = values
                .iter()
                .fold(0, |a, v| if *v == '#' { a + 1 } else { a });
            if value == '#' {
                // Currently on, need 2 or 3 neighbors to stay on
                if neighbors_on == 2 || neighbors_on == 3 {
                    next_values.push_back('#');
                } else {
                    next_values.push_back('.');
                }
            } else {
                // Currently off, need 3 neighbors on to turn on
                if neighbors_on == 3 {
                    next_values.push_back('#');
                } else {
                    next_values.push_back('.');
                }
            }
        }
    }

    // Now set the updated lights value
    for y in 0..day.board.height() {
        for x in 0..day.board.width() {
            let point = BoardPoint { x, y };
            day.board.set_at(point, next_values.pop_front().unwrap());
        }
    }

    if corners_always_on {
        turn_on_corners(day);
    }
}

/// Force the corner lights to be on
fn turn_on_corners(day: &mut Day18) {
    let dim = day.board.width() - 1; // Width and height are the same
    day.board.set_at(BoardPoint { x: 0, y: 0 }, '#');
    day.board.set_at(BoardPoint { x: dim, y: 0 }, '#');
    day.board.set_at(BoardPoint { x: 0, y: dim }, '#');
    day.board.set_at(BoardPoint { x: dim, y: dim }, '#');
}

/// Count how many lights are turned on
fn count_lights_on(day: &Day18) -> u32 {
    let mut count = 0;
    for y in 0..day.board.height() {
        for x in 0..day.board.width() {
            let point = BoardPoint { x, y };
            if day.board.value_at(point) == '#' {
                count += 1;
            }
        }
    }
    count
}

impl Puzzle for Day18 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut day = Day18 {
            board: Board::new(),
        };

        for line in input.lines() {
            let chars: Vec<char> = line.chars().collect();
            day.board.push_row(chars);
        }

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        // Toggle the lights 100 times
        let steps = if self.board.width() > 50 { 100 } else { 4 };
        for _ in 0..steps {
            reset_lights(self, false);
        }

        // Count how many lights are on
        let count = count_lights_on(self);
        Ok(count.to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(4.to_string()),
            false => Some(814.to_string()),
        }
    }

    fn solve_part2(&mut self) -> Result<String> {
        // Corners must always be on
        turn_on_corners(self);

        // Toggle the lights 100 times
        let steps = if self.board.width() > 50 { 100 } else { 5 };
        for _ in 0..steps {
            reset_lights(self, true);
        }

        // Count how many lights are on
        let count = count_lights_on(self);
        Ok(count.to_string())
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(17.to_string()),
            false => Some(924.to_string()),
        }
    }
}
