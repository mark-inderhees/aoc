// 2016 Day 9
// https://adventofcode.com/2016/day/9
// --- Day 9: Explosives in Cyberspace ---
// Decompress data using a silly format

use anyhow::Result;
use regex::Regex;

use crate::puzzle::Puzzle;

#[allow(unused_imports)]
use crate::utils::utils::*;

#[allow(unused_imports)]
use std::collections::VecDeque;

pub struct Day09 {
    compressions: Vec<String>,
}

impl Puzzle for Day09 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut day = Day09 {
            compressions: vec![],
        };

        for line in input.lines() {
            day.compressions.push(line.to_string());
        }

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        // Find length of decompressed string, compression markers look like
        // (LxC) for length of chars to read after marker and count for number
        // of times to repeat that pattern.
        // If a marker is found inside a marker, then ignore it.

        let mut answer = 0;
        enum State {
            Normal,        // Just copy to decompressed
            InMarker,      // This is marker
            Decompressing, // Building a string to repeat
        }
        for compression in self.compressions.iter() {
            let mut decoded = "".to_string(); // Final output string
            let mut state = State::Normal;
            let mut marker = "".to_string();
            let mut repeat_str = "".to_string();
            let mut repeat_len = 0;
            let mut repeat_count = 0;
            let mut repeat_index = 0;
            for character in compression.chars() {
                match state {
                    State::InMarker => match character {
                        ')' => {
                            // Done reading in a marker, grep out the data and move to decompressing
                            let re = Regex::new(r"(\d*)x(\d*)").unwrap();
                            let matches = re.captures(&marker).unwrap();
                            repeat_len = find_val(matches.get(1).unwrap().as_str());
                            repeat_count = find_val(matches.get(2).unwrap().as_str());
                            log::debug!("Found marker {}: {}x{}", marker, repeat_len, repeat_count);
                            state = State::Decompressing
                        }
                        _ => {
                            // Add character to marker string
                            marker.push(character);
                        }
                    },
                    State::Normal => match character {
                        '(' => {
                            // Just found a marker, start new string for it
                            marker = "".to_string();
                            state = State::InMarker;
                        }
                        _ => {
                            // Copy directly to output
                            decoded += &character.to_string()
                        }
                    },
                    State::Decompressing => {
                        // Add to string to be repeated
                        repeat_str.push(character);
                        repeat_index += 1;
                        if repeat_index == repeat_len {
                            // Done building the repeat string, now add it to
                            // output and move back to normal state.
                            log::debug!("Found repeat {}", repeat_str);
                            for _ in 0..repeat_count {
                                decoded += &repeat_str;
                            }
                            state = State::Normal;
                            repeat_index = 0;
                            repeat_str = "".to_string()
                        }
                    }
                }
            }
            log::debug!("Decompressed {}", decoded);
            log::debug!("Len {}", decoded.len());
            answer = decoded.len();
        }
        Ok(answer.to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(238.to_string()),
            false => Some(112830.to_string()),
        }
    }

    fn solve_part2(&mut self) -> Result<String> {
        // Like part1, but now if a marker is found inside a marker, it applies
        // at scale. Find the length of the decompressed string.

        let mut answer = 0;

        enum State {
            Normal,   // Not inside a marker
            InMarker, // Inside a marker
        }

        // Repeat struct stores info on the marker
        #[derive(Debug)]
        struct Repeat {
            length: usize,     // Number of chars after the marker in original string
            count: usize,      // Number of times to repeat
            index: usize,      // Index of marker in original string
            end_index: usize,  // Range this marker applies to in original string
            characters: usize, // Fully decompressed size this marker represents
            consumed: bool,    // If this marker as be consumed by another marker
        }

        for compression in self.compressions.iter() {
            answer = 0;
            log::debug!("Decompress {}", compression);
            let mut repeats: Vec<Repeat> = vec![];
            let mut state = State::Normal;
            let mut marker = "".to_string();
            let mut marker_index = 0; // Start index of the current marker
            let mut max_end_index = 0; // Max end index of any currently known markers

            for (i, character) in compression.chars().enumerate() {
                match character {
                    '(' => {
                        // Found a marker, start building its string
                        marker = "".to_string();
                        marker_index = i;
                        state = State::InMarker;
                    }
                    ')' => {
                        // Done with the marker, parse out info and add it to list of markers
                        let re = Regex::new(r"(\d*)x(\d*)").unwrap();
                        let matches = re.captures(&marker).unwrap();
                        let length = find_val(matches.get(1).unwrap().as_str());
                        let count = find_val(matches.get(2).unwrap().as_str());
                        let repeat = Repeat {
                            length,
                            count,
                            index: marker_index,
                            end_index: i + length,
                            characters: 0,
                            consumed: false,
                        };
                        log::debug!(
                            "Found marker {}: {}x{}, {:?}",
                            marker,
                            repeat.length,
                            repeat.count,
                            repeat
                        );
                        max_end_index = max_end_index.max(repeat.end_index);
                        repeats.push(repeat);
                        state = State::Normal;
                    }
                    _ => match state {
                        State::InMarker => {
                            // Add char to marker string
                            marker.push(character)
                        }
                        State::Normal => {
                            if i > max_end_index || max_end_index == 0 {
                                // This char is not covered by a marker, so add it to output directly
                                answer += 1;
                            }
                        }
                    },
                }
            }

            // Now loop through all markers in reverse direction and find overlaps.
            // If a marker overlaps, then consume the child and add it into the parent.
            for i in (0..repeats.len()).rev() {
                log::debug!("{} {:?}", i, repeats[i]);
                let mut characters = 0;
                for j in (i + 1)..repeats.len() {
                    if (repeats[j].index <= repeats[i].end_index) && !repeats[j].consumed {
                        // Found an unconsumed child, add it in
                        characters += repeats[j].characters;
                        repeats[j].consumed = true;
                        log::debug!("    {} {:?}", j, repeats[j]);
                    }
                }

                if characters == 0 {
                    // This marker has no child markers, so just use it on its own
                    characters = repeats[i].length
                }

                repeats[i].characters = characters * repeats[i].count;
                log::debug!("{} {:?}", i, repeats[i]);
            }

            // Now add in the size of parent markers to the output
            for repeat in repeats.iter() {
                if !repeat.consumed {
                    answer += repeat.characters;
                }
            }
            log::debug!("Length {}", answer);
        }

        Ok(answer.to_string())
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(445.to_string()),
            false => Some(10931789799i64.to_string()),
        }
    }
}
