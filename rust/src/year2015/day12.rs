// 2015 Day 12
// https://adventofcode.com/2015/day/12

use anyhow::Result;
use json::*;

use crate::puzzle::Puzzle;

#[allow(unused_imports)]
use crate::utils::utils::*;

#[allow(unused_imports)]
use std::collections::VecDeque;

pub struct Day12 {
    parsed: JsonValue,
}

fn walk_array(arr: &JsonValue, ignore_red: bool) -> f64 {
    let mut output: f64 = 0f64;
    for value in arr.members() {
        if value.is_array() {
            output += walk_array(value, ignore_red);
        } else if value.is_object() {
            output += walk_object(value, ignore_red);
        } else if value.is_number() {
            let number: f64 = value.as_number().unwrap().into();
            output += number;
        } else if value.is_string() {
            // TODO what to do with string
        } else {
            panic!("Unexpected value type in array");
        }
    }

    output
}

fn walk_object(obj: &JsonValue, ignore_red: bool) -> f64 {
    let mut output: f64 = 0f64;
    for (_key, value) in obj.entries() {
        if value.is_object() {
            output += walk_object(value, ignore_red);
        } else if value.is_array() {
            output += walk_array(value, ignore_red);
        } else if value.is_number() {
            let number: f64 = value.as_number().unwrap().into();
            output += number;
        } else if value.is_string() {
            let string = value.as_str().unwrap();
            if ignore_red && string == "red" {
                return 0f64;
            }
        } else {
            panic!("Unexpected value type in object");
        }
    }

    output
}

impl Puzzle for Day12 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut day = Day12 {
            parsed: JsonValue::new_array(),
        };

        day.parsed = json::parse(input).unwrap();

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        let answer = walk_object(&self.parsed, false);
        Ok(answer.to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(6.to_string()),
            false => Some(111754.to_string()),
        }
    }

    fn solve_part2(&mut self) -> Result<String> {
        let answer = walk_object(&self.parsed, true);
        Ok(answer.to_string())
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(4.to_string()),
            false => None,
        }
    }
}
