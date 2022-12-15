use anyhow::Result;
use std::cmp::*;

use crate::puzzle::Puzzle;

#[allow(unused_imports)]
use crate::utils::utils::*;

pub struct Day15 {
    target_row: i32,
    pairs: Vec<Pair>,
    max: i32,
}

#[derive(Debug)]
struct Pair {
    sensor: UtilsPoint,
    beacon: UtilsPoint,
    distance: i32,
}

fn sensor_covers_row(pair: &Pair, row: i32) -> bool {
    let minimum = pair.sensor.y - pair.distance;
    let maximum = pair.sensor.y + pair.distance;
    (minimum..=maximum).contains(&row)
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

        for line in input.lines() {
            let vals: Vec<i32> = get_vals(line);
            let sensor = UtilsPoint {
                x: vals[0],
                y: vals[1],
            };
            let beacon = UtilsPoint {
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

        let mut count = 0;
        for x in min_x..=max_x {
            let here = UtilsPoint {
                x,
                y: self.target_row,
            };

            // Compare manhattan distance to each pair and if that's inside the distance
            for pair in &self.pairs {
                let dist = manhattan_distance(here, pair.sensor);
                if pair.distance > dist {
                    count += 1;
                    break;
                } else if pair.distance == dist && pair.beacon.x != x {
                    count += 1;
                    break;
                }
            }
        }

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
                let here = UtilsPoint { x, y };
                for pair in self.pairs.iter() {
                    let dist = manhattan_distance(here, pair.sensor);
                    perf += 1;
                    if pair.distance >= dist {
                        keep_going = true;

                        // Try to jump to the right most unknown spot for this sensor
                        let y_from_sensor = (here.y - pair.sensor.y).abs();
                        let x_to_move_to = pair.sensor.x + pair.distance - y_from_sensor + 1;
                        if x_to_move_to <= self.max {
                            x = x_to_move_to;
                            break;
                        }

                        // Skip to the start of the next line
                        x = 0;
                        y += 1;
                        break;
                    }
                }
                if !keep_going {
                    let answer = here.x as i64 * 4000000 + here.y as i64;
                    log::debug!("Perf {perf}");
                    log::debug!("{:?}", here);
                    return Ok(answer.to_string());
                }
            }
            y += 1;
        }

        Ok("Unexpected result".to_string()) // Should be unreachable
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(56000011.to_string()),
            false => Some(11747175442119i64.to_string()),
        }
    }
}
