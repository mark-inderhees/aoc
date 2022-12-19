use anyhow::Result;
use std::collections::HashMap;

use crate::puzzle::Puzzle;

#[allow(unused_imports)]
use crate::utils::utils::*;

pub struct Day16 {
    valves: HashMap<String, Valve>,
}

#[derive(Debug, Clone)]
struct Valve {
    id: String,
    rate: u32,
    connections: Vec<String>,
    on: bool,
    distances: HashMap<String, u32>,
}

// Find the distance between two valves
fn distance(valves: &HashMap<String, Valve>, start: &str, end: &str) -> u32 {
    struct PathWork {
        id: String,
        count: u32,
    }
    let mut jobs = vec![PathWork {
        id: start.to_string(),
        count: 0,
    }];
    let mut shortest_path = u32::MAX;
    let mut step_count_map: HashMap<String, u32> = HashMap::new();

    while jobs.len() > 0 {
        let job = jobs.pop().unwrap();

        // Check if this count is already too long
        if job.count >= shortest_path {
            continue;
        }

        // Check if we've ever been here at a more optimized path
        if let Some(step_count) = step_count_map.get(&job.id) {
            if job.count >= *step_count {
                continue;
            }
        }
        step_count_map.insert(job.id.clone(), job.count);

        // Try all new locations
        for new_id in &valves[&job.id].connections {
            // Check if we are done
            if *new_id == end {
                let final_count = job.count + 1;
                log::trace!("{} THIS IS THE END = {}", new_id, final_count);
                if final_count < shortest_path {
                    shortest_path = final_count;
                }
                continue;
            }

            // We can move, so do it!
            jobs.push(PathWork {
                id: new_id.to_string(),
                count: job.count + 1,
            });
        }
    }
    log::trace!("Shortest path from {start} to {end} is {shortest_path}");

    shortest_path
}

// Try all combinations of paths to figure out the best score
// Returns a map of all best state found, this includes intermediate state
fn highest_score(day: &Day16, total_time: u32) -> HashMap<Vec<String>, u32> {
    // Add first job
    #[derive(Clone)]
    struct PathWork {
        id: String,
        time_left: u32,
        turned_on: Vec<String>,
        total_flow: u32,
    }
    let mut jobs = vec![PathWork {
        id: "AA".to_string(), // Start at AA
        time_left: total_time,
        turned_on: vec![],
        total_flow: 0,
    }];

    // Store best scores for all intermediate state
    let mut answers: HashMap<Vec<String>, u32> = HashMap::new();

    while jobs.len() > 0 {
        let mut job = jobs.pop().unwrap();

        // Always turn on if not start
        if job.id != "AA" {
            // Turn valve on
            job.turned_on.push(job.id.to_string());
            job.total_flow += day.valves[&job.id].rate * job.time_left;

            // Build state map, keeping max score at this intermediate state
            job.turned_on.sort();
            let best_answer = *answers.get(&job.turned_on).or_else(|| Some(&0)).unwrap();
            answers.insert(
                job.turned_on.clone(),
                std::cmp::max(best_answer, job.total_flow),
            );
        }

        for (new_id, dist) in &day.valves[&job.id].distances {
            // Dont go places we have been
            if job.turned_on.contains(&new_id) {
                continue;
            }

            // Create new job
            let mut new_job = PathWork {
                id: new_id.to_string(),
                time_left: job.time_left,
                turned_on: job.turned_on.clone(),
                total_flow: job.total_flow,
            };

            // Tick time, stop if we are done
            new_job.time_left -= std::cmp::min(*dist, new_job.time_left);
            if new_job.time_left == 0 {
                continue;
            }

            jobs.push(new_job.clone());
        }
    }

    answers
}

impl Puzzle for Day16 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut day = Day16 {
            valves: HashMap::new(),
        };

        for line in input.lines() {
            let rate = get_val(line);
            let line2 = line.replace(",", "");
            let split: Vec<&str> = line2.split(" ").collect();
            let id = split[1].to_string();
            let on = match rate {
                0 => true,
                _ => false,
            };
            let mut valve = Valve {
                id: id.clone(),
                rate,
                connections: vec![],
                on,
                distances: HashMap::new(),
            };
            for connection in split.iter().skip(9) {
                valve.connections.push(connection.to_string());
            }
            day.valves.insert(id, valve.clone());
        }

        // Calculate all distances between points we care about
        let copy = day.valves.clone();
        for start in day.valves.values_mut() {
            if !start.on || start.id == "AA" {
                for end in copy.values() {
                    if !end.on && start.id != end.id {
                        let distance = distance(&copy, &start.id, &end.id);
                        start.distances.insert(end.id.clone(), distance + 1);
                    }
                }
            }
        }

        log::debug!("{:#?}", day.valves);

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        let answers = highest_score(self, 30);
        let score = answers.values().max().unwrap();

        Ok(score.to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(1651.to_string()),
            false => Some(1792.to_string()),
        }
    }

    fn solve_part2(&mut self) -> Result<String> {
        let answers = highest_score(self, 26);
        let mut scores = vec![];

        // Find the highest score for two workers, where there is no intersection
        for (key1, value1) in answers.iter() {
            for (key2, value2) in answers.iter() {
                // Test for worker path intersection
                if !key1.iter().any(|x| key2.contains(x)) {
                    scores.push(*value1 + *value2);
                }
            }
        }
        let score = scores.iter().max().unwrap();

        return Ok(score.to_string());
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(1707.to_string()),
            false => Some(2587.to_string()),
        }
    }
}
