use anyhow::Result;

use crate::puzzle::Puzzle;
use crate::utils::board::*;
use crate::utils::tetris::*;

#[allow(unused_imports)]
use crate::utils::utils::*;

pub struct Day17 {
    commands: Vec<Direction>,
    tetris: Tetris,
}

#[derive(Clone, Copy)]
struct RoundInfo {
    shape_index: usize,
    command_index: usize,
    height: u32,
}

fn play_game(day: &mut Day17) -> Vec<RoundInfo> {
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
    log::info!("Shapes {}. Commands {}.", shapes.len(), day.commands.len());

    let mut round_info = vec![];

    while shape_count < total {
        round_info.push(RoundInfo {
            shape_index,
            command_index,
            height: day.tetris.get_stack_height(),
        });

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

    round_info
}

impl Puzzle for Day17 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut day = Day17 {
            commands: vec![],
            tetris: Tetris::new(),
        };

        let input_to_use = input.trim();

        // Try to find pattern in input, test data is >>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>
        let len = input_to_use.to_string().chars().count();
        for skip in 0..len / 2 {
            for width in 1..len / 2 {
                let sub_string = &input_to_use.to_string()[skip..skip + width];
                let mut good = false;
                for start in (skip+width..len).step_by(width) {
                    if start + width >= len {
                        break;
                    }
                    good = true;
                    let sub_string2 = &input_to_use.to_string()[start..start + width];
                    if sub_string != sub_string2 {
                        // This skip and width pair are not good
                        good = false;
                        break;
                    }
                }
                if good {
                    log::info!("Found an input pattern at skip {skip} width {width}. Pattern {sub_string}");
                    break;
                }
            }
        }

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
        let round_info = play_game(self);
        let mut skip_to_use = 0;
        let mut step_to_use = 0;
        let mut diff_to_use = 0;

        let mut done = false;
        for skip in 0..1000 {
            for step in 1..1000 {
                let mut previous = RoundInfo {
                    shape_index: 0,
                    command_index: 0,
                    height: 0,
                };
                let mut diff = 0;
                let mut good = false;
                for repeat in round_info.iter().skip(skip).step_by(step) {
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

        let mut part2 = 0;
        if step_to_use != 0 {
            part2 = (1000000000000u64 - skip_to_use) / step_to_use * diff_to_use
                + round_info[skip_to_use as usize].height as u64;
        }

        Ok(part2.to_string())
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(1514285714288u64.to_string()),
            false => None,
        }
    }
}
