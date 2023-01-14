// 2015 Day 24
// https://adventofcode.com/2015/day/24
// --- Day 24: It Hangs in the Balance ---
// Balance presents in the sled

use anyhow::Result;
use itertools::Itertools;

use crate::puzzle::Puzzle;
use crate::utils::utils::*;

pub struct Day24 {
    presents: Vec<u64>,
}

/// Remove all items from the original list, returning a new smaller list.
fn remove_vec_items(original: &Vec<u64>, to_remove: &Vec<u64>) -> Vec<u64> {
    let mut output = original.clone();

    for item in to_remove.iter() {
        let i = output.iter().position(|&x| x == *item).unwrap();
        output.remove(i);
    }

    output
}

fn is_good_sum(group_weight: u64, presents: &Vec<u64>) -> bool {
    let sum: u64 = presents.iter().sum();
    if sum != group_weight {
        return false;
    }
    return true;
}

fn is_good_group(
    group_weight: u64,
    input_presents: &Vec<u64>,
    output_presents: &mut Vec<u64>,
    count: u64,
) -> bool {
    if count == 0 {
        return is_good_sum(group_weight, input_presents);
    }

    for len in 1..input_presents.len() - 1 {
        for group in input_presents.iter().combinations(len) {
            let group: Vec<u64> = group.iter().map(|&&x| x.clone()).collect();

            // Is this group the correct weight
            if !is_good_sum(group_weight, &group) {
                continue;
            }

            output_presents.extend(remove_vec_items(&input_presents, &group));
            let mut next_output_presents: Vec<u64> = vec![];
            if is_good_group(
                group_weight,
                output_presents,
                &mut next_output_presents,
                count - 1,
            ) {
                return true;
            }
        }
    }

    false
}

/// Balance the presents in the sled. Put the fewest possible presents in the
/// front seat. Find the smalles quantum of the possible front seat
/// configuration. The number of groups in the sled can be 3 or 4.
fn find_lowest_quantum_of_fewest_front_seat_balanced_presents(
    presents: &Vec<u64>,
    groups: u64,
) -> u64 {
    // Need to balance presents into 3 or 4 equal weight groups
    let total_weight: u64 = presents.iter().sum();
    assert_eq!(total_weight % groups, 0);
    let group_weight = total_weight / groups;
    log::debug!("Total weight {total_weight}, looking for group weight {group_weight}");

    // Front group must have fewest presents possible.
    // Return smallest guantum from that group of front presents options.
    let mut smallest_front_len = usize::MAX;
    let mut smallest_front_quantum = u64::MAX;

    for len1 in 1..presents.len() - 1 {
        for group1 in presents.iter().combinations(len1) {
            let group1: Vec<u64> = group1.iter().map(|&&x| x.clone()).collect();

            // Is the front group the correct weight?
            let sum1: u64 = group1.iter().sum();
            if sum1 != group_weight {
                continue;
            }

            // Check if smallest front group is already found
            if len1 > smallest_front_len {
                // There are no more small front groups, all done
                return smallest_front_quantum;
            }

            // Is this front group quantum too large?
            let quantum: u64 = group1.iter().product();
            if quantum > smallest_front_quantum {
                continue;
            }

            let back_presents = remove_vec_items(presents, &group1);
            let mut trunk_presents: Vec<u64> = vec![];
            if is_good_group(
                group_weight,
                &back_presents,
                &mut trunk_presents,
                groups - 2,
            ) {
                smallest_front_len = std::cmp::min(smallest_front_len, len1);
                smallest_front_quantum = std::cmp::min(smallest_front_quantum, quantum);
            }
        }
    }

    smallest_front_quantum
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
        // Find answer if 3 groups
        let answer = find_lowest_quantum_of_fewest_front_seat_balanced_presents(&self.presents, 3);
        Ok(answer.to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(99.to_string()),
            false => Some(11846773891u64.to_string()),
        }
    }

    fn solve_part2(&mut self) -> Result<String> {
        // Find answer if 4 groups
        let answer = find_lowest_quantum_of_fewest_front_seat_balanced_presents(&self.presents, 4);
        Ok(answer.to_string())
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(44.to_string()),
            false => Some(80393059.to_string()),
        }
    }
}
