// 2015 Day 9
// https://adventofcode.com/2015/day/9
// --- Day 9: All in a Single Night ---
// Find best (and worst!) paths given distances between cities.

use anyhow::Result;
use std::collections::HashMap;

use crate::puzzle::Puzzle;
use crate::utils::utils::*;

pub struct Day09 {
    locations: HashMap<String, HashMap<String, u32>>,
}

/// Find the shortest and longest distance it takes to visit all of the cities
fn find_distances(day: &Day09) -> (u32, u32) {
    // Try all permutations of cities
    let cities: Vec<String> = day.locations.keys().map(|x| x.to_string()).collect();
    struct Work {
        city: String,
        visited: Vec<String>,
        distance: u32,
    }
    let mut jobs = vec![];

    // Init work list with each city as a potential start point
    for city in cities.iter() {
        let job = Work {
            city: city.to_string(),
            visited: vec![city.to_string()],
            distance: 0,
        };
        jobs.push(job)
    }

    // Find shortest and longest routes
    let mut shortest = u32::MAX;
    let mut longest = 0;

    while jobs.len() > 0 {
        let job = jobs.pop().unwrap();

        // Visit new cities, only if we have not been there yet
        let mut did_new_work = false;
        for city in cities.iter() {
            if !job.visited.contains(city) {
                // Update distance and visited list, then schedule new work
                let map = day.locations.get(&job.city).unwrap();
                let distance = map.get(city).unwrap() + job.distance;
                let mut visited = job.visited.clone();
                visited.push(city.to_string());
                let job = Work {
                    city: city.to_string(),
                    visited,
                    distance,
                };
                jobs.push(job);
                did_new_work = true;
            }
        }

        // Check if this route is done, then update return values
        if !did_new_work {
            shortest = std::cmp::min(shortest, job.distance);
            longest = std::cmp::max(longest, job.distance);
        }
    }

    (shortest, longest)
}

impl Puzzle for Day09 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut day = Day09 {
            locations: HashMap::new(),
        };

        // Input is a list of cites with distances like
        // city_a to city_b = <number>
        for line in input.lines() {
            let distance: u32 = find_val(line);

            // Get city names
            let splits: Vec<&str> = line.split(" ").collect();
            let from = splits[0].to_string();
            let to = splits[2].to_string();

            // Add this distance to the map
            let map = day.locations.entry(from.clone()).or_insert(HashMap::new());
            map.insert(to.clone(), distance);

            // Also insert the other direction
            let map = day.locations.entry(to).or_insert(HashMap::new());
            map.insert(from, distance);
        }

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        // Find shortest distance
        let answer = find_distances(self).0;
        Ok(answer.to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(605.to_string()),
            false => Some(207.to_string()),
        }
    }

    fn solve_part2(&mut self) -> Result<String> {
        // Find longest distance
        let answer = find_distances(self).1;
        Ok(answer.to_string())
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(982.to_string()),
            false => Some(804.to_string()),
        }
    }
}
