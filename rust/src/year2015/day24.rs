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

fn find_lowest_quantum_of_fewest_front_seat_balanced_presents(
    presents: &Vec<u64>,
    groups: u64,
) -> u64 {
    // Need to balance presents into 3 equal weight groups
    let total_weight: u64 = presents.iter().sum();
    assert_eq!(total_weight % groups, 0);
    let group_weight = total_weight / groups;
    log::debug!("Total weight {total_weight}, looking for group weight {group_weight}");

    let mut smallest_front_len = usize::MAX;
    let mut smallest_front_quantum = u64::MAX;

    'loop_len1: for len1 in 1..presents.len() - 2 {
        'loop_group1: for group1 in presents.iter().combinations(len1) {
            log::trace!("Group1 {group1:?}");
            let group1: Vec<u64> = group1.iter().map(|&&x| x.clone()).collect();
            let sum1: u64 = group1.iter().sum();
            if sum1 != group_weight {
                continue;
            }

            if len1 > smallest_front_len {
                // There are no more small front groups, all done
                break 'loop_len1;
            }
            let quantum: u64 = group1.iter().product();
            if quantum > smallest_front_quantum {
                continue;
            }

            let back_presents = remove_vec_items(presents, &group1);
            for len2 in 1..back_presents.len() - 1 {
                for group2 in back_presents.iter().combinations(len2) {
                    let group2: Vec<u64> = group2.iter().map(|&&x| x.clone()).collect();
                    let sum2: u64 = group2.iter().sum();
                    if sum2 != group_weight {
                        continue;
                    }
                    let mut good = false;
                    if groups == 3 {
                        let group3 = remove_vec_items(&back_presents, &group2);
                        let sum3: u64 = group3.iter().sum();
                        if sum3 != group_weight {
                            continue;
                        }
                        good = true;
                    } else {
                        let trunk = remove_vec_items(&back_presents, &group2);
                        'loop_len3: for len3 in 1..trunk.len() - 1 {
                            for group3 in trunk.iter().combinations(len3) {
                                let group3: Vec<u64> = group3.iter().map(|&&x| x.clone()).collect();
                                let sum3: u64 = group3.iter().sum();
                                if sum3 != group_weight {
                                    log::trace!("Sum3 no good {} {:?}", sum3, group3);
                                    continue;
                                }
                                let group4 = remove_vec_items(&trunk, &group3);
                                let sum4: u64 = group4.iter().sum();
                                if sum4 != group_weight {
                                    log::trace!("Sum4 no good {} {:?}", sum3, group4);
                                    continue;
                                }
                                good = true;
                                break 'loop_len3;
                            }
                        }
                    }

                    if good {
                        smallest_front_len = std::cmp::min(smallest_front_len, len1);
                        smallest_front_quantum = std::cmp::min(smallest_front_quantum, quantum);

                        // Don't need any more group1s as exact make up of group2
                        // and group3 are not important.
                        continue 'loop_group1;
                    }
                }
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
