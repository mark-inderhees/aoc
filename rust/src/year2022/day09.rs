// 2022 Day 9
// https://adventofcode.com/2022/day/9
// --- Day 9: Rope Bridge ---
// Snake! Walk a maze, having different parts of the snake follow the head.

use anyhow::Result;

use crate::puzzle::Puzzle;
use crate::utils::board::*;

pub struct Day09 {
    board: Board<char>,
    visited: Board<char>,
    commands: Vec<(Direction, u32)>,
}

/// Move the snake. Return the number of unique squares visited by the tail.
fn move_it(day: &mut Day09, player_count: usize) -> u32 {
    // Init size based on test or real input
    let debug = day.commands.len() < 10;
    let dim = match debug {
        true => 6,
        false => 1000,
    };

    // Initial location in the middle of the board so we do not overflow
    let init = match debug {
        true => BoardPoint { x: 0, y: 5 },
        false => BoardPoint {
            x: dim as i32 / 2,
            y: dim as i32 / 2,
        },
    };

    // Build the game board and history board.
    for _ in 0..dim {
        day.board.push_row(vec!['.'; dim]);
        day.visited.push_row(vec!['.'; dim]);
    }

    // Add all the other players
    for player in 0..player_count {
        day.board
            .add_player(init, char::from_digit(player as u32, 10).unwrap());
    }

    // Add main player
    day.board.add_player(init, 'S');
    day.visited.set_at(init, '#');
    day.board.print_board_with_players();

    // Walk all the commands
    for (direction, step_count) in &day.commands {
        log::debug!("== {direction:#?} {step_count} ==");
        // Run the direction command the requested number of times
        for _ in 0..*step_count {
            day.board.step_player(0, *direction); // Step first player

            // Move the tail players to follow the previous player based on that players location
            for player in 1..player_count {
                let prev_player = player - 1;
                if !day.board.is_nearby(prev_player, player) {
                    // Need to move player 2
                    let way_to_go = day.board.where_to_move(player, prev_player);
                    day.board.step_player(player, way_to_go);
                    let p2_loc = day.board.player_location(player);

                    // If this is the tail and it moved, mark this as a new location visited
                    if player == player_count - 1 {
                        day.visited.set_at(p2_loc, '#');
                    }
                }
            }
            day.board.print_board_with_players();
        }
    }

    log::debug!("{:#?}", day.visited.grid());

    // Return count of places the tail visited
    day.visited.grid().iter().fold(0, |a, x| match *x {
        '#' => a + 1,
        _ => a,
    })
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

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        let player_count = 2;
        let count = move_it(self, player_count);
        Ok(count.to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(13.to_string()),
            false => Some(6337.to_string()),
        }
    }

    fn solve_part2(&mut self) -> Result<String> {
        let player_count = 10;
        let count = move_it(self, player_count);
        Ok(count.to_string())
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(1.to_string()),
            false => Some(2455.to_string()),
        }
    }
}
