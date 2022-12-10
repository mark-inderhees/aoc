use anyhow::Result;

use crate::puzzle::Puzzle;
use crate::utils::cpu::*;
use crate::utils::crt::*;

pub struct Day10 {
    cpu: Cpu,
    crt: Crt,
}

impl Puzzle for Day10 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut day = Day10 {
            cpu: Cpu::new(),
            crt: Crt::new(40, 6),
        };

        for line in input.lines() {
            let instruction = match line {
                "noop" => Instruction::Noop,
                _ => {
                    let parts: Vec<&str> = line.split(" ").collect();
                    let i: i32 = parts[1].parse()?;
                    Instruction::Addx(i)
                }
            };

            let reg_x = day.cpu.get_reg_x();
            day.crt.print_sprite(reg_x as usize);
            let count = Cpu::get_cycle_count(&instruction);
            for _ in 0..count {
                day.crt.step(reg_x);
            }
            day.cpu.run_instruction(&instruction);
        }

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        let mut count = 0;
        for x in (19..220).step_by(40) {
            let state = self.cpu.state_history[x];
            log::debug!("{:#?}", state.reg_x);
            count += (1 + x as i32) * state.reg_x;
        }

        Ok(count.to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(13140.to_string()),
            false => Some(13760.to_string()),
        }
    }

    fn solve_part2(&mut self) -> Result<String> {
        let mut line = self.crt.to_string();
        for _ in 0..6 {
            let lines = line.split_at(40);
            log::debug!("{}", lines.0);
            line = lines.1.to_string();
        }
        Ok(self.crt.to_string())
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some("##..##..##..##..##..##..##..##..##..##..###...###...###...###...###...###...###.####....####....####....####....####....#####.....#####.....#####.....#####.....######......######......######......###########.......#######.......#######.....".to_string()),
            false => Some("###..####.#..#.####..##..###..####.####.#..#.#....#.#.....#.#..#.#..#.#....#....#..#.###..##.....#..#....#..#.###..###..###..#....#.#...#...#....###..#....#....#.#..#....#.#..#....#..#.#....#....#....#..#.#....#..#.####..##..#....####.#....".to_string()),
        }
    }
}