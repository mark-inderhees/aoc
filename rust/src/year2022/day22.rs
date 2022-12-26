// 2022 Day 22
// https://adventofcode.com/2022/day/22
// --- Day 22: Monkey Map ---
// The monkeys want you to follow a weird map that transports you to vairous spots.
// But it's actually a 3D cube!

use anyhow::Result;
use core::panic;
use std::vec;

use crate::puzzle::Puzzle;
use crate::utils::board::*;
use crate::utils::board3d::*;
use crate::utils::utils::*;

pub struct Day22 {
    board: Board<char>,     // part 1
    board3d: Board3D<char>, // part 2
    commands: Vec<Command>,
    board_offsets: Vec<BoardPoint>,
}

#[derive(Debug, Clone, Copy)]
enum Command {
    Move(u32),
    TurnClockwise,        // R
    TurnCounterClockwise, // L
}

// Find the new direction based on the current direction and to turn 90 or -90 degrees.
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
    let new_direction = directions[new_index];
    log::debug!("Now facing {:?}", new_direction);
    new_direction
}

// Move through the 2d board. When you get to an edge, you zap back to the other side.
fn navigate(day: &mut Day22) -> Direction {
    let mut direction = Direction::Right;

    for command in &day.commands {
        match command {
            Command::Move(distance) => {
                log::debug!("Move {distance}");
                for _ in 0..*distance {
                    if day.board.step(direction).is_none() {
                        break;
                    }
                }
            }
            _ => direction = turn_me(direction, command.clone()),
        }
    }

    direction
}

// Move throught he 3d cube!
fn navigate3d(day: &mut Day22) -> Direction {
    let mut direction = Direction::Right;

    let player_id = 0;
    for command in &day.commands {
        match command {
            Command::Move(distance) => {
                log::debug!("Move {distance}");
                for _ in 0..*distance {
                    if day.board3d.step_player(player_id, direction).is_none() {
                        break;
                    }
                }
            }
            _ => direction = turn_me(direction, command.clone()),
        }
    }

    direction
}

