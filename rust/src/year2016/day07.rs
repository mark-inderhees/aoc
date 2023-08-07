// 2016 Day 7
// https://adventofcode.com/2016/day/7
// --- Day 7: Internet Protocol Version 7 ---
// Parse strings for repeating patterns

use anyhow::Result;

use crate::puzzle::Puzzle;

#[allow(unused_imports)]
use crate::utils::utils::*;

#[allow(unused_imports)]
use std::collections::VecDeque;

pub struct Day07 {
    ips: Vec<String>, // Input is a list of IPv7s
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
        // Find IPs that have abba type of pattern outside of []
        let mut valid_count = 0;
        for ip in &self.ips {
            let mut valid = false;
            let mut in_brackets = false;
            let chars: Vec<char> = ip.chars().collect();
            log::debug!("IP {}", ip);
            for window in chars.windows(4) {
                log::debug!("Window {:?}", window);
                // First check if in or out of brackets
                if window.contains(&'[') {
                    log::debug!("Start brackets");
                    in_brackets = true;
                } else if window.contains(&']') {
                    log::debug!("End brackets");
                    in_brackets = false;
                } else {
                    // Now test for abba pattern
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
                        // Found a match, if in brackets then we known we are
                        // done and not valid. If out of brackets, this could
                        // be valid but need to test full IP.
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
        // Test IP for repeating pattern like that has aba outside of brackets
        // and bab inside of brackets.
        let mut valid_count = 0;
        for ip in &self.ips {
            let mut valid = false;
            let mut in_brackets = false;

            // Need to keep list of found aba type of patterns both inside and
            // outside of brackets so only a single pass of IP is needed
            let mut matches_inside = vec![];
            let mut matches_outside = vec![];

            let chars: Vec<char> = ip.chars().collect();
            log::debug!("IP {}", ip);
            for window in chars.windows(3) {
                log::debug!("Window {:?}", window);
                // First check if inside or out of brackets
                if window.contains(&'[') {
                    log::debug!("Start brackets");
                    in_brackets = true;
                } else if window.contains(&']') {
                    log::debug!("End brackets");
                    in_brackets = false;
                } else {
                    // Now test for aba type of pattern
                    log::debug!("Testing chars {} {} {}", window[0], window[1], window[2]);
                    if (window[0] == window[2]) && (window[0] != window[1]) {
                        // Found pattern, build opposite that needs match, then
                        // look to see if we've found the opposite yet.
                        let opposite: &[char] = &[window[1], window[0], window[1]];
                        if in_brackets {
                            matches_inside.push(window);
                            if matches_outside.contains(&opposite) {
                                log::debug!("Found both inside and out, done!");
                                valid = true;
                                break;
                            }
                        } else {
                            matches_outside.push(window);
                            if matches_inside.contains(&opposite) {
                                log::debug!("Found both outside and in, done!");
                                valid = true;
                                break;
                            }
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

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(3.to_string()),
            false => Some(242.to_string()),
        }
    }
}
