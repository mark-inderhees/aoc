// 2015 Day 19
// https://adventofcode.com/2015/day/19
// --- Day 19: Medicine for Rudolph ---

use anyhow::Result;
use std::cmp::Ordering;
use std::collections::HashSet;

use crate::puzzle::Puzzle;
use crate::utils::molecule::*;

pub struct Day19 {
    molecule: Molecule,
    replacements: Vec<Replacement>,
    replacement_strings: Vec<ReplacementString>,
    starts: Vec<Molecule>,
}

#[derive(Debug)]
struct Replacement {
    from: Atom,
    to: Molecule,
}

struct ReplacementString {
    from: String,
    to: String,
}

/// Count how many distinct molecules can be created by doing a single
/// replacement on the medicine molecule.
fn count_replacements(day: &Day19) -> usize {
    // Use hash set to prevent duplicates
    let mut count: HashSet<String> = HashSet::new();

    // Run each replacement once
    for replacement in day.replacements.iter() {
        let molecules = day.molecule.replace(&replacement.from, &replacement.to);

        // A replacement could happen at multiple locations, resulting in multiple molecules
        for molecule in molecules {
            count.insert(molecule.to_string());
        }
    }

    count.len()
}

fn find_best_replacement_path(day: &Day19) -> usize {
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

            for replacement in day.replacement_strings.iter() {
                let new_molecule = molecule.replacen(&replacement.to, &replacement.from, 1);
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
            replacement_strings: vec![],
            starts: vec![],
        };

        for line in split[0].lines() {
            let molecules: Vec<&str> = line.trim().split(" ").collect();
            if line.starts_with("e") {
                day.starts.push(Molecule::new_from_string(molecules[2]));
            } else {
                day.replacements.push(Replacement {
                    from: Atom::new(molecules[0]),
                    to: Molecule::new_from_string(molecules[2]),
                });
                day.replacement_strings.push(ReplacementString {
                    from: molecules[0].to_string(),
                    to: molecules[2].to_string(),
                })
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
            false => Some(195.to_string()),
        }
    }
}
