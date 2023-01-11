// 2015 Day 20
// https://adventofcode.com/2015/day/20
// --- Day 20: Infinite Elves and Infinite Houses ---
// Factorization!

use anyhow::Result;

use crate::puzzle::Puzzle;
use crate::utils::primes::*;
use crate::utils::utils::*;

pub struct Day20 {
    primes: Primes,
    target: usize,
}

// TODO clean this up
// fn find_presents_per_house_to_target_old(day: &mut Day20) -> usize {
//     // The number of presents for a house is like
//     // sum_of_factors * 10
//     // So house 8 is (1+2+4+8)*10 = 150
//     // sum_of_factors can also be done as prime factorization:
//     // sum_of_factors = x^n ---> sum(x^0,x^1,..,x^n)
//     // So for house 6, (1+2+3+6)*10 = 120
//     // or 2^1*3^1 -> (1+2)*(1+3) = 12  then *10=120

//     day.presents_per_hosue.push(10); // Hack for house 1 as 1 is not prime

//     for house in 2..day.target {
//         let factors = day.primes.factorization(house);
//         let mut sum_factors = 1;
//         for factor in factors.iter() {
//             let mut sum = 0;
//             for power in 0..=factor.exponent {
//                 sum += factor.base.pow(power as u32);
//             }
//             sum_factors *= sum;
//         }
//         day.presents_per_hosue.push(sum_factors * 10);
//         log::debug!("House {house} got {}", day.presents_per_hosue[house - 1]);
//         if day.presents_per_hosue[house - 1] >= day.target {
//             return house;
//         }
//     }

//     0 // TODO THis isbaaaaaaaaaad
// }

/// Find the house that receives at least the target number of presents.
/// A house recieves presents = sum_of_factors * scaler
/// Execpt with stop_after_50_houses, a factor is removed from the sum if it
/// has already been used at 50 other houses.
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
        let answer = find_target_house(self, 10, false);
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
        let answer = find_target_house(self, 11, true);
        Ok(answer.to_string())
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(6.to_string()),
            false => Some(786240.to_string()),
        }
    }
}
