// 2015 Day 7
// https://adventofcode.com/2015/day/7
// --- Day 7: Some Assembly Required ---
// For a list of chained bitwise operations, walk the chain and do the work

use anyhow::Result;

use crate::puzzle::Puzzle;

#[allow(unused_imports)]
use crate::utils::utils::*;

use std::collections::HashMap;
#[allow(unused_imports)]
use std::collections::VecDeque;

#[derive(Clone)]
pub struct Day07 {
    operations: HashMap<String, Operation>,
}

#[derive(Copy, Clone)]
enum Operators {
    SetValue(u16),
    SetResult,
    And,
    Or,
    Not,
    LShift(u16),
    RShift(u16),
}

#[derive(Clone)]
struct Operation {
    #[allow(dead_code)]
    name: String,
    result: Option<u16>,
    operator: Operators,
    lhs: String,
    rhs: String,
}

fn solve(day: &mut Day07) -> u16 {
    let mut jobs = vec!["a".to_string()];

    while jobs.len() > 0 {
        let job = jobs.pop().unwrap();
        let mut operation = day.operations[&job].clone();

        if let Some(_) = operation.result {
            // Do nothing
        } else {
            // Need to find the value
            let lhs_value = if operation.lhs == "" {
                None
            } else {
                day.operations[&operation.lhs].result
            };

            let rhs_value = if operation.rhs == "" {
                None
            } else {
                day.operations[&operation.rhs].result
            };

            match operation.operator {
                Operators::SetValue(x) => {
                    operation.result = Some(x);
                    log::debug!("Set {x} for {job}");
                }
                Operators::SetResult => {
                    if let Some(lhs) = lhs_value {
                        operation.result = Some(lhs);
                        log::debug!("Set {lhs} for {job}");
                    }
                }
                Operators::And => {
                    if let (Some(lhs), Some(rhs)) = (lhs_value, rhs_value) {
                        operation.result = Some(lhs & rhs);
                        log::debug!("Do {lhs} & {rhs} = {} for {job}", lhs & rhs);
                    }
                }
                Operators::Or => {
                    if let (Some(lhs), Some(rhs)) = (lhs_value, rhs_value) {
                        operation.result = Some(lhs | rhs);
                        log::debug!("Do {lhs} | {rhs} = {} for {job}", lhs | rhs);
                    }
                }
                Operators::Not => {
                    if let Some(lhs) = lhs_value {
                        operation.result = Some(!lhs);
                        log::debug!("Do !{lhs} = {} for {job}", !lhs);
                    }
                }
                Operators::LShift(x) => {
                    if let Some(lhs) = lhs_value {
                        operation.result = Some(lhs << x);
                        log::debug!("Do {lhs} << {x} = {} for {job}", lhs << x);
                    }
                }
                Operators::RShift(x) => {
                    if let Some(lhs) = lhs_value {
                        operation.result = Some(lhs >> x);
                        log::debug!("Do {lhs} >> {x} = {} for {job}", lhs >> x);
                    }
                }
            }

            if operation.result.is_none() {
                // Need to schedule more work
                jobs.push(job); // Reschedule this job so it runs after operands
                if operation.lhs != "" {
                    jobs.push(operation.lhs);
                }
                if operation.rhs != "" {
                    jobs.push(operation.rhs);
                }
            } else {
                // Update this entry in the map
                day.operations.insert(job, operation);
            }
        }
    }

    return day.operations["a"].result.unwrap();
}

impl Puzzle for Day07 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut day = Day07 {
            operations: HashMap::new(),
        };

        for line in input.lines() {
            let operator = if line.contains("AND") {
                Operators::And
            } else if line.contains("OR") {
                Operators::Or
            } else if line.contains("NOT") {
                Operators::Not
            } else if line.contains("LSHIFT") {
                Operators::LShift(find_val(line))
            } else if line.contains("RSHIFT") {
                Operators::RShift(find_val(line))
            } else {
                // Set a specific value, it could be absolute or relative
                let values = find_vals(line);
                if values.len() == 0 {
                    Operators::SetResult
                } else {
                    Operators::SetValue(values[0])
                }
            };

            let splits: Vec<&str> = line.split(" ").collect();
            let name = splits.last().unwrap().to_string();
            let mut lhs = "".to_string();
            let mut rhs = "".to_string();
            match operator {
                Operators::And => {
                    lhs = splits[0].to_string();
                    rhs = splits[2].to_string();
                }
                Operators::Or => {
                    lhs = splits[0].to_string();
                    rhs = splits[2].to_string();
                }
                Operators::Not => {
                    lhs = splits[1].to_string();
                }
                Operators::LShift(_) => {
                    lhs = splits[0].to_string();
                }
                Operators::RShift(_) => {
                    lhs = splits[0].to_string();
                }
                Operators::SetResult => {
                    lhs = splits[0].to_string();
                }
                Operators::SetValue(_) => {
                    // No operands
                }
            }

            day.operations.insert(
                name.clone(),
                Operation {
                    name,
                    result: None,
                    operator,
                    lhs: lhs.clone(),
                    rhs: rhs.clone(),
                },
            );

            // If lhs or rhs were absolute values, insert dummy entries with results
            let test_lhs: Vec<u16> = find_vals(&lhs);
            if test_lhs.len() > 0 {
                day.operations.insert(
                    lhs.clone(),
                    Operation {
                        name: lhs.clone(),
                        result: Some(test_lhs[0]),
                        operator,
                        lhs: "".to_string(),
                        rhs: "".to_string(),
                    },
                );
            }
        }

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        let answer = solve(self);
        Ok(answer.to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some((123 & 456).to_string()),
            false => Some(3176.to_string()),
        }
    }

    fn solve_part2(&mut self) -> Result<String> {
        let mut self_clone = self.clone();
        let b_value = solve(&mut self_clone);
        let mut b = self.operations["b"].clone();
        b.result = Some(b_value);
        self.operations.insert("b".to_string(), b);
        let answer = solve(self);

        Ok(answer.to_string())
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(72.to_string()), // This is not actually testing anything
            false => Some(14710.to_string()),
        }
    }
}
