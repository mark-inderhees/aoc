use std::collections::VecDeque;

use anyhow::Result;

use crate::puzzle::Puzzle;

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

        let inputs: Vec<&str> = input.split("\r\n\r\n").collect();
        let board = inputs[0];
        let moves = inputs[1];

        let mut lines: Vec<&str> = board.lines().collect();
        lines.pop(); // Drop index of stacks
        for line in lines {
            for (i, c) in line.chars().skip(1).step_by(4).enumerate() {
                if c != ' ' {
                    day.stacks[i].push_front(c);
                }
            }
        }

        for line in moves.lines() {
            let m: Vec<usize> = line
                .split(" ")
                .skip(1)
                .step_by(2)
                .collect::<Vec<&str>>()
                .iter()
                .map(|a| a.parse::<usize>().unwrap())
                .collect();
            day.moves.push(Move {
                count: m[0],
                source_index: m[1] - 1,
                dest_index: m[2] - 1,
            });
        }

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        for m in &self.moves {
            for _ in 0..m.count {
                let c = self.stacks[m.source_index].pop_back().unwrap();
                self.stacks[m.dest_index].push_back(c);
            }
        }

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
        for m in &self.moves {
            let source = &mut self.stacks[m.source_index];
            let c = source.split_off(source.len() - m.count);
            self.stacks[m.dest_index].extend(c);
        }

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
