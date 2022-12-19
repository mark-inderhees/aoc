use anyhow::Result;

use crate::puzzle::Puzzle;
use crate::utils::board::*;
use crate::utils::utils::*;

pub struct Day12 {
    grid: Board<char>,
    start_player: PlayerId,
    end_player: PlayerId,
}

fn valid_move(from: char, to: char) -> bool {
    let from = from as u32;
    let to = to as u32;
    to <= from + 1
}

impl Puzzle for Day12 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut day = Day12 {
            grid: Board::new(),
            start_player: INVALID_PLAYER,
            end_player: INVALID_PLAYER,
        };

        for (i, mut line) in input.lines().enumerate() {
            let line_orig = line.clone();
            let line2 = &line.replace("S", "a");
            line = line2;
            let line3 = &line.replace("E", "z");
            line = line3;
            day.grid.push_row(line.chars().collect());
            if char_in_string(&'S', &line_orig.to_string()) {
                let start = BoardPoint {
                    x: line_orig.find("S").unwrap() as i32,
                    y: i as i32,
                };
                day.start_player = day.grid.add_player(start, 'S');
            }
            if char_in_string(&'E', &line_orig.to_string()) {
                let end = BoardPoint {
                    x: line_orig.find("E").unwrap() as i32,
                    y: i as i32,
                };
                day.end_player = day.grid.add_player(end, 'E');
            }
        }

        log::debug!("{:#?}", day.grid);

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        let answer = self
            .grid
            .find_shortest_path(self.start_player, self.end_player, valid_move);
        Ok(answer.to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(31.to_string()),
            false => Some(528.to_string()),
        }
    }

    fn solve_part2(&mut self) -> Result<String> {
        let mut answers = vec![];
        for y in 0..self.grid.grid().rows() {
            for x in 0..self.grid.grid().cols() {
                let chr = self.grid.grid()[y][x];
                if chr == 'a' {
                    let point = BoardPoint {
                        x: x as i32,
                        y: y as i32,
                    };
                    let new_player = self.grid.add_player(point, '?');
                    answers.push(self.grid.find_shortest_path(
                        new_player,
                        self.end_player,
                        valid_move,
                    ));
                }
            }
        }
        answers.sort();
        Ok(answers[0].to_string())
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(29.to_string()),
            false => Some(522.to_string()),
        }
    }
}
