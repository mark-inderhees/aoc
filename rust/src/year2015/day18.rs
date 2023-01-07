// 2015 Day 18
// https://adventofcode.com/2015/day/18
// --- Day 18: Like a GIF For Your Yard ---

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

fn reset_lights(day: &mut Day18, corners_always_on: bool) {
    // A light which is on stays on when 2 or 3 neighbors are on, and turns off otherwise.
    // A light which is off turns on if exactly 3 neighbors are on, and stays off otherwise.
    let player_id = 0;
    let mut next_values = VecDeque::new();
    for y in 0..day.board.height() {
        for x in 0..day.board.width() {
            let point = BoardPoint { x, y };
            let value = day.board.value_at(point);
            day.board.set_player_location(player_id, point);
            let values = day.board.nearby_values(player_id);
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

    for y in 0..day.board.height() {
        for x in 0..day.board.width() {
            let point = BoardPoint { x, y };
            day.board.set_at(point, next_values.pop_front().unwrap());
        }
    }

    if corners_always_on {
        let dim = day.board.width() - 1; // Width and height are the same
        day.board.set_at(BoardPoint { x: 0, y: 0 }, '#');
        day.board.set_at(BoardPoint { x: dim, y: 0 }, '#');
        day.board.set_at(BoardPoint { x: 0, y: dim }, '#');
        day.board.set_at(BoardPoint { x: dim, y: dim }, '#');
    }
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

        day.board.add_player(BoardPoint { x: 0, y: 0 }, '?');
        // day.board.print_board_with_players_pretty();

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        let steps = if self.board.width() > 50 { 100 } else { 4 };
        for _ in 0..steps {
            reset_lights(self, false);
        }

        let mut count = 0;
        for y in 0..self.board.height() {
            for x in 0..self.board.width() {
                let point = BoardPoint { x, y };
                if self.board.value_at(point) == '#' {
                    count += 1;
                }
            }
        }
        Ok(count.to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(4.to_string()),
            false => Some(814.to_string()),
        }
    }

    fn solve_part2(&mut self) -> Result<String> {
        let dim = self.board.width() - 1; // Width and height are the same
        self.board.set_at(BoardPoint { x: 0, y: 0 }, '#');
        self.board.set_at(BoardPoint { x: dim, y: 0 }, '#');
        self.board.set_at(BoardPoint { x: 0, y: dim }, '#');
        self.board.set_at(BoardPoint { x: dim, y: dim }, '#');

        let steps = if self.board.width() > 50 { 100 } else { 5 };
        for _ in 0..steps {
            reset_lights(self, true);
        }

        let mut count = 0;
        for y in 0..self.board.height() {
            for x in 0..self.board.width() {
                let point = BoardPoint { x, y };
                if self.board.value_at(point) == '#' {
                    count += 1;
                }
            }
        }
        Ok(count.to_string())
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(17.to_string()),
            false => Some(924.to_string()),
        }
    }
}
