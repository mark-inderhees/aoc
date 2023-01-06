// 2015 Day 17
// https://adventofcode.com/2015/day/17
// --- Day 17: No Such Thing as Too Much ---
// Find combinations that sum up to a specific capacity

use anyhow::Result;

use crate::puzzle::Puzzle;
use crate::utils::utils::*;

pub struct Day17 {
    containers: Vec<Container>,
    target: u32,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct Container {
    id: u32,
    capacity: u32,
}

/// Find how many combinations sum to the capacity. Also return how many use the fewest containers.
fn find_combinations(day: &Day17) -> (u32, u32) {
    // Find all combinations that add up to target
    struct Work {
        used: Vec<Container>,
        available: Vec<Container>,
    }
    let mut jobs = vec![Work {
        used: vec![],
        available: day.containers.clone(),
    }];

    // Store list of good combinations
    let mut good = vec![];

    while jobs.len() > 0 {
        let mut job = jobs.pop().unwrap();
        let sum: u32 = job.used.iter().fold(0, |a, c| a + c.capacity);
        if sum == day.target {
            // This is a good one
            log::debug!("Found good combo {:?} = {sum}", job.used);
            good.push(job.used);
            continue;
        } else if sum > day.target {
            // This is no good
            continue;
        }

        // Try new combinations
        while job.available.len() > 0 {
            let value = job.available.pop().unwrap();
            let mut new_job = Work {
                used: job.used.clone(),
                available: job.available.clone(),
            };
            new_job.used.push(value);
            jobs.push(new_job);
        }
    }

    // Count of valid combinations
    let count = good.len() as u32;

    // Count of combinations that have fewest containers
    let mut count2 = 0;
    let mut min = usize::MAX;
    for answer in good.iter() {
        min = std::cmp::min(min, answer.len());
    }
    for answer in good.iter() {
        if answer.len() == min {
            count2 += 1;
        }
    }

    (count, count2)
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
        // Find number of good combinations
        let answer = find_combinations(self).0;
        Ok(answer.to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(4.to_string()),
            false => Some(4372.to_string()),
        }
    }

    fn solve_part2(&mut self) -> Result<String> {
        // Find number of good combinations that use fewest containers
        let answer = find_combinations(self).1;
        Ok(answer.to_string())
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(3.to_string()),
            false => Some(4.to_string()),
        }
    }
}
