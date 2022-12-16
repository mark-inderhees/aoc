use std::collections::HashMap;

use anyhow::Result;

use crate::puzzle::Puzzle;

#[allow(unused_imports)]
use crate::utils::utils::*;

// PART 1 WOOOOOORKS

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

#[derive(Clone)]
struct PathWork {
    id: String,
    time_left: u32,
    time_passed: u32,
    score: u32,
    rate: u32,
    turned_on: Vec<String>,
}

fn tick(job: &mut PathWork, time: u32) -> bool {
    let to_tick = std::cmp::min(time, job.time_left);

    job.score += job.rate * to_tick;
    job.time_left -= to_tick;
    job.time_passed += to_tick;

    job.time_left == 0
}

fn finalize(job: &PathWork, highest_score: &mut u32) {
    if job.score > *highest_score {
        *highest_score = job.score;
    }
}

fn highest_score(day: &Day16) -> u32 {
    let mut jobs = vec![PathWork {
        id: "AA".to_string(),
        time_left: 30,
        time_passed: 1,
        score: 0,
        rate: 0,
        turned_on: vec![],
    }];
    let mut highest_score = 0;

    while jobs.len() > 0 {
        let mut job = jobs.pop().unwrap();

        // If this is turned on, then leave
        if job.turned_on.contains(&job.id) {
            let done = tick(&mut job, u32::MAX);
            assert!(done);
            if done {
                finalize(&job, &mut highest_score);
                continue;
            }
        }

        // Always turn on if not start
        if job.id != "AA" {
            let done = tick(&mut job, 1);
            if done {
                finalize(&job, &mut highest_score);
                continue;
            }

            // Valve is on
            job.turned_on.push(job.id.to_string());
            job.rate += day.valves[&job.id].rate;
        }

        for (new_id, dist) in &day.valves[&job.id].distances {
            let mut new_job = PathWork {
                id: new_id.to_string(),
                time_left: job.time_left,
                time_passed: job.time_passed,
                score: job.score,
                rate: job.rate,
                turned_on: job.turned_on.clone(),
            };

            let done = tick(&mut new_job, *dist);
            if done {
                finalize(&new_job, &mut highest_score);
                continue;
            }

            jobs.push(new_job.clone());
        }
    }

    highest_score
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
                        start.distances.insert(end.id.clone(), distance);
                    }
                }
            }
        }

        log::debug!("{:#?}", day.valves);

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        let score = highest_score(self);

        Ok(score.to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(1651.to_string()),
            false => Some(1792.to_string()),
        }
    }

    fn solve_part2(&mut self) -> Result<String> {
        let score = 0;

        Ok(score.to_string())
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(1707.to_string()),
            false => None,
        }
    }
}
