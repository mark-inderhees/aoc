// 2015 Day 23
// https://adventofcode.com/2015/day/23
// --- Day 23: Opening the Turing Lock ---
// Run a program. Reading comprehension, jio is jump if ONE.

use anyhow::Result;

use crate::puzzle::Puzzle;
use crate::utils::cpu::*;
use crate::utils::utils::*;

pub struct Day23 {
    cpu: Cpu,
}

/// Helper to parse input, find the register from a string
fn register_from_str(string: &str) -> Register {
    match string {
        "a" => Register::A,
        "b" => Register::B,
        _ => panic!("Unexpected register"),
    }
}

impl Puzzle for Day23 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut day = Day23 { cpu: Cpu::new() };

        for line in input.lines() {
            // Each line is a basic instruction
            let line = line.replace(",", "");
            let splits: Vec<&str> = line.split(" ").collect();
            let register = splits[1];
            let instruction = match splits[0] {
                "hlf" => Instruction::Half(register_from_str(register)),
                "tpl" => Instruction::Triple(register_from_str(register)),
                "inc" => Instruction::Increment(register_from_str(register)),
                "jmp" => Instruction::Jump(find_val(&line)),
                "jie" => Instruction::JumpIfEven(register_from_str(register), find_val(&line)),
                "jio" => Instruction::JumpIfOne(register_from_str(register), find_val(&line)),
                _ => panic!("Unexpected instruction"),
            };
            day.cpu.add_instruction(&instruction);
        }

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        // What is in reg B at the end?
        self.cpu.run();
        let answer = self.cpu.reg(Register::B);
        Ok(answer.to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(0.to_string()),
            false => Some(255.to_string()),
        }
    }

    fn solve_part2(&mut self) -> Result<String> {
        // What is in reg B at the end, if A starts a 1?
        self.cpu.set_reg(Register::A, 1);
        self.cpu.run();
        let answer = self.cpu.reg(Register::B);
        Ok(answer.to_string())
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(0.to_string()),
            false => Some(334.to_string()),
        }
    }
}
