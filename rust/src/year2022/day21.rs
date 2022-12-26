// 2022 Day 21
// https://adventofcode.com/2022/day/21
// --- Day 21: Monkey Math ---
// Walk a whole bunch of math equations
// In part 2, do algebra to solve math equations

use anyhow::Result;
use std::collections::HashMap;

use crate::puzzle::Puzzle;
use crate::utils::utils::*;

pub struct Day21 {
    monkeys: HashMap<String, Monkey>,
}

#[derive(Default, Debug, Clone)]
struct Monkey {
    value: Option<i128>,
    id: String,
    operator: Operator,
    left: String,
    right: String,
    try_count: u32,
}

// Walk a tree of math equations, starting at the root. If an equation cannot
// be solved yet, then move to the child nodes to try and solve that equation.
fn populate(day: &mut Day21, start_id: &str) {
    #[derive(Clone)]
    struct Work {
        id: String,
    }
    let job = Work {
        id: start_id.to_string(),
    };
    let mut jobs = vec![job];

    // Utilize try count to opportunistically fail if a branch is currently unsolvable
    for monkey in day.monkeys.values_mut() {
        monkey.try_count = 0;
    }

    while jobs.len() > 0 {
        // Get the job to do
        let job = jobs.pop().unwrap();
        let mut monkey = day.monkeys[&job.id].clone();
        monkey.try_count += 1;
        day.monkeys.insert(job.id.clone(), monkey.clone());

        // Look at the job's value
        if let Some(_value) = monkey.value {
            panic!("Not expected populated value");
        } else {
            // We have work to do, check if left and right values are available, else schedule work
            let mut new_jobs = vec![job.clone()]; // Reschedule this job last in the stack

            // Ignore unsolved leaf nodes
            if monkey.left == "" || monkey.right == "" {
                continue;
            }

            // Only try a node twice, else it is currently unsolveable
            if monkey.try_count > 2 {
                continue;
            }

            // Get the values to operate on
            let monkey_left = &day.monkeys[&monkey.left];
            let monkey_right = &day.monkeys[&monkey.right];

            if let (Some(value_left), Some(value_right)) = (monkey_left.value, monkey_right.value) {
                // We can comput now, so do it!
                monkey.value = Some(do_math(monkey.operator, value_left, value_right));
                log::debug!(
                    "Doing math {} = {} {:?} {}, {} {:?} {} = {}",
                    monkey.id,
                    monkey_left.id,
                    monkey.operator,
                    monkey_right.id,
                    value_left,
                    monkey.operator,
                    value_right,
                    monkey.value.unwrap()
                );
                day.monkeys.insert(job.id.clone(), monkey);
                continue;
            }

            // Cannot do math yet, schedule work for unsolved nodes
            if monkey_left.value.is_none() {
                new_jobs.push(Work {
                    id: monkey_left.id.clone(),
                });
            }
            if monkey_right.value.is_none() {
                new_jobs.push(Work {
                    id: monkey_right.id.clone(),
                });
            }

            // Push the new jobs onto the stack
            jobs.extend(new_jobs);
        }
    }
}

// Search the hash map, find the monkey that depends on me
// The solve_for_left means that my_id is on the left side
// Returns tuple of (key_id, value_id, operator, solve_for_left)
fn find_monkeys(day: &Day21, my_id: &str) -> (String, String, Operator, bool) {
    for (key, monkey) in day.monkeys.iter() {
        if monkey.left == my_id {
            // my_id is on the left side
            return (
                key.to_string(),
                monkey.right.to_string(),
                monkey.operator,
                true,
            );
        } else if monkey.right == my_id {
            // my_id is on the right side
            return (
                key.to_string(),
                monkey.left.to_string(),
                monkey.operator,
                false,
            );
        }
    }
    panic!("Could not find your monkey");
}

