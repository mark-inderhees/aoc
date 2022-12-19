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
    total_flow_from_on_vavles: u32,
}

// Move time based on how far moved and increase score
fn tick(job: &mut PathWork, time: u32) -> bool {
    let to_tick = std::cmp::min(time, job.time_left);

    job.score += job.rate * to_tick;
    job.time_left -= to_tick;
    job.time_passed += to_tick;

    job.time_left == 0
}

// Update overall highest score
fn finalize(job: &PathWork, highest_score: &mut u32) {
    if job.score > *highest_score {
        *highest_score = job.score;
    }
}

// Try all combinations of paths to figure out the best score
// Returns a map of all best state found, this includes intermediate state
fn highest_score(day: &Day16, total_time: u32) -> HashMap<Vec<String>, u32> {
    let mut jobs = vec![PathWork {
        id: "AA".to_string(), // Start at AA
        time_left: total_time,
        time_passed: 1,
        score: 0,
        rate: 0,
        turned_on: vec![],
        total_flow_from_on_vavles: 0, // Alternative count, instead of incrementing score on ticks
    }];
    let mut highest_score = 0;
    let mut answers: HashMap<Vec<String>, u32> = HashMap::new();

    while jobs.len() > 0 {
        let mut job = jobs.pop().unwrap();

        // Always turn on if not start
        if job.id != "AA" {
            // Turn valve on
            job.turned_on.push(job.id.to_string());
            job.rate += day.valves[&job.id].rate;

            // Build state map, keeping max score at this intermediate state
            job.turned_on.sort();
            let best_answer = match answers.get(&job.turned_on) {
                Some(x) => *x,
                None => 0,
            };
            job.total_flow_from_on_vavles =
                job.total_flow_from_on_vavles + day.valves[&job.id].rate * job.time_left;
                answers.insert(
                job.turned_on.clone(),
                std::cmp::max(best_answer, job.total_flow_from_on_vavles),
            );
        }

        for (new_id, dist) in &day.valves[&job.id].distances {
            // Dont go places we have been
            if job.turned_on.contains(&new_id) {
                continue;
            }

            let mut new_job = PathWork {
                id: new_id.to_string(),
                time_left: job.time_left,
                time_passed: job.time_passed,
                score: job.score,
                rate: job.rate,
                turned_on: job.turned_on.clone(),
                total_flow_from_on_vavles: job.total_flow_from_on_vavles,
            };

            let done = tick(&mut new_job, *dist);
            if done {
                finalize(&new_job, &mut highest_score);
                continue;
            }

            jobs.push(new_job.clone());
        }
    }

    answers
}

#[allow(dead_code)]
fn tick_p2(job: &mut PathWorkP2) -> bool {
    // let to_tick = std::cmp::min(time, job.time_left);
    let mut to_tick = job.time_left;
    if !job.p1_done {
        to_tick = std::cmp::min(job.p1_dist, to_tick);
    }

    if !job.p2_done {
        to_tick = std::cmp::min(job.p2_dist, to_tick);
    }

    // assert_ne!(to_tick, 0);

    job.score += job.rate * to_tick;
    job.time_left -= to_tick;
    job.time_passed += to_tick;

    if !job.p1_done {
        job.p1_dist -= to_tick;
    }
    if !job.p2_done {
        job.p2_dist -= to_tick;
    }

    job.time_left == 0
}

#[allow(dead_code)]
fn finalize_p2(job: &PathWorkP2, highest_score: &mut u32) {
    log::debug!("Part 2 all done {}", job.score);
    if job.score > *highest_score {
        *highest_score = job.score;
    }
}

#[derive(Clone)]
#[allow(dead_code)]
struct PathWorkP2 {
    p1_id: String,
    p2_id: String,
    p1_dist: u32,
    p2_dist: u32,
    p1_done: bool,
    p2_done: bool,
    time_left: u32,
    time_passed: u32,
    score: u32,
    rate: u32,
    turned_on: Vec<String>,
}

