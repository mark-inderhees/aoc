use anyhow::Result;

use crate::puzzle::Puzzle;
use crate::utils::board::*;
use crate::utils::utils::*;

pub struct Day12 {
    grid: Board<char>,
    start: BoardPoint,
    end: BoardPoint,
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
            start: BoardPoint { x: 0, y: 0 },
            end: BoardPoint { x: 0, y: 0 },
        };

        day.grid.add_player(BoardPoint { x: 0, y: 0 }, 'S');
        day.grid.add_player(BoardPoint { x: 0, y: 0 }, 'E');

        for (i, mut line) in input.lines().enumerate() {
            if char_in_string(&'S', &line.to_string()) {
                day.start.x = line.find("S").unwrap() as i32;
                day.start.y = i as i32;
                day.grid.set_player_location(0, day.start);
            }
            if char_in_string(&'E', &line.to_string()) {
                day.end.x = line.find("E").unwrap() as i32;
                day.end.y = i as i32;
                day.grid.set_player_location(1, day.end);
            }
            let line2 = &line.replace("S", "a");
            line = line2;
            let line3 = &line.replace("E", "z");
            line = line3;
            day.grid.push_row(line.chars().collect());
        }

        log::debug!("{:#?}", day.grid);

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        let me = self.grid.get_player_location(0);
        let mut total_count = vec![];
        self.grid.find_shortest_path(me, 0, &mut total_count, valid_move, 1);
        total_count.sort();
        println!("{:?}", total_count);
        let answer = total_count[0];
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
                    self.grid.set_location(point);
                    let mut total_count = vec![];
                    self.grid.find_shortest_path(point, 0, &mut total_count, valid_move, 1);
                    total_count.sort();
                    if total_count.len() > 0 {
                        answers.push(total_count[0]);
                    }
                }
            }
        }
        answers.sort();
        println!("{:?}", answers);
        let answer = answers[0];
        Ok(answer.to_string())
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(29.to_string()),
            false => Some(522.to_string()),
        }
    }
}
