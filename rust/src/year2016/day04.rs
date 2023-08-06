// 2016 Day 4
// https://adventofcode.com/2016/day/4
// --- Day 4: Security Through Obscurity ---
// Decode room names with checksums and char math

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
    valid: bool,
}

pub struct Day04 {
    rooms: Vec<Room>,
}

// Given a char and a offset value, incremente the char value. Lower case only,
// if goes beyond z, then wrap around to a. Also convert - to space.
fn add_to_char(c: char, value: u32) -> char {
    // Convert dash to space
    if c == '-' {
        return ' ';
    }

    // Check for wrap around
    let u = c as u32;
    let z = 'z' as u32;
    if u + value > z {
        // There is wrap around, add the extra to a
        let extra = u + value - z - 1;
        return char::from_u32('a' as u32 + extra).unwrap();
    }

    // No wrap around, simple math
    char::from_u32(u + value).unwrap()
}

impl Puzzle for Day04 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut day = Day04 { rooms: vec![] };

        for line in input.trim().split('\n') {
            // Read in the room info, it contains a roomname-sectorid[checksum]
            // Where roomname has dashes in it
            let mut line_str = line.to_string();
            let last_dash = line_str.rfind("-").unwrap();
            let name: String = line_str.drain(..last_dash).collect();
            let first_bracket = line_str.find("[").unwrap();
            let sector_id: String = line_str.drain(1..first_bracket).collect();
            let sector_id = find_val(&sector_id);
            let last_bracket = line_str.find("]").unwrap();
            let checksum: String = line_str.drain(2..last_bracket).collect();

            // Now validate the checksum to see if it's a real room
            // Check sum is the frequency of the chars sorted in order
            // If a char count is the same, then sort alphabetical

            // Start by counting how many of each letter there are
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

            // Now sort the letters by frequency then alphabetical
            letters.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));

            // Now build the checksum and check if it's valid
            let checksum2 = &letters[0..5];
            let checksum2 = checksum2
                .iter()
                .fold("".to_string(), |acc, x| acc + &x.0.to_string());
            let valid = checksum == checksum2;

            day.rooms.push(Room {
                name,
                sector_id,
                valid,
            });
        }

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        // Sum the sector ids of valid rooms
        let sum = self.rooms.iter().fold(0, |acc, x| match x.valid {
            true => acc + x.sector_id,
            false => acc,
        });
        Ok(sum.to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(2841.to_string()),
            false => Some(185371.to_string()),
        }
    }

    fn solve_part2(&mut self) -> Result<String> {
        // Decode the room name and look for "northpole object storage"
        // To decode, increment each char by sector_id
        let mut sector_id = 0;
        for room in &self.rooms {
            let offset = room.sector_id % 26;
            if room.valid {
                let real_name = room.name.chars().fold("".to_string(), |acc, x| {
                    acc + &add_to_char(x, offset).to_string()
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
            true => Some(984.to_string()),
            false => Some(984.to_string()),
        }
    }
}
