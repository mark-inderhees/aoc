use anyhow::Result;

use crate::puzzle::Puzzle;

pub struct Day11 {
    monkeys: Vec<Monkey>,
}

#[derive(Debug)]
enum Operation {
    Multiply(u32),
    Add(u32),
    Square,
    Unknown,
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u32>,
    operation: Operation,
    test: u32,
    if_true: u32,
    if_false: u32,
}

fn get_u32(input: &str) -> u32 {
    let parts: Vec<&str> = input.split(" ").collect();
    parts[1].parse().unwrap()
}

impl Puzzle for Day11 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut day = Day11 { monkeys: vec![] };

        let monkeys = input.split("\r\n\r\n");
        for monkey in monkeys {
            let mut m = Monkey {
                items: vec![],
                operation: Operation::Unknown,
                test: 0,
                if_true: 0,
                if_false: 0,
            };
            let mut lines = monkey.lines();
            _ = lines.next(); // Drop "Monkey 0:"
            let items = &lines.next().unwrap()["  Starting items: ".len()..];
            for item in items.split(", ") {
                m.items.push(item.parse().unwrap());
            }

            let operation = &lines.next().unwrap()["  Operation: new = old ".len()..];
            m.operation = match operation {
                x if x.starts_with("* old") => Operation::Square,
                x if x.starts_with("*") => Operation::Multiply(get_u32(x)),
                x if x.starts_with("+") => Operation::Add(get_u32(x)),
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
        let mut temp_items: Vec<Vec<u32>> = vec![vec![]; self.monkeys.len()];
        let mut count: Vec<u32> = vec![0; self.monkeys.len()];
        let mut round = 0;
        while round < 20 {
            for v in temp_items.iter_mut() {
                v.clear();
            }

            for (m, monkey) in self.monkeys.iter_mut().enumerate() {
                log::debug!("Monkey {m}:");
                monkey.items.extend(temp_items[m].iter());
                temp_items[m].clear();
                monkey.items.retain(|&item| {
                    count[m] += 1;
                    let mut worry = item;
                    log::debug!("  Monkey inspects an item with a worry level of {worry}.");
                    match monkey.operation {
                        Operation::Multiply(x) => {
                            worry *= x;
                            log::debug!("    Worry level is multiplied by {x} to {worry}.");
                        }
                        Operation::Add(x) => {
                            worry += x;
                            log::debug!("Worry level increases by {x} to {worry}.");
                        }
                        Operation::Square => {
                            log::debug!(
                                "    Worry level is multiplied by {worry} to {}.",
                                worry * worry
                            );
                            worry *= worry;
                        }
                        _ => panic!("Unexpected operation"),
                    }
                    worry /= 3;
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
                    log::debug!(
                        "    Item with worry level {worry} is thrown to monkey {destination}."
                    );
                    temp_items[destination as usize].push(worry);

                    false // Never keep in the retain
                });
            }

            for (i, items) in temp_items.iter().enumerate() {
                self.monkeys[i].items.extend(items);
            }

            log::info!(
                "After round {}, the monkeys are holding items with these worry levels:",
                round + 1
            );
            for (m, monkey) in self.monkeys.iter().enumerate() {
                log::info!("Monkey {}: {:?}", m, monkey.items);
            }
            round += 1;
        }
        log::debug!("{count:?}");
        count.sort();
        count.reverse();
        log::debug!("{count:?}");
        let answer = count[0] * count[1];
        Ok(answer.to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => None,
            false => None,
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
