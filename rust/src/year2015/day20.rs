// 2015 Day 20
// https://adventofcode.com/2015/day/20

use anyhow::Result;

use crate::puzzle::Puzzle;
use crate::utils::primes::*;

#[allow(unused_imports)]
use crate::utils::utils::*;

#[allow(unused_imports)]
use std::collections::VecDeque;

pub struct Day20 {
    primes: Primes,
    target: usize,
    presents_per_hosue: Vec<usize>,
}

fn find_presents_per_house_to_target(day: &mut Day20) -> usize {
    // The number of presents for a house is like
    // sum_of_factors * 10
    // So house 8 is (1+2+4+8)*10 = 150
    // sum_of_factors can also be done as prime factorization:
    // sum_of_factors = x^n ---> sum(x^0,x^1,..,x^n)
    // So for house 6, (1+2+3+6)*10 = 120
    // or 2^1*3^1 -> (1+2)*(1+3) = 12  then *10=120

    day.presents_per_hosue.push(10); // Hack for house 1 as 1 is not prime

    for house in 2..day.target {
        let factors = day.primes.factorization(house);
        let mut sum_factors = 1;
        for factor in factors.iter() {
            let mut sum = 0;
            for power in 0..=factor.exponent {
                sum += factor.base.pow(power as u32);
            }
            sum_factors *= sum;
        }
        day.presents_per_hosue.push(sum_factors * 10);
        log::debug!("House {house} got {}", day.presents_per_hosue[house - 1]);
        if day.presents_per_hosue[house - 1] >= day.target {
            return house;
        }
    }

    0 // TODO THis isbaaaaaaaaaad
}

fn find_presents_per_house_to_target2(day: &mut Day20) -> usize {
    // The number of presents for a house is like
    // sum_of_factors * 10
    // So house 8 is (1+2+4+8)*10 = 150
    // sum_of_factors can also be done as prime factorization:
    // sum_of_factors = x^n ---> sum(x^0,x^1,..,x^n)
    // So for house 6, (1+2+3+6)*10 = 120
    // or 2^1*3^1 -> (1+2)*(1+3) = 12  then *10=120

    // but now, stop a base after 50 times

    day.presents_per_hosue.push(10); // Hack for house 1 as 1 is not prime

    for house in 2..day.target {
        let factors = day.primes.all_factors(house);
        let mut sum_factors = 0;
        for factor in factors.iter() {
            if house / factor > 50 {
                continue;
            }
            sum_factors += factor;
        }
        day.presents_per_hosue.push(sum_factors * 11);
        log::debug!("House {house} got {}", day.presents_per_hosue[house - 1]);
        if day.presents_per_hosue[house - 1] >= day.target {
            return house;
        }
    }

    0 // TODO THis isbaaaaaaaaaad
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
            presents_per_hosue: vec![],
        };

        for i in 2..100 {
            log::debug!(
                "{i} {:?} {:?}",
                day.primes.factorization(i),
                day.primes.all_factors(i)
            );
        }

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        let answer = find_presents_per_house_to_target(self);
        Ok(answer.to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(8.to_string()),
            false => Some(776160.to_string()),
        }
    }

    fn solve_part2(&mut self) -> Result<String> {
        let answer = find_presents_per_house_to_target2(self);
        Ok(answer.to_string())
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(6.to_string()),
            false => Some(786240.to_string()),
        }
    }
}