#[allow(dead_code)]
fn highest_score_p2(day: &Day16) -> u32 {
    // THIS P2 does NOT WORK
    // It's CLOSE on the test data, but too low by 2
    // For the input data, it runs forever :'(
    let mut jobs = vec![PathWorkP2 {
        p1_id: "AA".to_string(),
        p2_id: "AA".to_string(),
        p1_dist: 0,
        p2_dist: 0,
        p1_done: false,
        p2_done: false,
        time_left: 26,
        time_passed: 1,
        score: 0,
        rate: 0,
        turned_on: vec![],
    }];
    let mut highest_score = 0;
    // let mut high_score_map: HashMap<u32, u32> = HashMap::new();
    // for i in (0..=26).rev() {
    //     high_score_map.insert(i, 0);
    // }

    while jobs.len() > 0 {
        let mut job = jobs.pop().unwrap();

        // let best_so_far = high_score_map[&job.time_left];
        // if best_so_far > job.score {
        //     // Quit this path
        //     continue;
        // }
        // // Update high score map
        // for i in (job.time_left..=26).rev() {
        //     if job.score > high_score_map[&i] {
        //         high_score_map.insert(i, job.score);
        //     }
        // }

        // Turn on if not start and player dist == 0
        if job.p1_id != "AA" && job.p1_dist == 0 && !job.turned_on.contains(&job.p1_id) {
            log::debug!("P1 opened {}", job.p1_id);
            job.turned_on.push(job.p1_id.to_string());
            job.rate += day.valves[&job.p1_id].rate;
        }

        if job.p2_id != "AA" && job.p2_dist == 0 && !job.turned_on.contains(&job.p2_id) {
            log::debug!("P2 opened {}", job.p2_id);
            job.turned_on.push(job.p2_id.to_string());
            job.rate += day.valves[&job.p2_id].rate;
        }

        // Only do new work if ready
        let potential_p1_targets = match job.p1_dist {
            0 => day.valves[&job.p1_id].distances.clone(),
            _ => HashMap::from([(job.p1_id.clone(), job.p1_dist)]),
        };

        let potential_p2_targets = match job.p2_dist {
            0 => day.valves[&job.p2_id].distances.clone(),
            _ => HashMap::from([(job.p2_id.clone(), job.p2_dist)]),
        };

        let mut did_new_work = false;
        for (new_id_p1, dist_p1) in &potential_p1_targets {
            for (new_id_p2, dist_p2) in &potential_p2_targets {
                // If code does not work, then remove the potential_pN_targets.len() part of logic
                if job.turned_on.contains(&new_id_p1) && potential_p1_targets.len() > 1 {
                    continue;
                } else if job.turned_on.contains(&new_id_p2) && potential_p2_targets.len() > 1 {
                    continue;
                }

                did_new_work = true;

                log::debug!(
                    "New job: P1 {}->{}, P2 {}->{}",
                    job.p1_id.clone(),
                    new_id_p1,
                    job.p2_id.clone(),
                    new_id_p2
                );
                let mut new_job = PathWorkP2 {
                    p1_id: new_id_p1.to_string(),
                    p2_id: new_id_p2.to_string(),
                    p1_dist: *dist_p1,
                    p2_dist: *dist_p2,
                    p1_done: job.p1_done,
                    p2_done: job.p2_done,
                    time_left: job.time_left,
                    time_passed: job.time_passed,
                    score: job.score,
                    rate: job.rate,
                    turned_on: job.turned_on.clone(),
                };

                let done = tick_p2(&mut new_job);
                if done {
                    finalize_p2(&new_job, &mut highest_score);
                    continue;
                }

                jobs.push(new_job.clone());
            }
        }

        if !did_new_work {
            // End the game
            let done = tick_p2(&mut job);
            if done {
                finalize_p2(&job, &mut highest_score);
                continue;
            }

            if job.p1_dist == 0 {
                job.p1_done = true;
            }
            if job.p2_dist == 0 {
                job.p2_done = true;
            }

            jobs.push(job.clone());
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
