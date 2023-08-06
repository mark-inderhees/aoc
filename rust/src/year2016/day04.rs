// 2016 Day 4
// https://adventofcode.com/2016/day/4

use anyhow::Result;

use crate::puzzle::Puzzle;

#[allow(unused_imports)]
use crate::utils::utils::*;

#[allow(unused_imports)]
use std::collections::VecDeque;

#[derive(Debug)]
struct Room {
    name: String,
    sector_id: u32,
    checksum: String,
    valid: bool,
}

pub struct Day04 {
    rooms: Vec<Room>,
}

fn add_chars(c: char, value: u32) -> char {
    if c == '-' {
        return ' ';
    }

    let u = c as u32;
    let z = 'z' as u32;
    if u + value > z {
        let extra = u + value - z - 1;
        return char::from_u32('a' as u32 + extra).unwrap();
    }
    char::from_u32(u + value).unwrap()
}

impl Puzzle for Day04 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut day = Day04 { rooms: vec![] };

        for line in input.trim().split('\n') {
            let mut line_str = line.to_string();
            let last_dash = line_str.rfind("-").unwrap();
            let name: String = line_str.drain(..last_dash).collect();
            let first_bracket = line_str.find("[").unwrap();
            let sector_id: String = line_str.drain(1..first_bracket).collect();
            let sector_id = find_val(&sector_id);
            let last_bracket = line_str.find("]").unwrap();
            let checksum: String = line_str.drain(2..last_bracket).collect();

            let mut letters: Vec<(char, u32)> = vec![];
            for letter in 'a'..='z' {
                let mut count = 0;
                for c in name.chars() {
                    if c == letter {
                        count += 1
                    };
                }
                letters.push((letter, count));
            }
            letters.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
            let checksum2 = &letters[0..5];
            let mut dst = [0u8];
            let checksum2 = checksum2
                .iter()
                .fold("".to_string(), |acc, x| acc + x.0.encode_utf8(&mut dst));
            let valid = checksum == checksum2;

            day.rooms.push(Room {
                name,
                sector_id,
                checksum,
                valid,
            });
        }

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        let mut sum = self.rooms.iter().fold(0, |acc, x| match x.valid {
            true => acc + x.sector_id,
            false => acc,
        });
        Ok(sum.to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(1514.to_string()),
            false => Some(185371.to_string()),
        }
    }

    fn solve_part2(&mut self) -> Result<String> {
        let mut sector_id = 0;
        for room in &self.rooms {
            let offset = room.sector_id % 26;
            if room.valid {
                let real_name = room.name.chars().fold("".to_string(), |acc, x| {
                    acc + &add_chars(x, offset).to_string()
                });
                if real_name == "northpole object storage" {
                    sector_id = room.sector_id;
                }
            }
        }
        Ok(sector_id.to_string())
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => None,
            false => Some(984.to_string()),
        }
    }
}
