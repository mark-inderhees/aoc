use std::collections::HashMap;

use anyhow::Result;

use crate::puzzle::Puzzle;

#[allow(unused_imports)]
use crate::utils::utils::*;

pub struct Day21 {
    monkeys: HashMap<String, Monkey>,
}

#[derive(Default, Debug, Clone, Copy)]
enum Operator {
    #[default]
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Default, Debug, Clone)]
struct Monkey {
    value: Option<i128>,
    id: String,
    operator: Operator,
    part1: String,
    part2: String,
    try_count: u32,
}

fn do_math(
    operator: Operator,
    value1: i128,
    value2: i128,
    invert_operations: bool,
    i_am_part_1: bool,
) -> i128 {
    if invert_operations {
        let key = value1;
        match operator {
            Operator::Add => key - value2,
            Operator::Subtract => {
                if i_am_part_1 {
                    return key + value2;
                } else {
                    return value2 - key;
                }
            }
            Operator::Multiply => key / value2,
            Operator::Divide => {
                if i_am_part_1 {
                    return key * value2
                }else {
                    return value2 / key;
                }
            }
        }
    } else {
        match operator {
            Operator::Add => value1 + value2,
            Operator::Subtract => value1 - value2,
            Operator::Multiply => value1 * value2,
            Operator::Divide => value1 / value2,
        }
    }
}

fn populate(day: &mut Day21, start_id: &str, invert_operations: bool) {
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

            if monkey.part1 == "" || monkey.part2 == "" {
                continue;
            }

            let monkey1 = &day.monkeys[&monkey.part1];
            let monkey2 = &day.monkeys[&monkey.part2];

            if monkey.try_count > 2 {
                continue;
            }

            if let (Some(value1), Some(value2)) = (monkey1.value, monkey2.value) {
                // We can comput now, so do it!
                monkey.value = Some(do_math(monkey.operator, value1, value2, invert_operations, false));
                log::debug!(
                    "Doing math {} = {} {:?} {}, {} {:?} {} = {}",
                    monkey.id,
                    monkey1.id,
                    monkey.operator,
                    monkey2.id,
                    value1,
                    monkey.operator,
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
            jobs.extend(new_jobs);
        }
    }
}

fn find_monkeys(day: &Day21, my_id: &str) -> (String, String, Operator, bool) {
    // Search the hash map, find the monkey that depends on me
    for (id1, monkey) in day.monkeys.iter() {
        if monkey.part1 == my_id {
            return (
                id1.to_string(),
                monkey.part2.to_string(),
                monkey.operator,
                true,
            );
        } else if monkey.part2 == my_id {
            return (
                id1.to_string(),
                monkey.part1.to_string(),
                monkey.operator,
                false,
            );
        }
    }
    panic!("Could not find your monkey");
}

fn populate_backwards(day: &mut Day21, start_id: &str, invert_operations: bool) {
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
                monkey.value = Some(do_math(
                    operator,
                    value1,
                    value2,
                    invert_operations,
                    i_am_part_1,
                ));
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
            monkey.part1 = data[0].to_string();
            monkey.part2 = data[2].to_string();
            day.monkeys.insert(monkey.id.clone(), monkey);
        }

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        populate(self, "root", false);
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
        // Side 1 has human for my test and real input
        let root = &self.monkeys["root"];
        let id1 = root.part1.clone();
        let id2 = root.part2.clone();

        // Make sure human is none
        let mut human = self.monkeys["humn"].clone();
        human.value = None;
        self.monkeys.insert("humn".to_string(), human);

        // Solve side 2 first
        log::debug!("Populate side 2, find {}", id2);
        populate(self, &id2, false);

        log::debug!("Trying to pre populate side 1, find {}", id1);
        populate(self, &id1, false);

        // Now we know what monkey on side 1 value should be
        let mut monkey1 = self.monkeys[&id1].clone();
        let value = self.monkeys[&id2].value.unwrap();
        log::debug!("Found value {value} for root");
        monkey1.value = Some(value);
        self.monkeys.insert(id1.clone(), monkey1);

        // Now use reverse logic to find human value
        log::debug!("Solve for human using inverse logic");
        // log::debug!("Monkies {:#?}", self.monkeys);
        populate_backwards(self, "humn", true);
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
