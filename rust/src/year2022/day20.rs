// 2022 Day 20
// https://adventofcode.com/2022/day/20
// --- Day 20: Grove Positioning System ---
// Walk a linked list and do operations to encode/decode data

use anyhow::Result;
use std::rc::Rc;

use crate::puzzle::Puzzle;
use crate::utils::linked_list::*;
use crate::utils::utils::*;

#[derive(Default)]
pub struct Day20 {
    linked_list: LinkedList<i64>,
}

/// Walk a link list in the original order. For each node, take the value of
/// the node and move the node that many places. Run this `iterations` times.
/// Before running logic, scale by `decription_key`.
fn scramble(day: &mut Day20, iterations: usize, decryption_key: i64) {
    // Scale each node by the decription key
    day.linked_list.move_to_head();
    for _ in 0..day.linked_list.len() {
        let value = day.linked_list.get_current_value() * decryption_key;
        day.linked_list.set_current_value(&value);
        day.linked_list.move_next();
    }

    // Run the requested iterations
    for _ in 0..iterations {
        // Modify each node in the list in the original order
        for i in 0..day.linked_list.len() {
            // Get node from original order
            let node = day.linked_list.values[i].clone();
            let node_weak = Rc::downgrade(&node);

            // Use value of node to determine the shift amount.
            // Do not do more than one full loop.
            let value = node.borrow().value;
            let len = day.linked_list.len() as i64 - 1; // -1 as pop makes list 1 smaller
            let mut shift = value % len;
            if shift == 0 {
                // No work when shift is zero
                continue;
            }

            // Pop will move next one, so reduce shift count for positive shift.
            // For negative shift, increase shift count by one as pop moved to next.
            shift -= 1;

            log::debug!("Moving {value}");

            // Pop this node
            day.linked_list.set_current(&node_weak);
            day.linked_list.pop();

            // Now walk the list the requested times
            for _ in 0..shift.abs() {
                if shift > 0 {
                    day.linked_list.move_next();
                } else {
                    day.linked_list.move_prev();
                }
            }

            // Finally, insert the node at this new locaiton
            day.linked_list.insert(&node_weak);
            day.linked_list.print();
        }
    }
}

/// Get the sum of the values 1000, 2000, and 3000 nodes beyond the location of
/// the node with value zero.
fn get_sum(day: &mut Day20) -> i64 {
    // Find the node with value zero
    loop {
        if day.linked_list.get_current_value() == 0 {
            break;
        }
        day.linked_list.move_next();
    }

    // Add values at index 1000, 2000, and 3000 to a vector
    let mut answers = vec![];
    for _ in 0..3 {
        for _ in 0..1000 % day.linked_list.len() {
            day.linked_list.move_next();
        }
        answers.push(day.linked_list.get_current_value());
    }
    log::debug!("{:?}", answers);

    // Sum the vector and return the answer
    let answer: i64 = answers.iter().sum();

    answer
}

impl Puzzle for Day20 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut day = Day20 {
            ..Default::default()
        };

        let mut values = vec![];
        for (i, line) in input.lines().enumerate() {
            values.push(get_val(line));
        }

        day.linked_list = LinkedList::new(&values);

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        // Run one scramble with scaler of 1
        scramble(self, 1, 1);
        let answer = get_sum(self);

        Ok(answer.to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(3.to_string()),
            false => Some(6640.to_string()),
        }
    }

    fn solve_part2(&mut self) -> Result<String> {
        // Run 10 scrambles with a big scaler
        scramble(self, 10, 811589153);
        let answer = get_sum(self);

        Ok(answer.to_string())
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(1623178306.to_string()),
            false => Some(11893839037215u64.to_string()),
        }
    }
}
