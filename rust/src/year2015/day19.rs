// 2015 Day 19
// https://adventofcode.com/2015/day/19
// --- Day 19: Medicine for Rudolph ---
// Given a molecule made up of atoms, do replacements to calculate best way
// to make the molecule.

use anyhow::Result;
use std::cmp::Ordering;
use std::collections::HashSet;

use crate::puzzle::Puzzle;
use crate::utils::molecule::*;

pub struct Day19 {
    /// The target molecule, made up of many atoms.
    molecule: Molecule,

    /// Replacements convert from one atom to multiple atoms (a molecule).
    replacements: Vec<Replacement>,

    /// Also have a string version of replacements for easy string matching.
    replacement_strings: Vec<ReplacementString>,

    /// Molecules can start with a couple of options.
    starts: Vec<String>,
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
/// replacement on the target molecule.
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

/// Find the fewest number of replacements needed to go from a start molecule
/// to the target molecule by using replacements.
fn find_best_replacement_path(day: &Day19) -> usize {
    // Instead of brute force path finding by starting form an initial molecule
    // and trying to build the target molecule, go from the target molecule and
    // shrink it to a start molecule.
    let mut steps = usize::MAX;

    // Start with the target and shrink it
    let mut molecules = vec![day.molecule.to_string()];

    loop {
        steps += 1;
        let mut new_molecules = vec![];

        // We keep state of multiple paths simultaneously, check each path at
        // this step instance.
        for molecule in molecules.iter() {
            // Check if done
            if day.starts.contains(&molecule) {
                steps += 1; // Need to count the initial e -> molecule step
                log::info!("Found path after {} steps", steps);
                return steps;
            }

            // Find all new molecules by doing a single reverse replacement
            for replacement in day.replacement_strings.iter() {
                let new_molecule = molecule.replacen(&replacement.to, &replacement.from, 1);
                new_molecules.push(new_molecule);
            }
        }

        // Keep only the best paths to prevent exponential growth. Aggressively
        // trim down the size, so keep only the 100 shortest paths.
        molecules = new_molecules;
        molecules.sort_by(|a, b| {
            if a == b {
                Ordering::Equal
            } else if a.len() > b.len() {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        });
        molecules.dedup();
        if molecules.len() > 100 {
            molecules = molecules.split_at(100).0.to_vec();
        }
        log::debug!("Round {steps} and molecules {}", molecules.len());
    }
}

impl Puzzle for Day19 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        // Input has the replacements listed first, then target molecule at the bottom
        let split: Vec<&str> = input.trim().split("\n\n").collect();

        #[allow(unused_mut)]
        let mut day = Day19 {
            molecule: Molecule::new_from_string(split[1].trim()),
            replacements: vec![],
            replacement_strings: vec![],
            starts: vec![],
        };

        for line in split[0].lines() {
            // Replacement lines look like
            // Ti => TiTi
            // e => HF
            let molecules: Vec<&str> = line.trim().split(" ").collect();
            if line.starts_with("e") {
                // This is a start
                day.starts.push(molecules[2].to_string());
            } else {
                // This is a replacement, store as a Replacement struct and basic strings
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
