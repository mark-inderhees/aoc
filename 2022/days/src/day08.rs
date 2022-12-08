use anyhow::Result;
use strum::IntoEnumIterator; // TODO move this into board.rs

use crate::puzzle::Puzzle;
use crate::board::*;

pub struct Day08 {
    board: Board<u32>,
    visible: Board<char>,
    score: Board<u32>,
}

impl Puzzle for Day08 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut day = Day08 {
            board: Board::new(),
            visible: Board::new(),
            score: Board::new(),
        };

        for line in input.lines() {
            let row: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
            let len = row.len();
            day.board.grid.push_row(row);
            day.visible.grid.push_row(vec!['.'; len]);
            day.score.grid.push_row(vec![0; len]);
        }

        log::debug!("Input Grid: {:#?}", day.board);

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        let x_max: i32 = self.board.grid.size().0.try_into().unwrap();
        let y_max: i32 = self.board.grid.size().1.try_into().unwrap();
        let mut count = x_max * 2 + y_max * 2 - 4;
        for y in 1..(y_max - 1) {
            for x in 1..(x_max - 1) {
                let mut scores = vec![];
                for direction in Direction::iter() {
                    scores.push(0);
                    self.board.set_location(x, y);
                    let height = self.board.get_current_value().clone();
                    let mut heights = vec![];
                    while let Some(height2) = self.board.step(direction) {
                        heights.push(height2.clone());

                        let s = scores.pop().unwrap().clone();
                        scores.push(s + 1);
                        if *height2 >= height {
                            // break; // part 2
                        }
                    }
                    let height_max = heights.iter().max().unwrap();
                    let visible = height > *height_max;
                    // log::debug!("At {x},{y} going {direction:?}: {height} vs {height_max} = {visible}, {heights:?}");
                    if visible {
                        count += 1;
                        let x_: usize = x.try_into().unwrap();
                        let y_: usize = y.try_into().unwrap();
                        self.visible.grid[y_][x_] = 'v';
                        break;
                    }
                }
                let mega_score = scores.iter().fold(1, |a, x| a * x);
                log::debug!("At {x},{y} score {scores:?} --> {mega_score}");
                let x_: usize = x.try_into().unwrap();
                let y_: usize = y.try_into().unwrap();
                self.score.grid[y_][x_] = mega_score;
            }
        }

        log::debug!("Input Grid: {:#?}", self.board.grid);
        log::debug!("{:#?}", self.visible.grid);
        log::info!("{count}");

        Ok(count.to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(21.to_string()),
            false => Some(1698.to_string()),
        }
    }

    fn solve_part2(&mut self) -> Result<String> {
        let x_max: i32 = self.board.grid.size().0.try_into().unwrap();
        let y_max: i32 = self.board.grid.size().1.try_into().unwrap();
        let mut count = x_max * 2 + y_max * 2 - 4;
        for y in 1..(y_max - 1) {
            for x in 1..(x_max - 1) {
                let mut scores = vec![];
                for direction in Direction::iter() {
                    scores.push(0);
                    self.board.set_location(x, y);
                    let height = self.board.get_current_value().clone();
                    let mut heights = vec![];
                    while let Some(height2) = self.board.step(direction) {
                        heights.push(height2.clone());

                        let s = scores.pop().unwrap().clone();
                        scores.push(s + 1);
                        if *height2 >= height {
                            break; // part 2
                        }
                    }
                    let height_max = heights.iter().max().unwrap();
                    let visible = height > *height_max;
                    // log::debug!("At {x},{y} going {direction:?}: {height} vs {height_max} = {visible}, {heights:?}");
                    if visible {
                        count += 1;
                        let x_: usize = x.try_into().unwrap();
                        let y_: usize = y.try_into().unwrap();
                        self.visible.grid[y_][x_] = 'v';
                        // break; part 2
                    }
                }
                let mega_score = scores.iter().fold(1, |a, x| a * x);
                log::debug!("At {x},{y} score {scores:?} --> {mega_score}");
                let x_: usize = x.try_into().unwrap();
                let y_: usize = y.try_into().unwrap();
                self.score.grid[y_][x_] = mega_score;
            }
        }

        log::debug!("Input Grid: {:#?}", self.board.grid);
        // log::debug!("{:#?}", self.visible.grid);
        log::debug!("{:#?}", self.score.grid);
        log::info!("{count}");
        let part2 = self.score.grid.iter().max().unwrap();
        log::info!("{}", part2);

        Ok(part2.to_string())
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(8.to_string()),
            false => Some(672280.to_string()),
        }
    }
}
