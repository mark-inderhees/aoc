use anyhow::Result;
use std::cmp::*;

use crate::puzzle::Puzzle;
use crate::utils::board::*;

#[allow(unused_imports)]
use crate::utils::utils::*;

pub struct Day15 {
    target_row: i32,
    pairs: Vec<Pair>,
}

#[derive(Debug)]
struct Pair {
    sensor: BoardPoint,
    beacon: BoardPoint,
    distance: i32,
}

fn manhattan_distance(p1: BoardPoint, p2: BoardPoint) -> i32 {
    let x = (p1.x - p2.x).abs();
    let y = (p1.y - p2.y).abs();
    x + y
}

fn sensor_covers_row(pair: &Pair, row: i32) -> bool {
    let minimum = pair.sensor.y - pair.distance;
    let maximum = pair.sensor.y + pair.distance;
    (minimum..=maximum).contains(&row)
}

fn draw_manhattan_radius(p: BoardPoint, dist: i32, board: &mut Board<char>) {
    let mut x_offset = 0;
    let y_min = p.y - dist;
    let y_max = p.y + dist;
    for y in y_min..p.y {
        for x in (p.x - x_offset)..=(p.x + x_offset) {
            board.set_at(BoardPoint { x, y }, '#');
        }
        x_offset += 1;
    }

    for y in p.y..=y_max {
        for x in (p.x - x_offset)..=(p.x + x_offset) {
            board.set_at(BoardPoint { x, y }, '#');
        }
        x_offset -= 1;
    }
}

fn offset_point(p: BoardPoint, offset: BoardPoint) -> BoardPoint {
    BoardPoint {
        x: p.x - offset.x,
        y: p.y - offset.y,
    }
}

impl Puzzle for Day15 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut day = Day15 {
            target_row: 0,
            pairs: vec![],
        };

        let test = input.lines().count() < 20;
        day.target_row = match test {
            true => 10,
            false => 2000000,
        };

        for line in input.lines() {
            let vals: Vec<i32> = get_vals(line);
            let sensor = BoardPoint {
                x: vals[0],
                y: vals[1],
            };
            let beacon = BoardPoint {
                x: vals[2],
                y: vals[3],
            };
            let pair = Pair {
                sensor,
                beacon,
                distance: manhattan_distance(sensor, beacon),
            };
            day.pairs.push(pair);
        }

        let min_x = day
            .pairs
            .iter()
            .fold(i32::MAX, |a, p| min(a, min(p.sensor.x, p.beacon.x)));
        let min_y = day
            .pairs
            .iter()
            .fold(i32::MAX, |a, p| min(a, min(p.sensor.y, p.beacon.y)));
        let max_x = day
            .pairs
            .iter()
            .fold(0, |a, p| max(a, max(p.sensor.x, p.beacon.x)));
        let max_y = day
            .pairs
            .iter()
            .fold(0, |a, p| max(a, max(p.sensor.y, p.beacon.y)));
        let max_dist = day.pairs.iter().fold(0, |a, p| max(a, p.distance));

        log::debug!("Pairs {:?}", day.pairs);
        log::debug!("Min {min_x},{min_y} to max {max_x}, {max_y}");
        log::debug!("Max distance {max_dist}");

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        let mut pairs = vec![];
        for pair in &self.pairs {
            if sensor_covers_row(pair, self.target_row) {
                pairs.push(pair);
            }
        }

        let max_dist = pairs.iter().fold(0, |a, p| max(a, p.distance));
        let min_x = pairs
            .iter()
            .fold(i32::MAX, |a, p| min(a, min(p.sensor.x, p.beacon.x)))
            - max_dist;
        let min_y = pairs
            .iter()
            .fold(i32::MAX, |a, p| min(a, min(p.sensor.y, p.beacon.y)))
            - max_dist;
        let max_x = pairs
            .iter()
            .fold(0, |a, p| max(a, max(p.sensor.x, p.beacon.x)))
            + max_dist;
        let max_y = pairs
            .iter()
            .fold(0, |a, p| max(a, max(p.sensor.y, p.beacon.y)))
            + max_dist;

        log::debug!("Pairs {:?}", pairs);
        log::debug!("Min {min_x},{min_y} to max {max_x}, {max_y}");
        log::debug!("Max distance {max_dist}");

        let mut board = Board::<char>::new();
        let width = max_x - min_x + 1;
        let height = max_y - min_y + 1;
        let offset = BoardPoint { x: min_x, y: min_y };
        for _ in 0..height {
            board.push_row(vec!['.'; width as usize]);
        }
        for pair in pairs {
            board.add_player(offset_point(pair.sensor, offset), 'S');
            board.add_player(offset_point(pair.beacon, offset), 'B');
            draw_manhattan_radius(offset_point(pair.sensor, offset), pair.distance, &mut board);
        }
        board.print_board_with_players_pretty();

        let mut count = 0;
        for x in 0..width {
            let here = BoardPoint {
                x,
                y: self.target_row - offset.y,
            };
            let value = board.get_at(here);
            if value == '#' && !board.player_is_here(here) {
                count += 1;
            }
        }

        Ok(count.to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(26.to_string()),
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
