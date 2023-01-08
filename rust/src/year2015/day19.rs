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
    starts: Vec<String>,
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
            log::debug!(
                "Replacement {replacement:?} made new molecule {}",
                molecule.to_string()
            );
            if count.contains_key(&molecule.to_string()) {
                log::debug!("Molecule already exists");
            }
            count.insert(molecule.to_string(), true);
            count2 += 1;
        }
    }

    // log::debug!("{:#?}", count);
    log::info!("Counts {} vs {count2}", count.len());

    count.len()
}

fn find_best_replacement_path(day: &Day19) -> usize {
    let mut best = usize::MAX;

    for start in day.starts.iter() {}

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
                day.starts.push(molecules[2].to_string());
            } else {
                day.replacements.push(Replacement {
                    from: Atom::new(molecules[0]),
                    to: Molecule::new_from_string(molecules[2].trim()),
                });
            }
        }

        log::debug!("{}", day.molecule.to_string());
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
