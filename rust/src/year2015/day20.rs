// 2015 Day 20
// https://adventofcode.com/2015/day/20
// --- Day 20: Infinite Elves and Infinite Houses ---
// Factorization! Or is it? Factorization worked but ended up being a little
// slow, so this turned into an optimization question.

use anyhow::Result;

use crate::puzzle::Puzzle;
use crate::utils::primes::*;
use crate::utils::utils::*;

pub struct Day20 {
    primes: Primes,
    target: usize,
}

/// NOTE this is unused as it's a little slow, 5 seconds to answer.
/// But I asume the idea was to use factorization, so I'll keep the logic.
/// Find the house that receives at least the target number of presents.
/// A house recieves presents = sum_of_factors * scaler
/// Execpt with stop_after_50_houses, a factor is removed from the sum if it
/// has already been used at 50 other houses.
#[allow(dead_code)]
fn find_target_house(day: &mut Day20, scaler: usize, stop_after_50_houses: bool) -> usize {
    // House present calculation is sum_of_factors * scaler
    // So house 8 is (1+2+4+8)*10 = 150

    // Iterate over all houses until the target present count is found
    for house in 1..day.target {
        // Get the factors for this house number
        let factors = day.primes.all_factors(house);

        // Sum up the factors
        let mut sum_factors = 0;
        for factor in factors.iter() {
            if stop_after_50_houses && house / factor > 50 {
                continue;
            }
            sum_factors += factor;
        }

        // Check if this is the target house
        let presents = sum_factors * scaler;
        log::debug!("House {house} got {}", presents);
        if presents >= day.target {
            return house;
        }
    }

    panic!("Did not find target house");
}

/// Find the house that receives at least the target number of presents.
fn find_target_house_via_elf(day: &mut Day20, scaler: usize, stop_after_50_houses: bool) -> usize {
    // A little bit of optimization magic, but max = day.target is still pretty fast
    let max = 1_000_000;

    let mut presents_at_house = vec![0; max];

    // Instead of iterting over houses, iterate over elfs.
    // This ends up being faster than trying to find factors as the logic can
    // step by a known value instead of hunting for factors.
    for elf in 1..max {
        let mut houses_visited = 0;
        for house in (elf..max).step_by(elf) {
            presents_at_house[house] += elf * scaler;

            houses_visited += 1;
            if stop_after_50_houses && houses_visited >= 50 {
                break;
            }
        }

        // Check if this is the target house
        if presents_at_house[elf] >= day.target {
            return elf;
        }
    }

    panic!("Did not find target house");
}

impl Puzzle for Day20 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        // Input is a simple number target
        let target: usize = find_val(input.trim());

        #[allow(unused_mut)]
        let mut day = Day20 {
            primes: Primes::new(target),
            target,
        };

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        // Find target house, using scaler 10 and infinite elf present delivery
        // let answer = find_target_house(self, 10, false);
        let answer = find_target_house_via_elf(self, 10, false);
        Ok(answer.to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(8.to_string()),
            false => Some(776160.to_string()),
        }
    }

    fn solve_part2(&mut self) -> Result<String> {
        // Find target house, using scaler 11 and elfs stop after 50 houses
        // let answer = find_target_house(self, 11, true);
        let answer = find_target_house_via_elf(self, 11, true);
        Ok(answer.to_string())
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(6.to_string()),
            false => Some(786240.to_string()),
        }
    }
}
