// 2015 Day 19
// https://adventofcode.com/2015/day/19
// --- Day 19: Medicine for Rudolph ---

use anyhow::Result;

use crate::puzzle::Puzzle;
use crate::utils::molecule::*;

use std::collections::HashMap;

pub struct Day19 {
    molecule: Molecule,
    replacements: Vec<Replacement>,
    starts: Vec<Molecule>,
}

#[derive(Debug)]
struct Replacement {
    from: Atom,
    to: Molecule,
}

fn count_replacements(day: &Day19) -> usize {
    let mut count: HashMap<String, bool> = HashMap::new();
    let mut count2 = 0;

    for replacement in day.replacements.iter() {
        let molecules = day.molecule.replace(&replacement.from, &replacement.to);
        for molecule in molecules {
            count.insert(molecule.to_string(), true);
            count2 += 1;
        }
    }

    log::debug!("Counts {} vs {count2}", count.len());

    count.len()
}

fn find_best_replacement_path(day: &Day19) -> u32 {
    let mut best = u32::MAX;
    let max_len = day.molecule.len();

    struct Work {
        molecule: Molecule,
        steps: u32,
    }
    let mut jobs = vec![];

    for start in day.starts.iter() {
        jobs.push(Work {
            molecule: start.clone(),
            steps: 1,
        });
    }

    while jobs.len() > 0 {
        let job = jobs.pop().unwrap();

        if job.molecule.len() > max_len {
            continue;
        }

        if job.steps >= best {
            continue;
        }

        if job.molecule == day.molecule {
            log::debug!("Built correct molecule after {} steps", job.steps);
            best = std::cmp::min(best, job.steps);
        }

        for replacement in day.replacements.iter() {
            let molecules = job.molecule.replace(&replacement.from, &replacement.to);
            for molecule in molecules {
                jobs.push(Work {
                    molecule,
                    steps: job.steps + 1,
                })
            }
        }
    }

    best
}

impl Puzzle for Day19 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        let split: Vec<&str> = input.trim().split("\n\n").collect();

        #[allow(unused_mut)]
        let mut day = Day19 {
            molecule: Molecule::new_from_string(split[1].trim()),
            replacements: vec![],
            starts: vec![],
        };

        for line in split[0].lines() {
            let molecules: Vec<&str> = line.split(" ").collect();
            if line.starts_with("e") {
                day.starts.push(Molecule::new_from_string(molecules[2]));
            } else {
                day.replacements.push(Replacement {
                    from: Atom::new(molecules[0]),
                    to: Molecule::new_from_string(molecules[2].trim()),
                });
            }
        }

        log::debug!("{}, {}", day.molecule.to_string(), day.molecule.len());
        log::trace!("{:#?}", day.replacements);

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        let answer = count_replacements(self);
        Ok(answer.to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(7.to_string()),
            false => Some(509.to_string()),
        }
    }

    fn solve_part2(&mut self) -> Result<String> {
        let answer = find_best_replacement_path(self);
        Ok(answer.to_string())
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(6.to_string()),
            false => None,
        }
    }
}
