use std::vec;

use anyhow::Result;
use rusttype::Point;

use crate::puzzle::Puzzle;
use crate::utils::board::*;
use crate::utils::utils::*;

pub struct Day12 {
    grid: Board<char>,
    step_count: Board<u32>,
    start_x: i32,
    start_y: i32,
    end_x: i32,
    end_y: i32,
}

fn move_it(day: &mut Day12, location: Point<i32>, count: u32, total_count: &mut Vec<u32>) {
    if count > 600 {
        return;
    }
    if total_count.len() > 0 && count >= *total_count.iter().min().unwrap() {
        return;
    }
    let step_count = day.step_count.grid()[location.y as usize][location.x as usize];
    if count >= step_count {
        return;
    }
    day.step_count.grid[location.y as usize][location.x as usize] = count;
    // Force current location
    day.grid.set_location(location.x, location.y);
    let my_char = day.grid.get_current_value();
    // print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    // day.grid.print_board_with_players();

    let mut directions = day.grid.get_nearby_squares(0);
    // let where_to_go = day.grid.where_to_move_straight(0, 1);
    // if directions.contains(&where_to_go) {
    //     let index = directions.iter().position(|x| *x == where_to_go).unwrap();
    //     directions.remove(index);
    //     let copy = directions.clone();
    //     directions = vec![where_to_go];
    //     directions.extend(copy.iter());
    // }

    // log::debug!(
    //     "[{}] At {}, {} = {}, go {:?}",
    //     count,
    //     location.x,
    //     location.y,
    //     my_char,
    //     directions
    // );

    // Try all new locations
    for direction in directions {
        if total_count.len() > 0 && count + 1 >= *total_count.iter().min().unwrap() {
            return;
        }

        // Try this location
        day.grid.set_location(location.x, location.y);
        day.grid.step(direction).unwrap();
        // log::debug!("Just moved {:?}", direction);
        let new_location = day.grid.get_player_location(0);
        let point = Point {
            x: new_location.0,
            y: new_location.1,
        };
        let near_char = day.grid.get_current_value();

        // See if we are allowed to move here
        // log::debug!("I have never been here");
        let from = my_char as u32;
        let to = near_char as u32;
        if to <= from + 1 {
            // Check if we are done
            if point.x == day.end_x && point.y == day.end_y {
                log::debug!("THIS IS THE END = {}", count);
                total_count.push(count + 1);
            }
            // We can move, so do it!
            // log::debug!(
            //     "Moving from {}, {} = {} to {}, {} = {}",
            //     location.x,
            //     location.y,
            //     my_char,
            //     point.x,
            //     point.y,
            //     near_char
            // );

            move_it(day, point, count + 1, total_count);
        } else {
            // log::debug!(
            //     "Cannot step from {}, {} = {} to {}, {} = {}",
            //     location.x,
            //     location.y,
            //     my_char,
            //     point.x,
            //     point.y,
            //     near_char
            // );
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
            start_x: 0,
            start_y: 0,
            end_x: 0,
            end_y: 0,
        };

        day.grid.add_player(0, 0, 'S');
        day.grid.add_player(0, 0, 'E');

        for (i, mut line) in input.lines().enumerate() {
            if char_in_string(&'S', &line.to_string()) {
                day.start_x = line.find("S").unwrap() as i32;
                day.start_y = i as i32;
                day.grid.set_player_location(0, day.start_x, day.start_y);
            }
            if char_in_string(&'E', &line.to_string()) {
                day.end_x = line.find("E").unwrap() as i32;
                day.end_y = i as i32;
                day.grid.set_player_location(1, day.end_x, day.end_y);
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
        let point = Point { x: me.0, y: me.1 };
        let mut total_count = vec![];
        move_it(self, point, 0, &mut total_count);
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
        for y in 0..self.grid.grid.rows() {
            for x in 0..self.grid.grid.cols() {
                let chr = self.grid.grid[y][x];
                if chr == 'a' {
                    let x_i32 = x as i32;
                    let y_i32 = y as i32;
                    self.grid.set_location(x_i32, y_i32);
                    let point = Point { x: x_i32, y: y_i32 };
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
