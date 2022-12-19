use std::collections::HashMap;

use anyhow::Result;

use crate::puzzle::Puzzle;
use crate::utils::board::*;
use crate::utils::tetris::*;

#[allow(unused_imports)]
use crate::utils::utils::*;

pub struct Day17 {
    commands: Vec<Direction>,
    tetris: Tetris,
    round_info: Vec<RoundInfo>,
}

#[derive(Clone, Copy, Debug)]
#[allow(dead_code)]
struct RoundInfo {
    shape_count: usize,
    shape_index: usize,
    command_index: usize,
    height: u32,
}

struct RepeatDetection {
    shape_count1: usize,
    shape_count2: usize,
}

fn play_game(day: &mut Day17) -> RepeatDetection {
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
    log::info!("Shapes {}. Commands {}.", shapes.len(), day.commands.len());

    let mut round_map: HashMap<String, RoundInfo> = HashMap::new();
    let mut repeat = RepeatDetection {
        shape_count1: 0,
        shape_count2: 0,
    };

    while shape_count < 2022 {
        let round_info = RoundInfo {
            shape_count,
            shape_index,
            command_index,
            height: day.tetris.get_stack_height(),
        };
        day.round_info.push(round_info);

        // Repeat detection, find where the grid starts to repeat
        if repeat.shape_count2 == 0 {
            let current_height = day.tetris.get_stack_height();
            if current_height > 100 {
                let key = day.tetris.get_rows_as_string(100);
                if round_map.contains_key(&key) {
                    log::info!("Match at {round_info:?}. Orig was {:?}", round_map[&key]);
                    repeat.shape_count1 = round_map[&key].shape_count;
                    repeat.shape_count2 = shape_count;
                } else {
                    round_map.insert(key, round_info);
                }
            }
        }

        if day.tetris.is_top_line_full() {
            log::info!("Top line full at shape #{shape_count}");
        }

        let shape = shapes[shape_index];
        let shape_id = day.tetris.add_shape(shape);

        let shape_count1 = shape_count;
        let shape_index1 = shape_index;
        let command_index1 = command_index;

        loop {
            let command = day.commands[command_index];
            command_index = (command_index + 1) % day.commands.len();
            if shape_count == 21 {
                // day.tetris.print();
            }
            day.tetris.move_shape(shape_id, command);
            if shape_count == 21 {
                // day.tetris.print();
            }
            if !day.tetris.move_shape(shape_id, Direction::Down) {
                break;
            }
        }

        log::debug!(
            "Round {} Start P: {} Start W: {} Tower height: {}",
            shape_count1,
            shape_index1,
            command_index1,
            day.tetris.get_stack_height()
        );

        shape_count += 1;
        shape_index = shape_count % shapes.len();
    }

    repeat
}

impl Puzzle for Day17 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut day = Day17 {
            commands: vec![],
            tetris: Tetris::new(),
            round_info: vec![],
        };

        let input_to_use = input.trim();
        for char in input_to_use.to_string().chars() {
            match char {
                '<' => day.commands.push(Direction::Left),
                '>' => day.commands.push(Direction::Right),
                _ => panic!("Unexpected char"),
            }
        }

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        play_game(self);
        Ok(self.tetris.get_stack_height().to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(3068.to_string()),
            false => Some(3177.to_string()),
        }
    }

    fn solve_part2(&mut self) -> Result<String> {
        let repeat = play_game(self);

        let height_initial = self.round_info[repeat.shape_count1].height as u64;
        let shape_count_diff = (repeat.shape_count2 - repeat.shape_count1) as u64;
        let shape_count_in_repeat = 1000000000000u64 - repeat.shape_count1 as u64;
        let shape_count_extra = (shape_count_in_repeat % shape_count_diff) as usize;
        let height_diff = (self.round_info[repeat.shape_count2].height
            - self.round_info[repeat.shape_count1].height) as u64;
        let height_extra = (self.round_info[repeat.shape_count1 + shape_count_extra].height
            - self.round_info[repeat.shape_count1].height) as u64;
        let answer = height_initial
            + (shape_count_in_repeat) / shape_count_diff * height_diff
            + height_extra;

        Ok(answer.to_string())
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(1514285714288u64.to_string()),
            false => Some(1565517241382u64.to_string()),
        }
    }
}
