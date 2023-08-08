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
            let mut repeate_len = 0;
            let mut repeate_count = 0;
            let mut repeate_index = 0;
            for char in compression.chars() {
                match state {
                    State::InMarker => match char {
                        ')' => {
                            let re = Regex::new(r"(\d*)x(\d*)").unwrap();
                            let matches = re.captures(&marker).unwrap();
                            repeate_len = find_val(matches.get(1).unwrap().as_str());
                            repeate_count = find_val(matches.get(2).unwrap().as_str());
                            log::debug!(
                                "Found marker {}: {}x{}",
                                marker,
                                repeate_len,
                                repeate_count
                            );
                            state = State::Decompressing
                        }
                        _ => {
                            marker.push(char);
                        }
                    },
                    State::Normal => match char {
                        '(' => {
                            marker = "".to_string();
                            state = State::InMarker;
                        }
                        _ => decoded += &char.to_string(),
                    },
                    State::Decompressing => {
                        repeat_str.push(char);
                        repeate_index += 1;
                        if repeate_index == repeate_len {
                            log::debug!("Found repeate {}", repeat_str);
                            for _ in 0..repeate_count {
                                decoded += &repeat_str;
                            }
                            state = State::Normal;
                            repeate_index = 0;
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
            true => Some(18.to_string()),
            false => Some(112830.to_string()),
        }
    }

    fn solve_part2(&mut self) -> Result<String> {
        Ok("to do".to_string())
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => None,
            false => None,
        }
    }
}
