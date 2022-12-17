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
        while shape_count < total {
            let shape = shapes[shape_index];
            let shape_id = self.tetris.add_shape(shape);

            self.tetris.print();

            loop {
                let command = self.commands[command_index];
                command_index = (command_index + 1) % self.commands.len();
                self.tetris.move_shape(shape_id, command);
                if !self.tetris.move_shape(shape_id, Direction::Down) {
                    break;
                }
            }

            self.tetris.print();
            assert_eq!(help[shape_count], self.tetris.get_stack_height());

            shape_count += 1;
            shape_index = shape_count % shapes.len();
        }

        Ok(self.tetris.get_stack_height().to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(3068.to_string()),
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
