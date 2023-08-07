// 2016 Day 7
// https://adventofcode.com/2016/day/7

use anyhow::Result;

use crate::puzzle::Puzzle;

#[allow(unused_imports)]
use crate::utils::utils::*;

#[allow(unused_imports)]
use std::collections::VecDeque;

pub struct Day07 {
    ips: Vec<String>,
}

impl Puzzle for Day07 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut day = Day07 { ips: vec![] };

        for line in input.trim().split("\n") {
            day.ips.push(line.to_string());
        }

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        let mut valid_count = 0;
        for ip in &self.ips {
            let mut valid = false;
            let mut in_brackets = false;
            let chars: Vec<char> = ip.chars().collect();
            log::debug!("IP {}", ip);
            for window in chars.windows(4) {
                log::debug!("Window {:?}", window);
                if window.contains(&'[') {
                    log::debug!("Start brackets");
                    in_brackets = true;
                } else if window.contains(&']') {
                    log::debug!("End brackets");
                    in_brackets = false;
                } else {
                    log::debug!(
                        "Testing chars {} {} {} {}",
                        window[0],
                        window[1],
                        window[2],
                        window[3]
                    );
                    if (window[0] == window[3])
                        && (window[1] == window[2])
                        && (window[0] != window[1])
                    {
                        if in_brackets {
                            log::debug!("Inside brackets, not valid IP");
                            valid = false;
                            break;
                        } else {
                            log::debug!("Valid IP!!!");
                            valid = true;
                        }
                    }
                }
            }
            if valid {
                valid_count += 1;
            }
        }

        Ok(valid_count.to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(2.to_string()),
            false => Some(110.to_string()),
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
