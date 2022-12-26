// 2022 Day 20
// https://adventofcode.com/2022/day/20
// --- Day 20: Grove Positioning System ---
// Walk a list and do operations to encode/decode data

use anyhow::Result;
use std::rc::Rc;

use crate::puzzle::Puzzle;
use crate::utils::linked_list::*;
use crate::utils::utils::*;

#[derive(Default)]
pub struct Day20 {
    values: Vec<i64>,
    linked_list: LinkedList<i64>,
}

fn scramble(day: &mut Day20, iterations: usize, decryption_key: i64) {
    for i in 0..day.linked_list.values.len() {
        let current = day.linked_list.values[i].clone();
        let value = current.borrow().value;
        current.borrow_mut().value = value * decryption_key;
    }

    for _ in 0..iterations {
        for i in 0..day.linked_list.values.len() {
            let current = day.linked_list.values[i].clone();
            let current_weak = Rc::downgrade(&current);

            let value = current.borrow().value;
            let len = day.linked_list.len() as i64 - 1; // -1 as pop makes list 1 smaller
            let mut shift = value % len;
            if shift == 0 {
                continue;
            }
            shift -= 1;

            log::debug!("Moving {value}");

            day.linked_list.set_current(&current_weak);
            day.linked_list.pop();
            for _ in 0..shift.abs() {
                if shift > 0 {
                    day.linked_list.move_next();
                } else {
                    day.linked_list.move_prev();
                }
            }

            day.linked_list.insert(&current_weak);
            day.linked_list.print();
        }
    }
}

fn get_sum(day: &mut Day20) -> i64 {
    loop {
        if day.linked_list.get_current_value() == 0 {
            break;
        }
        day.linked_list.move_next();
    }

    let mut answers = vec![];
    for _ in 0..3 {
        for _ in 0..1000 % day.linked_list.len() {
            day.linked_list.move_next();
        }
        answers.push(day.linked_list.get_current_value());
    }
    log::debug!("{:?}", answers);
    let answer: i64 = answers.iter().sum();

    answer
}

impl Puzzle for Day20 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut day = Day20 {
            values: vec![],
            ..Default::default()
        };

        for (i, line) in input.lines().enumerate() {
            day.values.push(get_val(line));
        }

        day.linked_list = LinkedList::new(&day.values);

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
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
