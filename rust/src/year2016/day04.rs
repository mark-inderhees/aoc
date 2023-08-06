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
    letters: Vec<(char, u32)>,
}

pub struct Day04 {
    rooms: Vec<Room>,
}

impl Puzzle for Day04 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut day = Day04 { rooms: vec![] };

        for line in input.trim().split('\n') {
            let mut line_str = line.to_string();
            let last_dash = line_str.rfind("-").unwrap();
            let name = line_str.drain(..last_dash).collect();
            let first_bracket = line_str.find("[").unwrap();
            let sector_id: String = line_str.drain(1..first_bracket).collect();
            let sector_id = find_val(&sector_id);
            let last_bracket = line_str.find("]").unwrap();
            let checksum = line_str.drain(2..last_bracket).collect();
            day.rooms.push(Room {
                name,
                sector_id,
                checksum,
                letters: vec![],
            });
        }
        log::debug!("Rooms {:?}", day.rooms);

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        let mut sum = 0;
        for room in self.rooms.iter_mut() {
            for letter in 'a'..='z' {
                let mut count = 0;
                for c in room.name.chars() {
                    if c == letter {
                        count += 1
                    };
                }
                room.letters.push((letter, count));
            }
            room.letters
                .sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
            let checksum = &room.letters[0..5];
            let mut dst = [0u8];
            let checksum = checksum
                .iter()
                .fold("".to_string(), |acc, x| acc + x.0.encode_utf8(&mut dst));
            if checksum == room.checksum {
                sum += room.sector_id;
            }
        }
        Ok(sum.to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(1514.to_string()),
            false => Some(185371.to_string()),
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
