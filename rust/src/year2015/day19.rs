// 2015 Day 19
// https://adventofcode.com/2015/day/19
// --- Day 19: Medicine for Rudolph ---

use anyhow::Result;

use crate::puzzle::Puzzle;
use crate::utils::molecule::*;

use std::cmp::Ordering;

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

fn find_best_replacement_path(day: &Day19) -> usize {
    // Build list of replacements
    let mut replacements: HashMap<String, String> = HashMap::new();
    for replacement in day.replacements.iter() {
        if replacements.contains_key(&replacement.to.to_string()) {
            panic!("Replacement collision");
        }
        replacements.insert(replacement.to.to_string(), replacement.from.to_string());
    }

    let mut starts = vec![];
    for start in day.starts.iter() {
        starts.push(start.to_string());
    }

    let mut best = usize::MAX;
    let mut molecules = vec![day.molecule.to_string()];

    loop {
        best += 1;

        let mut new_molecules = vec![];
        for molecule in molecules.iter() {
            if starts.contains(&molecule) {
                log::info!("Found path after {} steps", best);
                return best + 1;
            }

            for (key, value) in replacements.iter() {
                let new_molecule = molecule.replacen(key, value, 1);
                new_molecules.push(new_molecule);
            }
        }

        molecules = new_molecules;
        molecules.sort_by(|a, b| {
            if a.len() > b.len() {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        });
        molecules.dedup();
        if molecules.len() > 100 {
            molecules = molecules.split_at(100).0.to_vec();
        }
        log::debug!("Round {best} and molecules {}", molecules.len());
    }
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