// Walk a tree of math equations, starting at a target leaf node. The math
// equations need to be solved using simple algebra. If a value is not known
// yet, then find parents and solve the equation for the parent node.
fn populate_backwards(day: &mut Day21, start_id: &str) {
    #[derive(Clone, Debug)]
    struct Work {
        id: String,
    }
    let job = Work {
        id: start_id.to_string(),
    };
    let mut jobs = vec![job];

    while jobs.len() > 0 {
        // Get the job
        let job = jobs.pop().unwrap();
        let mut monkey = day.monkeys[&job.id].clone();

        // Check the value on the job
        if let Some(_value) = monkey.value {
            panic!("Not expected populated value");
        } else {
            // Try to compute the value of the job. Or if it cannot be compute
            // yet, schedule jobs for the parents.
            let mut new_jobs = vec![job.clone()]; // Reschedule this job last in the stack

            // Find the parent nodes by searchin th hash map
            let monkey_ids = find_monkeys(day, &job.id);
            let monkey_id_key = monkey_ids.0;
            let monkey_id_value = monkey_ids.1;
            let operator = monkey_ids.2;
            let solve_for_left = monkey_ids.3; // Bool for if this node's value is on the left side of the operation
            let monkey_key = &day.monkeys[&monkey_id_key];
            let monkey_value = &day.monkeys[&monkey_id_value];

            if let (Some(value_key), Some(value_value)) = (monkey_key.value, monkey_value.value) {
                // We can comput now, so do it!
                monkey.value = Some(solve_math(operator, value_key, value_value, solve_for_left));
                log::debug!(
                    "Doing math {} = {} {:?} {}, {} {:?} {} = {}",
                    monkey.id,
                    monkey_key.id,
                    operator,
                    monkey_value.id,
                    value_key,
                    operator,
                    value_value,
                    monkey.value.unwrap()
                );
                day.monkeys.insert(job.id.clone(), monkey);
                continue;
            }

            // We cannot do math yet, find the needed values
            if monkey_key.value.is_none() {
                new_jobs.push(Work {
                    id: monkey_key.id.clone(),
                });
            } else {
                panic!("If we cannot do math, it must be because they key is not yet known");
            }

            if monkey_value.value.is_none() {
                panic!("To solve equations, the other value must always be already known");
            }

            log::debug!("Schedule new jobs for {new_jobs:?}");
            jobs.extend(new_jobs);
        }
    }
}

impl Puzzle for Day21 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut day = Day21 {
            monkeys: HashMap::new(),
        };

        // Parse input like
        // root: pppw + sjmn
        // dbpl: 5
        for line in input.lines() {
            let mut monkey = Monkey {
                ..Default::default()
            };
            let mut data: Vec<&str> = line.split(":").collect();
            monkey.id = data[0].to_string();
            let str = data[1].to_string();
            if str.contains("+") {
                monkey.operator = Operator::Add;
            } else if str.contains("-") {
                monkey.operator = Operator::Subtract;
            } else if str.contains("*") {
                monkey.operator = Operator::Multiply;
            } else if str.contains("/") {
                monkey.operator = Operator::Divide;
            } else {
                monkey.value = Some(find_val(&str));
                day.monkeys.insert(monkey.id.clone(), monkey);
                continue;
            }
            data = data[1].trim().split(" ").collect();
            monkey.left = data[0].to_string();
            monkey.right = data[2].to_string();
            day.monkeys.insert(monkey.id.clone(), monkey);
        }

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        // Simple solve for root
        populate(self, "root");
        let answer = self.monkeys["root"].value.unwrap();
        Ok(answer.to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(152.to_string()),
            false => Some(80326079210554u64.to_string()),
        }
    }

    fn solve_part2(&mut self) -> Result<String> {
        // Instead of solving for root, solve for humn

        // Make sure human is none
        let mut human = self.monkeys["humn"].clone();
        human.value = None;
        self.monkeys.insert("humn".to_string(), human);

        // Left side has human for my test and real input
        // This could be automated but I am lazy and assume we need to solve for left
        let root = &self.monkeys["root"];
        let id_left = root.left.clone();
        let id_right = root.right.clone();

        // Solve side right first
        log::debug!("Populate side right, find {}", id_right);
        populate(self, &id_right);

        // Solve as much of left side as possible
        log::debug!("Trying to pre populate side left, find {}", id_left);
        populate(self, &id_left);

        // Now we know what the monkey on left's value should be
        let mut monkey_left = self.monkeys[&id_left].clone();
        let value = self.monkeys[&id_right].value.unwrap();
        log::debug!("Found value {value} for root");
        monkey_left.value = Some(value);
        self.monkeys.insert(id_left.clone(), monkey_left);

        // Now use reverse logic to find human value
        log::debug!("Solve for human using reverse logic");
        populate_backwards(self, "humn");
        let answer = self.monkeys["humn"].value.unwrap();

        Ok(answer.to_string())
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(301.to_string()),
            false => Some(3617613952378u64.to_string()),
        }
    }
}
