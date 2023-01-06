// 2015 Day 17
// https://adventofcode.com/2015/day/17

use anyhow::Result;

use crate::puzzle::Puzzle;

#[allow(unused_imports)]
use crate::utils::utils::*;

use std::collections::HashMap;
#[allow(unused_imports)]
use std::collections::VecDeque;

pub struct Day17 {
    containers: Vec<Container>,
    target: u32,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct Container {
    id: u32,
    capacity: u32,
}

fn find_combinations(day: &Day17) -> u32 {
    // Find all combinations that add up to target
    struct Work {
        used: Vec<Container>,
        available: Vec<Container>,
    }
    let mut jobs = vec![Work {
        used: vec![],
        available: day.containers.clone(),
    }];

    let mut count = 0;
    // let mut good = HashMap::new();

    while jobs.len() > 0 {
        let mut job = jobs.pop().unwrap();
        let sum: u32 = job.used.iter().fold(0, |a, c| a + c.capacity);
        if sum == day.target {
            // This is a good one
            job.used.sort();
            log::debug!("Found good combo {:?} = {sum}", job.used);
            count += 1;
            // let capacities: Vec<String> = job
            //     .used
            //     .iter()
            //     .map(|c| format!("{}:{}", c.id, c.capacity))
            //     .collect();
            // log::info!("Found good {} vs {}, {:?}", count, good.len(), capacities);
            // good.insert(job.used, true);
            continue;
        } else if sum > day.target {
            // This is no good
            continue;
        }

        while job.available.len() > 0 {
            let value = job.available.pop().unwrap();
            let mut new_job = Work {
                used: job.used.clone(),
                available: job.available.clone(),
            };
            new_job.used.push(value.clone());
            jobs.push(new_job);
        }
    }

    count
    // good.len() as u32
}

impl Puzzle for Day17 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut day = Day17 {
            containers: vec![],
            target: 0,
        };

        for (i, line) in input.lines().enumerate() {
            day.containers.push(Container {
                id: i as u32,
                capacity: find_val(line),
            });
        }

        day.target = if day.containers.len() > 10 { 150 } else { 25 };

        log::debug!("{:#?}", day.containers);

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        let answer = find_combinations(self);
        Ok(answer.to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(4.to_string()),
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
