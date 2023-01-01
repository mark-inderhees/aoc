// 2015 Day 5
// https://adventofcode.com/2015/day/5
// --- Day 5: Doesn't He Have Intern-Elves For This? ---
// Parse strings for funny requirements like double letters

use anyhow::Result;

use crate::puzzle::Puzzle;

#[allow(unused_imports)]
use crate::utils::utils::*;

#[allow(unused_imports)]
use std::collections::VecDeque;

pub struct Day05 {
    strings: Vec<String>,
}

/// Is a string naughty or nice for part 1
fn is_string_nice(string: &str) -> bool {
    // Needs at least 3 vowels (aeiou)
    let mut vowel_count = 0;
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];
    for char in string.chars() {
        if vowels.contains(&char) {
            vowel_count += 1;
        }
    }
    if vowel_count < 3 {
        log::debug!("{string} does not have at least 3 vowels");
        return false;
    }

    // Needs a double letter (like xx)
    let mut has_double_letter = false;
    for (i, char) in string.chars().enumerate() {
        if i < string.chars().count() - 1 {
            if char == string.chars().nth(i + 1).unwrap() {
                has_double_letter = true;
                break;
            }
        }
    }
    if !has_double_letter {
        log::debug!("{string} does not have a double letter");
        return false;
    }

    // Cannot contain ab, cd, pq, or xy
    let forbidden = vec!["ab", "cd", "pq", "xy"];
    for forbid in forbidden {
        if string.contains(forbid) {
            log::debug!("{string} contains {forbid}");
            return false;
        }
    }

    log::debug!("{string} is nice");
    true
}

/// Is a string naughty or nice for part 2
fn is_string_nice_part2(string: &str) -> bool {
    // Contains double pairs, like xy in xyxy, but no overlap like aaa
    let mut has_double_pair = false;
    for (i, char) in string.chars().enumerate() {
        if i < string.chars().count() - 2 {
            // Build double pair
            let char2 = string.chars().nth(i + 1).unwrap();
            let pair = format!("{char}{char2}");

            // Search rest of string
            let substring = string.split_at(i + 2).1;
            if substring.contains(&pair) {
                log::info!("{string} has double pair {pair}, second half {substring}");
                has_double_pair = true;
                break;
            }
        }
    }
    if !has_double_pair {
        log::debug!("{string} does not have a double pair");
        return false;
    }

    // Hash repeat with one letter between, like efe in abcdefeghi
    let mut has_repeat = false;
    for (i, char) in string.chars().enumerate() {
        if i < string.chars().count() - 2 {
            // Skip one letter, compare if same
            if char == string.chars().nth(i + 2).unwrap() {
                log::info!("{string} has repeat {char}");
                has_repeat = true;
                break;
            }
        }
    }
    if !has_repeat {
        log::debug!("{string} does not have a repeat letter");
        return false;
    }

    log::info!("{string} is nice");
    true
}

impl Puzzle for Day05 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut day = Day05 { strings: vec![] };

        for line in input.lines() {
            day.strings.push(line.to_string().to_lowercase());
        }

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        let mut nice_count = 0;
        for string in &self.strings {
            if is_string_nice(string) {
                nice_count += 1;
            }
        }
        Ok(nice_count.to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(2.to_string()),
            false => Some(255.to_string()),
        }
    }

    fn solve_part2(&mut self) -> Result<String> {
        let mut nice_count = 0;
        for string in &self.strings {
            if is_string_nice_part2(string) {
                nice_count += 1;
            }
        }
        Ok(nice_count.to_string())
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(2.to_string()),
            false => Some(55.to_string()),
        }
    }
}
