use anyhow::Result;
use rusttype::Point;
use std::cmp::*;

use crate::puzzle::Puzzle;
// use crate::utils::board::*;

#[allow(unused_imports)]
use crate::utils::utils::*;

pub struct Day15 {
    target_row: i32,
    pairs: Vec<Pair>,
    max: i32,
}

type BoardPoint = Point<i32>;

#[derive(Debug)]
struct Pair {
    sensor: BoardPoint,
    beacon: BoardPoint,
    distance: i32,
    id: u32,
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

// fn draw_manhattan_radius(p: BoardPoint, dist: i32, board: &mut Board<char>) {
//     let mut x_offset = 0;
//     let y_min = p.y - dist;
//     let y_max = p.y + dist;
//     for y in y_min..p.y {
//         for x in (p.x - x_offset)..=(p.x + x_offset) {
//             board.set_at(BoardPoint { x, y }, '#');
//         }
//         x_offset += 1;
//     }

//     for y in p.y..=y_max {
//         for x in (p.x - x_offset)..=(p.x + x_offset) {
//             board.set_at(BoardPoint { x, y }, '#');
//         }
//         x_offset -= 1;
//     }
// }

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
            max: 0,
        };

        let test = input.lines().count() < 20;
        day.target_row = match test {
            true => 10,
            false => 2000000,
        };
        day.max = match test {
            true => 20,
            false => 4000000,
        };
        log::debug!("Part 1 target {}", day.target_row);
        log::debug!("Part 2 max {}", day.max);

        let mut id = 0;

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
                id,
            };
            id += 1;
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

        // let mut board = Board::<char>::new();
        // let width = max_x - min_x + 1;
        // let height = max_y - min_y + 1;
        // let offset = BoardPoint { x: min_x, y: min_y };
        // for _ in 0..height {
        //     board.push_row(vec!['.'; width as usize]);
        // }
        // for pair in pairs {
        //     board.add_player(offset_point(pair.sensor, offset), 'S');
        //     board.add_player(offset_point(pair.beacon, offset), 'B');
        //     draw_manhattan_radius(offset_point(pair.sensor, offset), pair.distance, &mut board);
        // }
        // board.print_board_with_players_pretty();

        let mut count = 0;
        for x in min_x..=max_x {
            let here = BoardPoint {
                x,
                y: self.target_row,
            };

            // Compare manhattan distance to each pair and if that's inside the distance
            let mut hash = '.';
            for pair in &self.pairs {
                let dist = manhattan_distance(here, pair.sensor);
                if pair.distance > dist {
                    count += 1;
                    hash = '#';
                    break;
                } else if pair.distance == dist && pair.beacon.x != x {
                    count += 1;
                    hash = '#';
                    break;
                }
            }
            // print!("{hash}");
        }
        println!("");

        Ok(count.to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(26.to_string()),
            false => Some(6275922.to_string()),
        }
    }

    fn solve_part2(&mut self) -> Result<String> {
        let mut perf = 0;
        let mut y = 0;
        while y <= self.max {
            let mut x = 0;
            while x <= self.max {
                let mut keep_going = false;
                let here = BoardPoint { x, y };
                for pair in self.pairs.iter() {
                    let dist = manhattan_distance(here, pair.sensor);
                    perf += 1;
                    if pair.distance >= dist {
                        keep_going = true;

                        // Try to jump to the right most unknown spot for this sensor
                        // let old_x = x;
                        // let old_y = y;
                        let y_from_sensor = (here.y - pair.sensor.y).abs();
                        let x_to_move_to = pair.sensor.x + pair.distance - y_from_sensor + 1;
                        if x_to_move_to <= self.max {
                            x = x_to_move_to;

                            let bob = manhattan_distance(BoardPoint { x, y }, pair.sensor);
                            let info = manhattan_distance(pair.sensor, pair.beacon);
                            assert_eq!(bob, pair.distance + 1);
                            // log::debug!(
                            //     "Using {}, Skipping from {old_x}, {old_y} right to {x}, {y}",
                            //     pair.id
                            // );
                            // let dist2 = manhattan_distance(BoardPoint { x, y }, pair.sensor);
                            // log::debug!("{dist2} vs {}", pair.distance);
                        } else {
                            // Skip to the start of the next line
                            x = 0;
                            y += 1;
                            // log::debug!(
                            //     "Using {}, Skipping from {old_x}, {old_y} down to {x}, {y}",
                            //     pair.id
                            // );
                        }
                        break;
                    }
                }
                // assert!(y <= self.max, "Y is toooo big");
                if !keep_going {
                    let answer = here.x as i64 * 4000000 + here.y as i64;
                    log::debug!("Perf {perf}");
                    log::debug!("{:?}", here);
                    return Ok(answer.to_string());
                }
            }
            y += 1;
        }

        Ok("to do".to_string())
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(56000011.to_string()),
            false => Some(11747175442119i64.to_string()),
        }
    }
}