// A direction has a score value to get answer
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
            board3d: Board3D::new(),
            commands: vec![],
            board_offsets: vec![],
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

        // Find 3d width
        let test = width < 20;
        let width3d = match test {
            true => 4,
            false => 50,
        };
        let height3d = width3d;

        // Add map lines to board
        let mut board_id_offset = 0;
        let mut y_offset = 0;
        for (i, line) in lines.iter().enumerate() {
            let mut chars: Vec<char> = line.chars().collect();
            let need_more = width - chars.len();
            if need_more > 0 {
                let more = vec![' '; need_more];
                chars.extend(more);
            }
            day.board.push_row(chars.clone());

            // Add lines to 3d board
            let lines3d: Vec<&[char]> = chars.chunks(width3d).collect();
            let mut board_id = 0;
            let mut x_offset = 0;
            for line3d in lines3d {
                if line3d.contains(&' ') {
                    x_offset += width3d as i32;
                    continue;
                }
                day.board3d
                    .push_row(board_id + board_id_offset, line3d.to_vec());
                if board_id + board_id_offset == day.board_offsets.len() {
                    day.board_offsets.push(BoardPoint {
                        x: x_offset,
                        y: y_offset,
                    });
                }
                board_id += 1;
                x_offset += width3d as i32;
            }

            if (i + 1) % width3d == 0 {
                board_id_offset += board_id;
                y_offset += height3d as i32;
            }
        }

        log::debug!("board offsets {:?}", day.board_offsets);

        // Add player at left most top row
        let mut start_x = 0;
        for x in 0..width as i32 {
            if day.board.value_at(BoardPoint { x, y: 0 }) == '.' {
                start_x = x;
                break;
            }
        }
        day.board.add_player(BoardPoint { x: start_x, y: 0 }, '+');
        day.board3d.add_player(0, BoardPoint { x: 0, y: 0 }, '+');

        // Config the board
        day.board.add_wraparound(' '); // This does the magic moves for part 1
        day.board.set_wraparound_mode();

        // Parse the commands
        let commands_move: Vec<_> = commands.split(|c| c == 'L' || c == 'R').collect();
        let commands_turn: Vec<_> = commands.match_indices(|c| c == 'L' || c == 'R').collect();
        log::trace!("Moves {commands_move:?}");
        log::trace!("Turns {commands_turn:?}");
        assert_eq!(commands_move.len() - 1, commands_turn.len()); // One more move than turn
        for (i, m) in commands_move.iter().enumerate() {
            day.commands.push(Command::Move(find_val(m)));
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

        // Build 3d board, need to connect the edges
        // Each folding for input looks different, I'm hard coding how test and my real work here
        if test {
            // Fold is like
            // __0_
            // 123_
            // __45

            // 1 top -> 0 top, down (inverse)
            // 1 left -> 5 bottom, up (inverse)
            // 1 bottom -> 4 bottom, up (inverse)
            // 2 top -> 0 left, right
            // 2 bottom -> 4 left, right (inverse)
            // 3 right -> 5 top, down (inverse)
            // 4 left -> 2 bottom, up (inverse)
            // 4 botom -> 1 bottom, up (inverse)
            // 5 top -> 3 right, left (inverse)
            // 5 right -> 0 right, left (inverse)
            // 5 bottom -> 1 left, right (inverse)
            day.board3d
                .set_edge(EdgeConnection::new(0, Edge::Top, 1, Edge::Top, true));
            day.board3d
                .set_edge(EdgeConnection::new(0, Edge::Right, 5, Edge::Right, true));
            day.board3d
                .set_edge(EdgeConnection::new(0, Edge::Bottom, 3, Edge::Top, false));
            day.board3d
                .set_edge(EdgeConnection::new(0, Edge::Left, 2, Edge::Top, false));
            day.board3d
                .set_edge(EdgeConnection::new(1, Edge::Right, 2, Edge::Left, false));
            day.board3d
                .set_edge(EdgeConnection::new(1, Edge::Bottom, 4, Edge::Bottom, true));
            day.board3d
                .set_edge(EdgeConnection::new(1, Edge::Left, 5, Edge::Bottom, true));
            day.board3d
                .set_edge(EdgeConnection::new(2, Edge::Right, 3, Edge::Left, false));
            day.board3d
                .set_edge(EdgeConnection::new(2, Edge::Bottom, 4, Edge::Left, true));
            day.board3d
                .set_edge(EdgeConnection::new(3, Edge::Right, 5, Edge::Top, true));
            day.board3d
                .set_edge(EdgeConnection::new(3, Edge::Bottom, 4, Edge::Top, false));
            day.board3d
                .set_edge(EdgeConnection::new(4, Edge::Right, 5, Edge::Left, false));
        } else {
            // Fold is like
            // _01
            // _2_
            // 34_
            // 5__
            // 0 top -> 5 left
            // 0 left -> 3 left (inverse)
            // 1 top -> 5 bottom
            // 1 right -> 4 right (inverse)
            // 1 bottom -> 2 right
            // 2 left -> 3 top
            // 2 right -> 1 bottom
            // 3 top -> 2 left
            // 3 left -> 0 left (inverse)
            // 4 right -> 1 right (inverse)
            // 4 bottom -> 5 right
            // 5 left -> 0 top
            // 5 right -> 4 bottom
            // 5 bottom -> 1 top
            day.board3d
                .set_edge(EdgeConnection::new(0, Edge::Top, 5, Edge::Left, false));
            day.board3d
                .set_edge(EdgeConnection::new(0, Edge::Right, 1, Edge::Left, false));
            day.board3d
                .set_edge(EdgeConnection::new(0, Edge::Bottom, 2, Edge::Top, false));
            day.board3d
                .set_edge(EdgeConnection::new(0, Edge::Left, 3, Edge::Left, true));
            day.board3d
                .set_edge(EdgeConnection::new(1, Edge::Top, 5, Edge::Bottom, false));
            day.board3d
                .set_edge(EdgeConnection::new(1, Edge::Right, 4, Edge::Right, true));
            day.board3d
                .set_edge(EdgeConnection::new(1, Edge::Bottom, 2, Edge::Right, false));
            day.board3d
                .set_edge(EdgeConnection::new(2, Edge::Bottom, 4, Edge::Top, false));
            day.board3d
                .set_edge(EdgeConnection::new(2, Edge::Left, 3, Edge::Top, false));
            day.board3d
                .set_edge(EdgeConnection::new(3, Edge::Right, 4, Edge::Left, false));
            day.board3d
                .set_edge(EdgeConnection::new(3, Edge::Bottom, 5, Edge::Top, false));
            day.board3d
                .set_edge(EdgeConnection::new(4, Edge::Bottom, 5, Edge::Right, false));
        }

        day.board.add_wall('#');
        day.board3d.add_wall('#');

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        let direction = navigate(self);
        let point = self.board.player_location(0);
        log::debug!("Ended at {:?}", point);
        let answer = (point.y + 1) * 1000 + (point.x + 1) * 4 + direction_value(direction);
        Ok(answer.to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(6032.to_string()),
            false => Some(26558.to_string()),
        }
    }

    fn solve_part2(&mut self) -> Result<String> {
        let direction = navigate3d(self);
        // Need to convert from 2d based direction to 3d based direction
        let real_direction = self.board3d.get_player_direction(0, direction);
        let (board_id, point) = self.board3d.player_location(0);
        // Also need to convert from 3d point to 2d point
        let real_point = BoardPoint {
            x: point.x + self.board_offsets[board_id].x,
            y: point.y + self.board_offsets[board_id].y,
        };
        log::debug!(
            "Ended at {:?} on board {} ({:?} + {:?}). Direction {:?}",
            real_point,
            board_id,
            point,
            self.board_offsets[board_id],
            real_direction,
        );
        let answer =
            (real_point.y + 1) * 1000 + (real_point.x + 1) * 4 + direction_value(real_direction);
        Ok(answer.to_string())
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(5031.to_string()),
            false => Some(110400.to_string()),
        }
    }
}
