// 2015 Day 14
// https://adventofcode.com/2015/day/14

use anyhow::Result;

use crate::puzzle::Puzzle;

#[allow(unused_imports)]
use crate::utils::utils::*;

#[allow(unused_imports)]
use std::collections::VecDeque;

pub struct Day14 {
    raindeers: Vec<Raindeer>,
}

#[derive(Debug)]
struct Raindeer {
    #[allow(dead_code)]
    name: String,
    speed: u32,
    duration: u32,
    rest: u32,
}

/// Calculate how for a raindeer has flown after a given time
fn distance_after_time(raindeer: &Raindeer, time: u32) -> u32 {
    let full_time_interval = raindeer.duration + raindeer.rest;
    let full_intervals = time / full_time_interval;
    let partial_time = std::cmp::min(time % full_time_interval, raindeer.duration);
    let distance =
        full_intervals * raindeer.speed * raindeer.duration + raindeer.speed * partial_time;
    distance
}

impl Puzzle for Day14 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut day = Day14 { raindeers: vec![] };

        for line in input.lines() {
            // Input looks like
            // Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
            let clean = line.trim().replace(".", "");
            let splits: Vec<&str> = clean.split(" ").collect();
            let values: Vec<u32> = find_vals(line);
            let name = splits[0].to_string();
            let speed = values[0];
            let duration = values[1];
            let rest = values[2];
            day.raindeers.push(Raindeer {
                name,
                speed,
                duration,
                rest,
            });
        }

        log::debug!("{:#?}", day.raindeers);

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        let mut max = 0;
        for raindeer in self.raindeers.iter() {
            max = std::cmp::max(max, distance_after_time(raindeer, 2503));
        }
        Ok(max.to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(2660.to_string()),
            false => Some(2660.to_string()),
        }
    }

    fn solve_part2(&mut self) -> Result<String> {
        let mut points = vec![0; self.raindeers.len()];
        for time in 1..=2503 {
            let mut max = 0;
            let mut max_i = vec![];
            for (i, raindeer) in self.raindeers.iter().enumerate() {
                let distance = distance_after_time(raindeer, time);
                if distance == max {
                    max_i.push(i);
                } else if distance > max {
                    max_i.clear();
                    max_i.push(i);
                    max = distance;
                }
            }

            for i in max_i {
                points[i] += 1;
            }
        }
        let best = points.iter().max().unwrap();
        Ok(best.to_string())
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(1564.to_string()),
            false => Some(1256.to_string()),
        }
    }
}
