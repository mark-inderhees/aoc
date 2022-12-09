use anyhow::Result;

use crate::puzzle::Puzzle;
use crate::utils::board::*;

pub struct Day09 {
    board: Board<char>,
    visited: Board<char>,
    commands: Vec<(Direction, u32)>,
}

impl Puzzle for Day09 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut day = Day09 {
            board: Board::new(),
            visited: Board::new(),
            commands: vec![],
        };

        for line in input.lines() {
            let l: Vec<&str> = line.split(" ").collect();
            let dir_char = l[0];
            let step_count = l[1].parse::<u32>().unwrap();
            let direction = match dir_char {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => panic!("Did not understand dir char"),
            };
            day.commands.push((direction, step_count));
        }

        let debug = false;
        let dim = match debug {
            true => 6,
            false => 1000,
        };
        for d in 0..dim {
            day.board.push_row(vec!['.'; dim]);
            day.visited.push_row(vec!['.'; dim]);
        }

        let initx = match debug {
            true => 0,
            false => dim as i32 / 2,
        };
        let inity = match debug {
            true => 5,
            false => dim as i32 / 2,
        };
        let player_count = 10 as usize;
        for player in 0..player_count {
            day.board
                .add_player(initx, inity, char::from_digit(player as u32, 10).unwrap());
        }
        let start = day.board.add_player(initx, inity, 'S');
        day.visited.set_at(initx, inity, '#');
        day.board.print_board_with_players();

        for (direction, step_count) in &day.commands {
            log::debug!("== {direction:#?} {step_count} ==");
            for _ in 0..*step_count {
                day.board.step(*direction);
                // day.board.print_board_with_players();
                for player in 1..player_count {
                    let prev_player = player - 1;
                    if !day.board.is_nearby(prev_player, player) {
                        // Need to move player 2
                        let way_to_go = day.board.where_to_move(player, prev_player);
                        day.board.step_player(player, way_to_go);
                        // day.board.print_board_with_players();
                        let p2_loc = day.board.get_player_location(player);
                        if player == player_count - 1 {
                            day.visited.set_at(p2_loc.0, p2_loc.1, '#');
                        }
                    }
                }
                day.board.print_board_with_players();
            }
        }

        log::debug!("{:#?}", day.visited.grid());

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        let count = self.visited.grid().iter().fold(0, |a, x| match *x {
            '#' => a + 1,
            _ => a,
        });
        Ok(count.to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(13.to_string()),
            false => Some(6337.to_string()),
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
