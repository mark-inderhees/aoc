// 2015 Day 24
// https://adventofcode.com/2015/day/24
// --- Day 24: It Hangs in the Balance ---

use anyhow::Result;
use itertools::Itertools;

use crate::puzzle::Puzzle;

#[allow(unused_imports)]
use crate::utils::utils::*;

#[allow(unused_imports)]
use std::collections::VecDeque;

pub struct Day24 {
    presents: Vec<u32>,
}

/// Remove all items from the original list, returning a new smaller list.
fn remove_vec_items(original: &Vec<u32>, to_remove: &Vec<u32>) -> Vec<u32> {
    let mut output = original.clone();

    for item in to_remove.iter() {
        let i = output.iter().position(|&x| x == *item).unwrap();
        output.remove(i);
    }

    output
}

fn find_lowest_quantum_of_fewest_front_seat_balanced_presents(presents: &Vec<u32>) -> u32 {
    // Need to balance presents into 3 equal weight groups
    let total_weight: u32 = presents.iter().sum();
    assert_eq!(total_weight % 3, 0);
    let group_weight = total_weight / 3;
    log::debug!("Total weight {total_weight}, looking for group weight {group_weight}");

    let mut balanced_sets = vec![];

    let mut smallest_front_len = usize::MAX;
    let mut smallest_front_quantum = u32::MAX;

    'loop_len1: for len1 in 1..presents.len() - 2 {
        'loop_group1: for group1 in presents.iter().combinations(len1) {
            log::trace!("Group1 {group1:?}");
            let group1: Vec<u32> = group1.iter().map(|&&x| x.clone()).collect();
            let sum1: u32 = group1.iter().sum();
            if sum1 != group_weight {
                continue;
            }

            if len1 > smallest_front_len {
                // There are no more small front groups, all done
                break 'loop_len1;
            }
            let quantum: u32 = group1.iter().product();
            if quantum > smallest_front_quantum {
                continue;
            }

            let back_presents = remove_vec_items(presents, &group1);
            for len2 in 1..back_presents.len() - 1 {
                for group2 in back_presents.iter().combinations(len2) {
                    let group2: Vec<u32> = group2.iter().map(|&&x| x.clone()).collect();
                    let sum2: u32 = group2.iter().sum();
                    if sum2 != group_weight {
                        continue;
                    }
                    let group3 = remove_vec_items(&back_presents, &group2);
                    let sum3: u32 = group3.iter().sum();
                    if sum3 != group_weight {
                        continue;
                    }
                    balanced_sets.push((group1.clone(), group2, group3));
                    log::debug!(
                        "Found balanced set {:?}, QE {}",
                        balanced_sets.last().unwrap(),
                        quantum
                    );

                    smallest_front_len = std::cmp::min(smallest_front_len, len1);
                    smallest_front_quantum = std::cmp::min(smallest_front_quantum, quantum);

                    // Don't need any more group1s as exact make up of group2
                    // and group3 are not important.
                    continue 'loop_group1;
                }
            }
        }
    }

    // Sort by len of first group
    balanced_sets.sort_by(|a, b| a.0.len().cmp(&b.0.len()));

    log::debug!("Groups {balanced_sets:?}");

    let front_set_len = balanced_sets[0].0.len();
    let mut small_front_sets: Vec<&(Vec<u32>, Vec<u32>, Vec<u32>)> = balanced_sets
        .iter()
        .filter(|x| x.0.len() == front_set_len)
        .collect();

    log::debug!("Small front groups {small_front_sets:?}");

    // Sort by quantum of first group
    small_front_sets.sort_by(|a, b| a.0.iter().product::<u32>().cmp(&b.0.iter().product()));
    log::debug!("Small front groups quantum {small_front_sets:?}");

    return small_front_sets[0].0.iter().product();
}

impl Puzzle for Day24 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut day = Day24 { presents: vec![] };

        for line in input.lines() {
            day.presents.push(find_val(line));
        }

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        let answer = find_lowest_quantum_of_fewest_front_seat_balanced_presents(&self.presents);
        Ok(answer.to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(99.to_string()),
            false => None,
        }
    }

    fn solve_part2(&mut self) -> Result<String> {
        Ok("to do".to_string())
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => None,
            false => None,
        }
    }
}
