// 2015 Day 12
// https://adventofcode.com/2015/day/12
// --- Day 12: JSAbacusFramework.io ---
// Parse json!

use anyhow::Result;
use json::*; // This does all the magic!

use crate::puzzle::Puzzle;

pub struct Day12 {
    parsed: JsonValue,
}

/// Walk all of the array members within a json value. Sum up all numbers.
/// Optionally ignore any objects that contain a string with "red".
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
            // Do nothing with strings inside arrays
        } else {
            panic!("Unexpected value type in array");
        }
    }

    output
}

/// Walk all of the object entries within a json value. Sum up all numbers.
/// Optionally ignore any objects that contain a string with "red".
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
                // Ignore objects that contain a string with "red"
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

        // The input is pure json. Contains objects, arrays, numbers, and strings.
        // The root is always an object.
        day.parsed = json::parse(input).unwrap();

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        // Sum all numbers present
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
        // Sum all numbers present. Unless the object contains a string with "red".
        let answer = walk_object(&self.parsed, true);
        Ok(answer.to_string())
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(4.to_string()),
            false => Some(65402.to_string()),
        }
    }
}
