use anyhow::Result;

use crate::puzzle::Puzzle;
use crate::utils::board::*;
use crate::utils::utils::*;

pub struct Day12 {
    grid: Board<char>,
    step_count: Board<u32>,
    start: BoardPoint,
    end: BoardPoint,
}

fn move_it(day: &mut Day12, location: BoardPoint, count: u32, total_count: &mut Vec<u32>) {
    if count > 600 {
        return;
    }

    // Check if we've ever been here at a more optimized path
    let step_count = day.step_count.grid()[location.y as usize][location.x as usize];
    if count >= step_count {
        return;
    }
    day.step_count.set_at(location, count);

    // Force current location
    day.grid.set_location(location);
    let my_char = day.grid.get_current_value();

    // Try all new locations
    let directions = day.grid.get_nearby_squares(0);
    for direction in directions {
        // Try this location
        day.grid.set_location(location);
        day.grid.step(direction).unwrap();
        let new_location = day.grid.get_player_location(0);
        let near_char = day.grid.get_current_value();

        // See if we are allowed to move here
        let from = my_char as u32;
        let to = near_char as u32;
        if to <= from + 1 {
            // Check if we are done
            if new_location.x == day.end.x && new_location.y == day.end.y {
                log::debug!("THIS IS THE END = {}", count);
                total_count.push(count + 1);
                return;
            }
            // We can move, so do it!
            move_it(day, new_location, count + 1, total_count);
        }
    }
}

impl Puzzle for Day12 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut day = Day12 {
            grid: Board::new(),
            step_count: Board::new(),
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
            let empty = vec![5000; day.grid.width() as usize];
            day.step_count.push_row(empty);
        }

        log::debug!("{:#?}", day.grid);

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        let me = self.grid.get_player_location(0);
        let mut total_count = vec![];
        move_it(self, me, 0, &mut total_count);
        total_count.sort();
        self.step_count.print_board_with_players();
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
                    move_it(self, point, 0, &mut total_count);
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
