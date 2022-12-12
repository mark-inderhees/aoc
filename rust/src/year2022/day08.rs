use anyhow::Result;

use crate::utils::board::*;
use crate::puzzle::Puzzle;

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
            day.board.push_row(row);
            day.visible.push_row(vec!['.'; len]);
            day.score.push_row(vec![0; len]);
        }

        day.board.add_player(0, 0, 0);

        log::debug!("Input Grid: {:#?}", day.board);

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        // Find how many trees are visible with respect to viewing from the outside of the board
        let mut visible_trees = self.board.width() * 2 + self.board.height() * 2 - 4;
        for y in 1..(self.board.height() - 1) {
            for x in 1..(self.board.width() - 1) {
                for direction in Direction::straight_iterator() {
                    self.board.set_location(x, y);
                    let tree_height = self.board.get_current_value().clone();
                    let mut tree_heights = vec![];
                    while let Some(tree_height2) = self.board.step(direction) {
                        tree_heights.push(tree_height2.clone());
                    }
                    let tree_height_max = tree_heights.iter().max().unwrap().clone();
                    let visible = tree_height > tree_height_max;
                    log::debug!("At {x},{y} going {direction:?}: {tree_height} vs {tree_height_max} = {visible}, {tree_heights:?}");
                    if visible {
                        visible_trees += 1;
                        self.visible.set_at(x, y, 'v');
                        break;
                    }
                }
            }
        }

        log::debug!("Input Grid: {:#?}", self.board.grid());
        log::debug!("{:#?}", self.visible.grid());
        log::info!("{visible_trees}");

        Ok(visible_trees.to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(21.to_string()),
            false => Some(1698.to_string()),
        }
    }

    fn solve_part2(&mut self) -> Result<String> {
        // Find how many trees we can see from within the forest
        for y in 1..(self.board.height() - 1) {
            for x in 1..(self.board.width() - 1) {
                let mut count_trees = vec![];
                for direction in Direction::straight_iterator() {
                    count_trees.push(0);
                    self.board.set_location(x, y);
                    let tree_height = self.board.get_current_value().clone();
                    while let Some(tree_height2) = self.board.step(direction) {
                        let s = count_trees.pop().unwrap().clone();
                        count_trees.push(s + 1);
                        if tree_height2 >= tree_height {
                            break;
                        }
                    }
                }

                // Calculate score as multiple of count in each direction
                let mega_score = count_trees.iter().fold(1, |a, x| a * x);
                log::debug!("At {x},{y} score {count_trees:?} --> {mega_score}");
                self.score.set_at(x, y, mega_score);
            }
        }

        log::debug!("Input Grid: {:#?}", self.board.grid());
        log::debug!("{:#?}", self.score.grid());
        let max = self.score.grid().iter().max().unwrap();
        log::info!("{}", max);

        Ok(max.to_string())
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(8.to_string()),
            false => Some(672280.to_string()),
        }
    }
}
