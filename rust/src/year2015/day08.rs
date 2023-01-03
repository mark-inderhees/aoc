// 2015 Day 8
// https://adventofcode.com/2015/day/8
// Count characters in strings with escape sequences

use anyhow::Result;

use crate::puzzle::Puzzle;

pub struct Day08 {
    strings: Vec<String>,
}

/// Returns number of chars in string and number of bytes it would be on disk
/// once escapes are processed.
fn string_counts(string: &str) -> (u32, u32) {
    let string_len = string.chars().count();
    let mut disk_len = 0;
    let mut i = 0;
    while i < string_len {
        // Count size on disk
        // Convert \\ => 1 char
        // Convert \" => 1 char
        // Convert \xXX => 1 char
        // Ignore start and end quotes ""
        let char = string.chars().nth(i).unwrap();
        if char == '\\' {
            // This is an escape sequence
            disk_len += 1;
            i += 1;
            let next_char = string.chars().nth(i).unwrap();
            if next_char == 'x' {
                // Also skip the two hexidecimal chars
                i += 2;
            }
        } else if char == '"' {
            // Ignore quotes
        } else {
            disk_len += 1;
        }

        i += 1;
    }

    log::debug!("For {string} found {string_len} and {disk_len}");

    (string_len as u32, disk_len)
}

/// Returns number of chars in string and number of chars in escaped version of string
fn escape_counts(string: &str) -> (u32, u32) {
    let string_len = string.chars().count();
    let mut escape_len = 0;
    let mut i = 0;

    while i < string_len {
        // Always count this char
        escape_len += 1;

        // Add extra escape chars where needed
        let char = string.chars().nth(i).unwrap();
        if char == '\\' {
            // This is an escape sequence
            let next_char = string.chars().nth(i + 1).unwrap();
            if next_char == 'x' {
                // Add one extra for hexidecimal escape
                escape_len += 1;
            } else if next_char == '"' {
                // Add two extra for escaped quotes
                escape_len += 2;
            } else if next_char == '\\' {
                // Add two extra for double quotes and then skip to next char
                // so the escape is not reprocessed. So add three in total.
                escape_len += 3;
                i += 1;
            }
        }

        i += 1;
    }

    // Add extra for leading and trailing quotes
    escape_len += 4;

    log::debug!("For {string} found {string_len} and {escape_len}");

    (string_len as u32, escape_len)
}

impl Puzzle for Day08 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut day = Day08 { strings: vec![] };

        for line in input.lines() {
            day.strings.push(line.to_string());
        }

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        let mut answer = 0;
        for string in self.strings.iter() {
            let counts = string_counts(string);
            answer = answer + counts.0 - counts.1;
        }
        Ok(answer.to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(12.to_string()),
            false => Some(1350.to_string()),
        }
    }

    fn solve_part2(&mut self) -> Result<String> {
        let mut answer = 0;
        for string in self.strings.iter() {
            let counts = escape_counts(string);
            answer = answer + counts.1 - counts.0;
        }
        Ok(answer.to_string())
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(19.to_string()),
            false => Some(2085.to_string()),
        }
    }
}
