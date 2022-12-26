// 2022 Day 5
// https://adventofcode.com/2022/day/5
// --- Day 5: Supply Stacks ---
// We have stacks of blocks. Need to move around the stacks.
// Can either be simple pop and push each block. Or pull full stack and extend.

use std::collections::VecDeque;

use anyhow::Result;

use crate::puzzle::Puzzle;
use crate::utils::utils::*;

pub struct Day05 {
    stacks: Vec<VecDeque<char>>,
    moves: Vec<Move>,
}

#[derive(Debug)]
struct Move {
    source_index: usize,
    dest_index: usize,
    count: usize,
}

impl Puzzle for Day05 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut day = Day05 {
            stacks: vec![VecDeque::new(); 10],
            moves: Vec::new(),
        };

        // Input is really two parts, the game board with current blocks
        // And the moves to do.
        let inputs: Vec<&str> = input.split("\n\n").collect();
        let board = inputs[0];
        let moves = inputs[1];

        let mut lines: Vec<&str> = board.lines().collect();
        lines.pop(); // Drop index of stacks
        for line in lines {
            // Line is like
            //     [N] [C]
            // [Z] [M] [P]
            // Start at second char and skip by 4s to get box value
            for (i, c) in line.chars().skip(1).step_by(4).enumerate() {
                if c != ' ' {
                    day.stacks[i].push_front(c);
                }
            }
        }

        // Moves look like
        // move 1 from 2 to 1
        for line in moves.lines() {
            let m = find_vals(line);
            day.moves.push(Move {
                count: m[0],
                source_index: m[1] - 1,
                dest_index: m[2] - 1,
            });
        }

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        // For moves, simply pop one item and move it to the destination
        for m in &self.moves {
            for _ in 0..m.count {
                let c = self.stacks[m.source_index].pop_back().unwrap();
                self.stacks[m.dest_index].push_back(c);
            }
        }

        // Answer is what is on top of each stack
        let mut answer = String::new();
        for stack in &mut self.stacks {
            if stack.len() > 0 {
                answer.push(stack.pop_back().unwrap());
            }
        }

        Ok(answer)
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some("CMZ".to_string()),
            false => Some("MQTPGLLDN".to_string()),
        }
    }

    fn solve_part2(&mut self) -> Result<String> {
        // For moves, split off from the source and extend destination
        for m in &self.moves {
            let source = &mut self.stacks[m.source_index];
            let c = source.split_off(source.len() - m.count);
            self.stacks[m.dest_index].extend(c);
        }

        // Answer is what is on top of each stack
        let mut answer = String::new();
        for stack in &mut self.stacks {
            if stack.len() > 0 {
                answer.push(stack.pop_back().unwrap());
            }
        }

        Ok(answer)
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some("MCD".to_string()),
            false => Some("LVZPSTTCZ".to_string()),
        }
    }
}
