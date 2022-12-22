use anyhow::Result;

use crate::puzzle::Puzzle;
use crate::utils::board::*;

#[allow(unused_imports)]
use crate::utils::utils::*;

#[allow(unused_imports)]
use std::collections::VecDeque;

pub struct Day22 {
    board: Board<char>,
    commands: Vec<Command>,
}

#[derive(Debug, Clone, Copy)]
enum Command {
    Move(u32),
    TurnClockwise,        // R
    TurnCounterClockwise, // L
}

fn turn_me(current_direction: Direction, how_to_turn: Command) -> Direction {
    log::debug!("Turn {how_to_turn:?}");
    let directions = vec![
        Direction::Up,
        Direction::Right,
        Direction::Down,
        Direction::Left,
    ];
    let offset = match how_to_turn {
        Command::TurnClockwise => 1,
        Command::TurnCounterClockwise => -1,
        _ => panic!("Unexpected turn"),
    };
    let current_index = directions
        .iter()
        .position(|&x| x == current_direction)
        .unwrap();
    let new_index = (current_index as i32 + offset).rem_euclid(directions.len() as i32) as usize;
    directions[new_index]
}

fn navigate(day: &mut Day22)->Direction {
    let mut direction = Direction::Right;

    for command in &day.commands {
        match command {
            Command::Move(distance) => {
                log::debug!("Move {distance}");
                for _ in 0..*distance {
                    day.board.step(direction);
                }
            }
            _ => direction = turn_me(direction, command.clone()),
        }
        // day.board.print_board_with_players_pretty();
        // let mut line = String::new();
        // let _ = std::io::stdin().read_line(&mut line).unwrap();
    }

    day.board.print_board_with_players_pretty();
    direction
}

fn direction_value(direction: Direction) -> i32 {
    match direction {
        Direction::Right => 0,
        Direction::Down => 1,
        Direction::Left => 2,
        Direction::Up => 3,
        _ => panic!("Unexpected direction"),
    }
}

impl Puzzle for Day22 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut day = Day22 {
            board: Board::new(),
            commands: vec![],
        };

        // Get the commands
        let mut lines: Vec<&str> = input.lines().collect();
        let commands = lines.pop().unwrap();
        lines.pop(); // Drop empty line

        // Find the width
        let mut width = 0;
        for line in &lines {
            width = std::cmp::max(line.chars().count(), width);
        }
        log::debug!("Width is {width}");

        // Add map lines to board
        for line in &lines {
            let mut chars: Vec<char> = line.chars().collect();
            let need_more = width - chars.len();
            if need_more > 0 {
                let more = vec![' '; need_more];
                chars.extend(more);
            }
            day.board.push_row(chars);
        }

        // Add player at left most top row
        let mut start_x = 0;
        for x in 0..width as i32 {
            if day.board.get_at(BoardPoint { x, y: 0 }) == '.' {
                start_x = x;
                break;
            }
        }
        day.board.add_player(BoardPoint { x: start_x, y: 0 }, '+');

        // Config the board
        day.board.add_wall('#');
        day.board.add_wraparound(' ');
        day.board.set_wraparound_mode();
        day.board.print_board_with_players_pretty();

        // Parse the commands
        let commands_move: Vec<_> = commands.split(|c| c == 'L' || c == 'R').collect();
        let commands_turn: Vec<_> = commands.match_indices(|c| c == 'L' || c == 'R').collect();
        log::trace!("Moves {commands_move:?}");
        log::trace!("Turns {commands_turn:?}");
        assert_eq!(commands_move.len() - 1, commands_turn.len()); // One more move than turn
        for (i, m) in commands_move.iter().enumerate() {
            day.commands.push(Command::Move(get_val(m)));
            if i >= commands_turn.len() {
                break;
            }
            let turn = match commands_turn[i] {
                (_, "R") => Command::TurnClockwise,
                (_, "L") => Command::TurnCounterClockwise,
                _ => panic!("Unuexpected turn char"),
            };
            day.commands.push(turn);
        }
        log::trace!("Commands {:?}", day.commands);

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        let direction = navigate(self);
        let point = self.board.get_player_location(0);
        log::debug!("Ended at {:?}", point);
        let answer = (point.y + 1) * 1000 + (point.x + 1) * 4 + direction_value(direction);
        Ok(answer.to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(6032.to_string()),
            false => None,
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
