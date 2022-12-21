use anyhow::Result;
use std::collections::HashMap;

use crate::puzzle::Puzzle;

#[allow(unused_imports)]
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

fn populate(day: &mut Day21, start_id: &str) {
    #[derive(Clone)]
    struct Work {
        id: String,
    }
    let job = Work {
        id: start_id.to_string(),
    };
    let mut jobs = vec![job];

    for monkey in day.monkeys.values_mut() {
        monkey.try_count = 0;
    }

    while jobs.len() > 0 {
        let job = jobs.pop().unwrap();
        let mut monkey = day.monkeys[&job.id].clone();
        monkey.try_count += 1;
        day.monkeys.insert(job.id.clone(), monkey.clone());

        if let Some(_value) = monkey.value {
            panic!("Not expected populated value");
        } else {
            // We have work to do, check if operators are available, else schedule work
            let mut new_jobs = vec![job.clone()];

            if monkey.left == "" || monkey.right == "" {
                continue;
            }

            let monkey_left = &day.monkeys[&monkey.left];
            let monkey_right = &day.monkeys[&monkey.right];

            if monkey.try_count > 2 {
                continue;
            }

            if let (Some(value1), Some(value2)) = (monkey_left.value, monkey_right.value) {
                // We can comput now, so do it!
                monkey.value = Some(do_math(monkey.operator, value1, value2));
                log::debug!(
                    "Doing math {} = {} {:?} {}, {} {:?} {} = {}",
                    monkey.id,
                    monkey_left.id,
                    monkey.operator,
                    monkey_right.id,
                    value1,
                    monkey.operator,
                    value2,
                    monkey.value.unwrap()
                );
                day.monkeys.insert(job.id.clone(), monkey);
                continue;
            }
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

            jobs.extend(new_jobs);
        }
    }
}

fn find_monkeys(day: &Day21, my_id: &str) -> (String, String, Operator, bool) {
    // Search the hash map, find the monkey that depends on me
    for (key, monkey) in day.monkeys.iter() {
        if monkey.left == my_id {
            return (
                key.to_string(),
                monkey.right.to_string(),
                monkey.operator,
                true,
            );
        } else if monkey.right == my_id {
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
        let job = jobs.pop().unwrap();
        let mut monkey = day.monkeys[&job.id].clone();

        if let Some(_value) = monkey.value {
            panic!("Not expected populated value");
        } else {
            // We have work to do, check if operators are available, else schedule work
            let mut new_jobs = vec![job.clone()];

            let monkey_ids = find_monkeys(day, &job.id);
            let monkey_id1 = monkey_ids.0;
            let monkey_id2 = monkey_ids.1;
            let operator = monkey_ids.2;
            let i_am_part_1 = monkey_ids.3;
            let monkey1 = &day.monkeys[&monkey_id1];
            let monkey2 = &day.monkeys[&monkey_id2];

            if let (Some(value1), Some(value2)) = (monkey1.value, monkey2.value) {
                // We can comput now, so do it!
                monkey.value = Some(solve_math(operator, value1, value2, i_am_part_1));
                log::debug!(
                    "Doing math {} = {} {:?} {}, {} {:?} {} = {}",
                    monkey.id,
                    monkey1.id,
                    operator,
                    monkey2.id,
                    value1,
                    operator,
                    value2,
                    monkey.value.unwrap()
                );
                day.monkeys.insert(job.id.clone(), monkey);
                continue;
            }

            if monkey2.value.is_none() {
                new_jobs.push(Work {
                    id: monkey2.id.clone(),
                });
            }
            if monkey1.value.is_none() {
                new_jobs.push(Work {
                    id: monkey1.id.clone(),
                });
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
                monkey.value = Some(get_val(&str));
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
        populate(self, "root");
        // log::debug!("Monkies {:#?}", self.monkeys);
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
        // Left side has human for my test and real input
        let root = &self.monkeys["root"];
        let id_left = root.left.clone();
        let id_right = root.right.clone();

        // Make sure human is none
        let mut human = self.monkeys["humn"].clone();
        human.value = None;
        self.monkeys.insert("humn".to_string(), human);

        // Solve side 2 first
        log::debug!("Populate side 2, find {}", id_right);
        populate(self, &id_right);

        log::debug!("Trying to pre populate side 1, find {}", id_left);
        populate(self, &id_left);

        // Now we know what monkey on side 1 value should be
        let mut monkey1 = self.monkeys[&id_left].clone();
        let value = self.monkeys[&id_right].value.unwrap();
        log::debug!("Found value {value} for root");
        monkey1.value = Some(value);
        self.monkeys.insert(id_left.clone(), monkey1);

        // Now use reverse logic to find human value
        log::debug!("Solve for human using inverse logic");
        // log::debug!("Monkies {:#?}", self.monkeys);
        populate_backwards(self, "humn");
        let answer = self.monkeys["humn"].value.unwrap();

        // log::debug!("Monkies {:#?}", self.monkeys);
        Ok(answer.to_string())
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(301.to_string()),
            false => Some(3617613952378u64.to_string()),
        }
    }
}
