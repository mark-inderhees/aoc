// 2016 Day 9
// https://adventofcode.com/2016/day/9

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
        let mut answer = 0;
        enum State {
            Normal,
            InMarker,
            Decompressing,
        }
        for compression in self.compressions.iter() {
            let mut decoded = "".to_string();
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
                            let re = Regex::new(r"(\d*)x(\d*)").unwrap();
                            let matches = re.captures(&marker).unwrap();
                            repeat_len = find_val(matches.get(1).unwrap().as_str());
                            repeat_count = find_val(matches.get(2).unwrap().as_str());
                            log::debug!("Found marker {}: {}x{}", marker, repeat_len, repeat_count);
                            state = State::Decompressing
                        }
                        _ => {
                            marker.push(character);
                        }
                    },
                    State::Normal => match character {
                        '(' => {
                            marker = "".to_string();
                            state = State::InMarker;
                        }
                        _ => decoded += &character.to_string(),
                    },
                    State::Decompressing => {
                        repeat_str.push(character);
                        repeat_index += 1;
                        if repeat_index == repeat_len {
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
        let mut answer = 0;
        // enum State {
        //     Normal,
        //     InMarker,
        //     JustFinishedMarker,
        // }
        // struct Repeat {
        //     length: usize,
        //     count: usize,
        // }

        // for compression in self.compressions.iter() {
        //     log::debug!("Decompress {}", compression);
        //     let mut length = 0;
        //     let mut state = State::Normal;
        //     let mut marker = "".to_string();
        //     let mut repeats: Vec<Repeat> = vec![];

        //     let mut i = 0;
        //     while i < compression.len() {
        //         let character = compression.chars().nth(i).unwrap();
        //         match character {
        //             '(' => {
        //                 marker = "".to_string();
        //                 state = State::InMarker;
        //             }
        //             ')' => {
        //                 let re = Regex::new(r"(\d*)x(\d*)").unwrap();
        //                 let matches = re.captures(&marker).unwrap();
        //                 let repeat = Repeat {
        //                     length: find_val(matches.get(1).unwrap().as_str()),
        //                     count: find_val(matches.get(2).unwrap().as_str()),
        //                 };
        //                 log::debug!(
        //                     "Found marker {}: {}x{}",
        //                     marker,
        //                     repeat.length,
        //                     repeat.count
        //                 );
        //                 repeats.push(repeat);
        //                 state = State::JustFinishedMarker
        //             }
        //             _ => match state {
        //                 State::InMarker => marker.push(character),
        //                 State::JustFinishedMarker => {
        //                     let first_length = repeats.last().unwrap().length;
        //                     let mut stack_length = first_length;
        //                     for repeat in repeats.iter() {
        //                         stack_length *= repeat.count;
        //                     }
        //                     i += first_length - 1;
        //                     length += stack_length;
        //                     log::debug!("Popped all markers, added length {}", stack_length);
        //                     state = State::Normal;
        //                 }
        //                 State::Normal => {
        //                     length += 1;
        //                 }
        //             },
        //         }
        //         i += 1;
        //     }
        //     log::debug!("Len {}", length);
        //     answer = length
        // }

        enum State {
            Normal,
            InMarker,
        }

        #[derive(Debug)]
        struct Repeat {
            length: usize,
            count: usize,
            index: usize,
            end_index: usize,
            characters: usize,
            consumed: bool,
        }

        for compression in self.compressions.iter() {
            answer = 0;
            log::debug!("Decompress {}", compression);
            let mut repeats: Vec<Repeat> = vec![];
            let mut state = State::Normal;
            let mut marker = "".to_string();
            let mut marker_index = 0;
            let mut max_end_index = 0;

            for (i, character) in compression.chars().enumerate() {
                match character {
                    '(' => {
                        marker = "".to_string();
                        marker_index = i;
                        state = State::InMarker;
                    }
                    ')' => {
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
                        State::InMarker => marker.push(character),
                        State::Normal => {
                            if i > max_end_index || max_end_index == 0 {
                                answer += 1;
                            }
                        }
                    },
                }
            }

            for i in (0..repeats.len()).rev() {
                log::debug!("{} {:?}", i, repeats[i]);
                let mut characters = 0;
                for j in (i + 1)..repeats.len() {
                    if (repeats[j].index <= repeats[i].end_index) && !repeats[j].consumed {
                        characters += repeats[j].characters;
                        repeats[j].consumed = true;
                        log::debug!("    {} {:?}", j, repeats[j]);
                    }
                }

                if characters == 0 {
                    characters = repeats[i].length
                }

                repeats[i].characters = characters * repeats[i].count;
                log::debug!("{} {:?}", i, repeats[i]);
            }

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
