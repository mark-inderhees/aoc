use anyhow::Result;
use std::fs;

use crate::puzzle::Puzzle;
use crate::utils::board::*;
use crate::utils::tetris::*;

#[allow(unused_imports)]
use crate::utils::utils::*;

pub struct Day17 {
    commands: Vec<Direction>,
    tetris: Tetris,
}

impl Puzzle for Day17 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut day = Day17 {
            commands: vec![],
            tetris: Tetris::new(),
        };

        for char in input.trim().to_string().chars() {
            match char {
                '<' => day.commands.push(Direction::Left),
                '>' => day.commands.push(Direction::Right),
                _ => panic!("Unexpected char"),
            }
        }

        // let mut id = day.tetris.add_shape(Shapes::Flat);
        // day.tetris.print();
        // day.tetris.move_shape(id, Direction::Down);
        // day.tetris.move_shape(id, Direction::Down);
        // day.tetris.move_shape(id, Direction::Down);
        // id = day.tetris.add_shape(Shapes::Square);
        // day.tetris.print();
        // day.tetris.move_shape(id, Direction::Right);
        // day.tetris.print();
        // day.tetris.move_shape(id, Direction::Down);
        // day.tetris.print();
        // day.tetris.move_shape(id, Direction::Right);
        // day.tetris.print();
        // day.tetris.move_shape(id, Direction::Left);
        // day.tetris.print();
        // panic!("Hi mark");

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        let raw_input = fs::read_to_string("input/2022/day17_help.test").expect("Input file error");
        let mut help: Vec<u32> = vec![];
        for line in raw_input.lines() {
            help.push(get_val(line));
        }

        let mut command_index = 0;
        let mut shape_index = 0;
        let mut shape_count = 0;
        let shapes = vec![
            Shapes::Flat,
            Shapes::Plus,
            Shapes::L,
            Shapes::Tall,
            Shapes::Square,
        ];
        let total = 2022;
        log::info!("Shapes {}. Commands {}.", shapes.len(), self.commands.len());
        #[derive(Clone, Copy)]
        struct RepeatInfo {
            shape_index: usize,
            command_index: usize,
            height: u32,
        }
        let mut repeats = vec![];
        while shape_count < total {
            repeats.push(RepeatInfo {
                shape_index,
                command_index,
                height: self.tetris.get_stack_height(),
            });

            if self.tetris.is_top_line_full() {
                log::info!("Top line full at shape #{shape_count}");
            }

            let shape = shapes[shape_index];
            let shape_id = self.tetris.add_shape(shape);

            let shape_count1 = shape_count;
            let shape_index1 = shape_index;
            let command_index1 = command_index;

            loop {
                let command = self.commands[command_index];
                command_index = (command_index + 1) % self.commands.len();
                if shape_count == 21 {
                    // self.tetris.print();
                }
                self.tetris.move_shape(shape_id, command);
                if shape_count == 21 {
                    // self.tetris.print();
                }
                if !self.tetris.move_shape(shape_id, Direction::Down) {
                    break;
                }
            }

            log::debug!(
                "Round {} Start P: {} Start W: {} Tower height: {}",
                shape_count1,
                shape_index1,
                command_index1,
                self.tetris.get_stack_height()
            );

            // self.tetris.print();
            // assert_eq!(help[shape_count], self.tetris.get_stack_height());

            shape_count += 1;
            shape_index = shape_count % shapes.len();
        }

        let mut skip_to_use = 0;
        let mut step_to_use = 0;
        let mut diff_to_use = 0;

        let mut done = false;
        for skip in 0..1000 {
            for step in 1..1000 {
                let mut previous = RepeatInfo {
                    shape_index: 0,
                    command_index: 0,
                    height: 0,
                };
                let mut diff = 0;
                let mut good = false;
                for repeat in repeats.iter().skip(skip).step_by(step) {
                    if previous.height != 0 {
                        if diff != 0 {
                            good = true;
                            let new_diff = repeat.height - previous.height;
                            if new_diff != diff
                                || previous.shape_index != repeat.shape_index
                                || previous.command_index != repeat.command_index
                            {
                                good = false;
                                break;
                            }
                        }
                        diff = repeat.height - previous.height
                    }
                    previous = *repeat;
                }
                if good {
                    log::info!("Potential repeat at skip {skip} step {step}, diff {diff}");
                    skip_to_use = skip as u64;
                    step_to_use = step as u64;
                    diff_to_use = diff as u64;
                    done = true;
                    break;
                }
            }
            if done {
                break;
            }
        }

        let part2 = (1000000000000u64 - skip_to_use) / step_to_use * diff_to_use
            + repeats[skip_to_use as usize].height as u64;
        println!("part2 {part2}");

        Ok(self.tetris.get_stack_height().to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(3068.to_string()),
            false => Some(3177.to_string()),
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
