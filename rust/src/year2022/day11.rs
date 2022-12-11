use anyhow::Result;

use crate::puzzle::Puzzle;
use crate::utils::utils::*;

pub struct Day11 {
    monkeys: Vec<Monkey>,
}

#[derive(Debug)]
enum Operation {
    Multiply(u64),
    Add(u64),
    Square,
    Unknown,
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test: u64,
    if_true: u64,
    if_false: u64,
}

fn get_u64(input: &str) -> u64 {
    let parts: Vec<&str> = input.split(" ").collect();
    parts[1].parse().unwrap()
}

fn answer(day: &mut Day11, max_rounds: u32, worry_reducer: u64) -> u64 {
    for monkey in &day.monkeys {
        log::debug!("{:#?}", monkey.test);
    }
    let magic = day.monkeys.iter().fold(1, |a, m| a * m.test);
    let mut temp_items: Vec<Vec<u64>> = vec![vec![]; day.monkeys.len()];
    let mut count: Vec<u64> = vec![0; day.monkeys.len()];
    let mut round = 0;
    while round < max_rounds {
        for v in temp_items.iter_mut() {
            v.clear();
        }

        for (m, monkey) in day.monkeys.iter_mut().enumerate() {
            log::debug!("Monkey {m}:");
            monkey.items.extend(temp_items[m].iter());
            temp_items[m].clear();
            monkey.items.retain(|&item| {
                count[m] += 1;
                let mut worry = item % magic;
                log::debug!("  Monkey inspects an item with a worry level of {worry}.");
                match monkey.operation {
                    Operation::Multiply(x) => {
                        worry = worry * x;
                        log::debug!("    Worry level is multiplied by {x} to {worry}.");
                    }
                    Operation::Add(x) => {
                        worry = worry + x;
                        log::debug!("Worry level increases by {x} to {worry}.");
                    }
                    Operation::Square => {
                        log::debug!(
                            "    Worry level is multiplied by {worry} to {}.",
                            worry * worry
                        );
                        worry = worry * worry;
                    }
                    _ => panic!("Unexpected operation"),
                }
                worry = worry / worry_reducer;
                log::debug!(
                    "    Monkey gets bored with item. Worry level is divided by 3 to {worry}."
                );
                let destination = if worry % monkey.test == 0 {
                    log::debug!("    Current worry level is divisible by {}.", monkey.test);
                    monkey.if_true
                } else {
                    log::debug!(
                        "    Current worry level is not divisible by {}.",
                        monkey.test
                    );
                    monkey.if_false
                };
                log::debug!("    Item with worry level {worry} is thrown to monkey {destination}.");
                temp_items[destination as usize].push(worry);

                false // Never keep in the retain
            });
        }

        for (i, items) in temp_items.iter().enumerate() {
            day.monkeys[i].items.extend(items);
        }

        log::debug!(
            "After round {}, the monkeys are holding items with these worry levels:",
            round + 1
        );
        for (m, monkey) in day.monkeys.iter().enumerate() {
            log::debug!("Monkey {}: {:?}", m, monkey.items);
        }

        let bob = [1, 20, 1000, 2000];
        if bob.contains(&(round + 1)) {
            log::debug!("== After round {} ==", round + 1);
            for (i, c) in count.iter().enumerate() {
                log::debug!("Monkey {i} inspected items {c} times.");
            }
        }

        round += 1;
    }

    log::debug!("{count:?}");
    count.sort();
    count.reverse();
    log::debug!("{count:?}");
    count[0] * count[1]
}

impl Puzzle for Day11 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut day = Day11 { monkeys: vec![] };

        let monkeys = input.split("\r\n\r\n");
        for monkey in monkeys {
            let mut lines = monkey.lines();
            _ = lines.next(); // Drop "Monkey 0:"
            let mut m = Monkey {
                items: get_vals::<u64>(lines.next().unwrap()),
                operation: Operation::Unknown,
                test: 0,
                if_true: 0,
                if_false: 0,
            };

            let operation = &lines.next().unwrap()["  Operation: new = old ".len()..];
            m.operation = match operation {
                x if x.starts_with("* old") => Operation::Square,
                x if x.starts_with("*") => Operation::Multiply(get_u64(x)),
                x if x.starts_with("+") => Operation::Add(get_u64(x)),
                _ => panic!("Unexpected operation input"),
            };

            m.test = lines.next().unwrap()["  Test: divisible by ".len()..]
                .parse()
                .unwrap();
            m.if_true = lines.next().unwrap()["    If true: throw to monkey ".len()..]
                .parse()
                .unwrap();
            m.if_false = lines.next().unwrap()["    If false: throw to monkey ".len()..]
                .parse()
                .unwrap();

            day.monkeys.push(m);
        }

        log::debug!("{:#?}", day.monkeys);

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        let answer = answer(self, 20, 3);
        Ok(answer.to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(10605.to_string()),
            false => Some(50830.to_string()),
        }
    }

    fn solve_part2(&mut self) -> Result<String> {
        let answer = answer(self, 10000, 1);
        Ok(answer.to_string())
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(2713310158u64.to_string()),
            false => Some(14399640002u64.to_string()),
        }
    }
}
