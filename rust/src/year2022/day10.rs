use anyhow::Result;

use crate::puzzle::Puzzle;
use crate::utils::cpu::*;

pub struct Day10 {
    cpu: Cpu,
}

impl Puzzle for Day10 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut day = Day10 { cpu: Cpu::new() };

        for line in input.lines() {
            let instruction = match line {
                "noop" => Instruction::Noop,
                _ => {
                    let parts: Vec<&str> = line.split(" ").collect();
                    let i: i32 = parts[1].parse()?;
                    Instruction::Addx(i)
                }
            };
            day.cpu.add_instruction(instruction);
        }

        day.cpu.run();

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        // log::debug!("{:#?}", self.cpu.state_history.iter().enumerate());
        log::debug!("{}", self.cpu.state_history.len());
        for (i,x) in (0..220).enumerate(){
            let state = self.cpu.state_history[x];
            // log::debug!("{} {}", i, state.reg_x);
        }

        log::debug!("hi mark");

        let mut count = 0;
        for x in (19..220).step_by(40){
            let state = self.cpu.state_history[x];
            log::debug!("{:#?}", state.reg_x);
            count += (1 + x as i32) * state.reg_x;
        }

        Ok(count.to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(13140.to_string()),
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
