// 2022 Day 13
// https://adventofcode.com/2022/day/13
// --- Day 13: Distress Signal ---
// Built lists of lists and compare based on special rules
// The key is to treat the input as a job vector
// Walk the lists in unison to do the compare
// Some fancy rules result in mutating a list as the game goes along

use anyhow::Result;
use std::vec;

use crate::puzzle::Puzzle;
use crate::utils::utils::*;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum PacketData {
    Integer(u32),
    ListStart,
    ListEnd,
}

#[derive(Debug, Clone)]
struct Pair {
    left: Vec<PacketData>,
    right: Vec<PacketData>,
}

pub struct Day13 {
    pairs: Vec<Pair>,
}

fn play_game_order(
    left_vec: &mut Vec<PacketData>,
    right_vec: &mut Vec<PacketData>,
    pair_i: usize,
) -> std::cmp::Ordering {
    let is_less = play_game(left_vec, right_vec, pair_i);
    if is_less {
        return std::cmp::Ordering::Less;
    }

    std::cmp::Ordering::Greater
}

fn play_game(
    left_vec: &mut Vec<PacketData>,
    right_vec: &mut Vec<PacketData>,
    pair_i: usize,
) -> bool {
    let mut index = 0;
    let mut depth = 2;
    let empty = "";
    // log::debug!("- Compare {:?} vs {:?}", pair.left, pair.right);
    log::debug!("\n== Pair {pair_i} ==");
    loop {
        let left = &left_vec[index];
        let right = &right_vec[index];

        match (left, right) {
            (PacketData::Integer(left_int), PacketData::Integer(right_int)) => {
                log::debug!("{empty:>depth$}- Compare {left_int} {right_int}");
                if left_int < right_int {
                    depth += 2;
                    log::debug!(
                        "{empty:>depth$}- Left side is smaller, so inputs are in the right order"
                    );
                    return true;
                } else if left_int > right_int {
                    depth += 2;
                    log::debug!(
                    "{empty:>depth$}- Right side is smaller, so inputs are not in the right order"
                );
                    return false;
                }
            }
            (PacketData::ListStart, PacketData::ListStart) => {
                log::debug!("{empty:>depth$}- Compare lists");
                depth += 2;
            }
            (PacketData::ListEnd, PacketData::ListEnd) => {
                depth -= 2;
            }
            (PacketData::ListEnd, _) => {
                log::debug!(
                    "{empty:>depth$}- Left side ran out of items, so inputs are in the right order"
                );
                return true;
            }
            (_, PacketData::ListEnd) => {
                log::debug!(
                    "{empty:>depth$}- Right side ran out of items, so inputs are not in the right order"
                );
                return false;
            }
            (PacketData::Integer(left_int), _) => {
                log::debug!("{empty:>depth$}- Mixed types; convert left to [{left_int}] and retry comparison");
                if left_vec.len() == index + 1 {
                    left_vec.push(PacketData::ListEnd);
                } else {
                    left_vec.insert(index + 1, PacketData::ListEnd);
                }
                left_vec.insert(index, PacketData::ListStart);
                continue;
            }
            (_, PacketData::Integer(right_int)) => {
                log::debug!("{empty:>depth$}- Mixed types; convert right to [{right_int}] and retry comparison");
                if right_vec.len() == index + 1 {
                    right_vec.push(PacketData::ListEnd);
                } else {
                    right_vec.insert(index + 1, PacketData::ListEnd);
                }
                right_vec.insert(index, PacketData::ListStart);
                continue;
            }
        }

        index += 1;
    }
}

fn get_list(s: &String) -> &str {
    // Dexpect first char to be "["
    let mut chars = s.chars();
    assert!('[' == chars.next().unwrap());

    let mut count = 1;
    let mut index = 0;
    while count > 0 {
        index += 1;
        let char = chars.next().unwrap();
        if char == '[' {
            count += 1;
        } else if char == ']' {
            count -= 1;
        }
    }

    s.split_at(index + 1).0
}

fn process_line(line: &String, datas: &mut Vec<PacketData>) {
    let mut depth = 0;
    log::trace!("");
    for (i, c) in line.chars().enumerate() {
        if c == '[' {
            let split = line.split_at(i).1.to_string();
            let list = get_list(&split);
            log::trace!("[{depth}] {list} - list start");
            datas.push(PacketData::ListStart);
            depth += 1;
        } else if c == ',' {
            // do nothing
        } else if c == ']' {
            log::trace!("[{depth}] list end");
            datas.push(PacketData::ListEnd);
            depth -= 1;
        } else if c == '\n' {
        } else {
            // get single value
            let split = line.split_at(i).1.to_string().replace("]", "");
            if split.len() > 0 {
                let value: u32 = find_val(&split);
                datas.push(PacketData::Integer(value));
                log::trace!("[{depth}] {value}");
            }
        }
    }
}

impl Puzzle for Day13 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut day = Day13 { pairs: vec![] };

        let pairs_txt = input.split("\n\n");

        for pair_txt in pairs_txt {
            let mut lines = pair_txt.lines();
            let left = lines.next().unwrap().to_string();
            let right = lines.next().unwrap().to_string();

            let mut pair = Pair {
                left: vec![],
                right: vec![],
            };
            process_line(&left, &mut pair.left);
            process_line(&right, &mut pair.right);
            day.pairs.push(pair);
        }

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        log::debug!("{:?}", self.pairs);
        let mut good: Vec<usize> = vec![];
        for (i, pair) in self.pairs.iter_mut().enumerate() {
            if play_game(&mut pair.left, &mut pair.right, i + 1) {
                good.push(i + 1);
            }
        }
        log::debug!("{:?}", good);
        Ok(good.iter().sum::<usize>().to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(13.to_string()),
            false => Some(5843.to_string()),
        }
    }

    fn solve_part2(&mut self) -> Result<String> {
        let mut data: Vec<Vec<PacketData>> = vec![];
        for pair in self.pairs.iter_mut() {
            data.push(pair.left.clone());
            data.push(pair.right.clone());
        }
        let mut two: Vec<PacketData> = vec![];
        two.push(PacketData::ListStart);
        two.push(PacketData::ListStart);
        two.push(PacketData::Integer(2));
        two.push(PacketData::ListEnd);
        two.push(PacketData::ListEnd);
        let mut six: Vec<PacketData> = vec![];
        six.push(PacketData::ListStart);
        six.push(PacketData::ListStart);
        six.push(PacketData::Integer(6));
        six.push(PacketData::ListEnd);
        six.push(PacketData::ListEnd);
        data.push(two.clone());
        data.push(six.clone());
        data.sort_by(|a, b| play_game_order(&mut a.clone(), &mut b.clone(), 0));
        let loc2 = data.iter().enumerate().find(|&x| *x.1 == two).unwrap().0;
        let loc6 = data.iter().enumerate().find(|&x| *x.1 == six).unwrap().0;
        log::trace!("{:#?} {loc2} {loc6}", data);
        let answer = (loc2 + 1) * (loc6 + 1);
        Ok(answer.to_string())
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(140.to_string()),
            false => Some(26289.to_string()),
        }
    }
}
