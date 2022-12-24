use anyhow::Result;
use core::panic;
use std::cmp::Eq;
use std::collections::HashMap;
use std::ops::Range;
use std::vec;

use crate::puzzle::Puzzle;
use crate::utils::board::*;
use crate::utils::board3d::*;

#[allow(unused_imports)]
use crate::utils::utils::*;

#[allow(unused_imports)]
use std::collections::VecDeque;

pub struct Day22 {
    board: Board<char, Day22BoardContext>,
    board3d: Board3D<char>,
    commands: Vec<Command>,
    context: Day22BoardContext,
    board_offsets: Vec<BoardPoint>,
}

#[derive(Debug, Clone, Copy)]
enum Command {
    Move(u32),
    TurnClockwise,        // R
    TurnCounterClockwise, // L
}

#[derive(Debug, Clone)]
enum Day22Edge {
    Horizontal(HorizontalEdge),
    Vertical(VerticalEdge),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct HorizontalEdge {
    y: i32,
    x_range: Range<i32>,
    direction: Direction,
    inverse: bool,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct VerticalEdge {
    x: i32,
    y_range: Range<i32>,
    direction: Direction,
    inverse: bool,
}

impl HorizontalEdge {
    pub fn get_at(&self, index: usize) -> i32 {
        let vector: Vec<i32> = self.x_range.clone().collect();
        let mut index2 = index;
        if self.inverse {
            index2 = vector.len() - 1 - index;
        }
        vector[index2]
    }
}

impl VerticalEdge {
    pub fn get_at(&self, index: usize) -> i32 {
        let vector: Vec<i32> = self.y_range.clone().collect();
        let mut index2 = index;
        if self.inverse {
            index2 = vector.len() - 1 - index;
        }
        vector[index2]
    }
}

#[derive(Clone, Debug)]
struct Day22BoardContext {
    horizontal_edge: HashMap<HorizontalEdge, Day22Edge>,
    vertical_edge: HashMap<VerticalEdge, Day22Edge>,
    wrapped: bool,
    new_direction: Direction,
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
    let new_direction = directions[new_index];
    log::debug!("Now facing {:?}", new_direction);
    new_direction
}

fn navigate(day: &mut Day22) -> Direction {
    let mut direction = Direction::Right;

    // let mut line = String::new();

    for command in &day.commands {
        match command {
            Command::Move(distance) => {
                log::debug!("Move {distance}");
                for _ in 0..*distance {
                    if day.board.step(direction).is_none() {
                        break;
                    }

                    let mut context = day.board.get_context();
                    if context.wrapped {
                        direction = context.new_direction;
                        context.wrapped = false;
                        day.board.set_context(&context);
                    }

                    // day.board.print_board_with_players_pretty();
                    // let _ = std::io::stdin().read_line(&mut line).unwrap();
                }
            }
            _ => direction = turn_me(direction, command.clone()),
        }
        // day.board.print_board_with_players_pretty();
        // let _ = std::io::stdin().read_line(&mut line).unwrap();
    }

    // day.board.print_board_with_players_pretty();
    direction
}

fn navigate3d(day: &mut Day22) -> Direction {
    let mut direction = Direction::Right;

    // let mut line = String::new();
    let player_id = 0;
    for command in &day.commands {
        match command {
            Command::Move(distance) => {
                log::debug!("Move {distance}");
                for _ in 0..*distance {
                    if day.board3d.step_player(player_id, direction).is_none() {
                        break;
                    }

                    // day.board3d.print_board3d_with_players_pretty();
                    // let _ = std::io::stdin().read_line(&mut line).unwrap();
                }
            }
            _ => direction = turn_me(direction, command.clone()),
        }
        // day.board3d.print_board3d_with_players_pretty();
        // let _ = std::io::stdin().read_line(&mut line).unwrap();
    }

    day.board3d.print_board3d_with_players_pretty();
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

fn custom_wraparound(
    context: &mut Day22BoardContext,
    point: BoardPoint,
    direction: Direction,
) -> BoardPoint {
    context.wrapped = false;
    let mut new_point = BoardPoint { x: 0, y: 0 };
    if direction == Direction::Up || direction == Direction::Down {
        for (horizontal_edge, edge) in &context.horizontal_edge {
            if horizontal_edge.x_range.contains(&point.x) && horizontal_edge.y == point.y {
                log::debug!("Found match on edge {horizontal_edge:?} to edge {edge:?}");
                context.wrapped = true;
                let index = horizontal_edge
                    .clone()
                    .x_range
                    .position(|x| x == point.x)
                    .unwrap();
                match edge {
                    Day22Edge::Horizontal(edge_h) => {
                        new_point = BoardPoint {
                            x: edge_h.get_at(index),
                            y: edge_h.y,
                        };
                        context.new_direction = edge_h.direction;
                    }
                    Day22Edge::Vertical(edge_v) => {
                        new_point = BoardPoint {
                            x: edge_v.x,
                            y: edge_v.get_at(index),
                        };
                        context.new_direction = edge_v.direction;
                    }
                }
                break;
            }
        }
    } else if direction == Direction::Left || direction == Direction::Right {
        for (vertical_edge, edge) in &context.vertical_edge {
            if vertical_edge.y_range.contains(&point.y) && vertical_edge.x == point.x {
                log::debug!("Found match on edge {vertical_edge:?} to edge {edge:?}");
                context.wrapped = true;
                let index = vertical_edge
                    .clone()
                    .y_range
                    .position(|y| y == point.y)
                    .unwrap();
                match edge {
                    Day22Edge::Horizontal(edge_h) => {
                        new_point = BoardPoint {
                            x: edge_h.get_at(index),
                            y: edge_h.y,
                        };
                        context.new_direction = edge_h.direction;
                    }
                    Day22Edge::Vertical(edge_v) => {
                        new_point = BoardPoint {
                            x: edge_v.x,
                            y: edge_v.get_at(index),
                        };
                        context.new_direction = edge_v.direction;
                    }
                }
                break;
            }
        }
    } else {
        panic!("Unsuppored custom wrap direciton");
    }

    if context.wrapped {
        log::debug!(
            "Doing custom wrap around from {:?} to {:?}",
            point,
            new_point
        );
        return new_point;
    }

    panic!("Could not find matching custom wraparound");
}

#[allow(dead_code)]
fn test_test_input(day: &mut Day22) {
    day.board.set_wraparound_custom_mode(custom_wraparound);
    struct TestCase {
        name: String,
        start: BoardPoint,
        direction: Direction,
        expect: BoardPoint,
    }
    let mut tests = vec![];
    let side = 4;
    tests.push(TestCase {
        name: "1 top -> 2 top, down (inverse)".to_string(),
        start: BoardPoint {
            x: side * 2 + 1,
            y: 0,
        },
        direction: Direction::Up,
        expect: BoardPoint { x: 2, y: 5 },
    });
    tests.push(TestCase {
        name: "1 left -> 3 top, down".to_string(),
        start: BoardPoint { x: side * 2, y: 1 },
        direction: Direction::Left,
        expect: BoardPoint { x: 5, y: 5 },
    });
    tests.push(TestCase {
        name: "1 right -> 6 right, left (inverse)".to_string(),
        start: BoardPoint {
            x: side * 3 - 1,
            y: 1,
        },
        direction: Direction::Right,
        expect: BoardPoint { x: 14, y: 10 },
    });
    tests.push(TestCase {
        name: "2 top -> 1 top, down (inverse)".to_string(),
        start: BoardPoint { x: 1, y: side },
        direction: Direction::Up,
        expect: BoardPoint { x: 10, y: 1 },
    });
    tests.push(TestCase {
        name: "2 left -> 6 bottom, up (inverse)".to_string(),
        start: BoardPoint { x: 0, y: side },
        direction: Direction::Left,
        expect: BoardPoint { x: 15, y: 10 },
    });
    tests.push(TestCase {
        name: "2 bottom -> 5 bottom, up (inverse)".to_string(),
        start: BoardPoint {
            x: 3,
            y: side * 2 - 1,
        },
        direction: Direction::Down,
        expect: BoardPoint { x: 8, y: 10 },
    });
    tests.push(TestCase {
        name: "3 top -> 1 left, right".to_string(),
        start: BoardPoint { x: 5, y: side },
        direction: Direction::Up,
        expect: BoardPoint { x: 9, y: 1 },
    });
    tests.push(TestCase {
        name: "3 bottom -> 5 left, right (inverse)".to_string(),
        start: BoardPoint {
            x: 5,
            y: side * 2 - 1,
        },
        direction: Direction::Down,
        expect: BoardPoint { x: 9, y: 10 },
    });
    tests.push(TestCase {
        name: "4 right -> 6 top, down (inverse)".to_string(),
        start: BoardPoint {
            x: side * 3 - 1,
            y: side * 2 - 1,
        },
        direction: Direction::Right,
        expect: BoardPoint { x: 12, y: 9 },
    });
    tests.push(TestCase {
        name: "5 left -> 3 bottom, up (inverse)".to_string(),
        start: BoardPoint {
            x: side * 2,
            y: side * 2 + 1,
        },
        direction: Direction::Left,
        expect: BoardPoint { x: 6, y: 6 },
    });
    tests.push(TestCase {
        name: "5 botom -> 2 bottom, up (inverse)".to_string(),
        start: BoardPoint {
            x: side * 3 - 1,
            y: side * 3 - 1,
        },
        direction: Direction::Down,
        expect: BoardPoint { x: 0, y: 6 },
    });
    tests.push(TestCase {
        name: "6 top -> 4 right, left (inverse)".to_string(),
        start: BoardPoint {
            x: side * 3 + 1,
            y: side * 2,
        },
        direction: Direction::Up,
        expect: BoardPoint { x: 10, y: 6 },
    });
    tests.push(TestCase {
        name: "6 right -> 1 right, left (inverse)".to_string(),
        start: BoardPoint {
            x: side * 4 - 1,
            y: side * 2 + 3,
        },
        direction: Direction::Right,
        expect: BoardPoint { x: 10, y: 0 },
    });
    tests.push(TestCase {
        name: "6 bottom -> 2 left, right (inverse)".to_string(),
        start: BoardPoint {
            x: side * 3,
            y: side * 3 - 1,
        },
        direction: Direction::Down,
        expect: BoardPoint { x: 1, y: 7 },
    });
    let abc: Vec<char> = ('a'..'z').collect();
    for (i, test) in tests.iter().enumerate() {
        log::debug!("Test {}: {}", abc[i], test.name);
        let mut direction = test.direction;
        let player = day.board.add_player(test.start, abc[i]);
        day.board.print_board_with_players_pretty();
        day.board.step_player(player, direction);
        day.board.print_board_with_players_pretty();
        let mut context = day.board.get_context();
        if context.wrapped {
            direction = context.new_direction;
            context.wrapped = false;
        }
        day.board.step_player(player, direction);
        log::debug!("");
        day.board.print_board_with_players_pretty();
        assert_eq!(test.expect, day.board.get_player_location(player));
    }

    panic!("TODO test all done");
}

#[allow(dead_code)]
fn test_real_input(day: &mut Day22) {
    day.board.set_wraparound_custom_mode(custom_wraparound);
    struct TestCase {
        name: String,
        start: BoardPoint,
        direction: Direction,
        expect: BoardPoint,
    }
    let mut tests = vec![];
    let side = 50;
    tests.push(TestCase {
        name: "1 top -> 6 left".to_string(),
        start: BoardPoint { x: side + 20, y: 0 },
        direction: Direction::Up,
        expect: BoardPoint { x: 0, y: 0 },
    });
    tests.push(TestCase {
        name: "1 left -> 4 left (inverse)".to_string(),
        start: BoardPoint { x: side, y: 30 },
        direction: Direction::Left,
        expect: BoardPoint { x: 0, y: 0 },
    });
    tests.push(TestCase {
        name: "2 top -> 6 bottom".to_string(),
        start: BoardPoint { x: side * 2, y: 0 },
        direction: Direction::Up,
        expect: BoardPoint { x: 0, y: 0 },
    });
    tests.push(TestCase {
        name: "2 right -> 5 right (inverse)".to_string(),
        start: BoardPoint {
            x: side * 3 - 1,
            y: 3,
        },
        direction: Direction::Right,
        expect: BoardPoint { x: 0, y: 0 },
    });
    tests.push(TestCase {
        name: "2 bottom -> 3 right".to_string(),
        start: BoardPoint {
            x: side * 3 - 1,
            y: side - 1,
        },
        direction: Direction::Down,
        expect: BoardPoint { x: 0, y: 0 },
    });
    tests.push(TestCase {
        name: "3 left -> 4 top".to_string(),
        start: BoardPoint { x: side, y: side },
        direction: Direction::Left,
        expect: BoardPoint { x: 0, y: 0 },
    });
    tests.push(TestCase {
        name: "3 right -> 2 bottom".to_string(),
        start: BoardPoint {
            x: side * 2 - 1,
            y: side + 2,
        },
        direction: Direction::Right,
        expect: BoardPoint { x: 0, y: 0 },
    });
    tests.push(TestCase {
        name: "4 top -> 3 left".to_string(),
        start: BoardPoint { x: 0, y: side * 2 },
        direction: Direction::Up,
        expect: BoardPoint { x: 0, y: 0 },
    });
    tests.push(TestCase {
        name: "4 left -> 1 left (inverse)".to_string(),
        start: BoardPoint { x: 0, y: side * 2 },
        direction: Direction::Left,
        expect: BoardPoint { x: 0, y: 0 },
    });
    tests.push(TestCase {
        name: "5 right -> 2 right (inverse)".to_string(),
        start: BoardPoint {
            x: 2 * side - 1,
            y: side * 2 + 20,
        },
        direction: Direction::Right,
        expect: BoardPoint { x: 0, y: 0 },
    });
    tests.push(TestCase {
        name: "5 bottom -> 6 right".to_string(),
        start: BoardPoint {
            x: side + 10,
            y: side * 3 - 1,
        },
        direction: Direction::Down,
        expect: BoardPoint { x: 0, y: 0 },
    });
    tests.push(TestCase {
        name: "6 left -> 1 top".to_string(),
        start: BoardPoint { x: 0, y: side * 3 },
        direction: Direction::Left,
        expect: BoardPoint { x: 0, y: 0 },
    });
    tests.push(TestCase {
        name: "6 right -> 5 bottom".to_string(),
        start: BoardPoint {
            x: side - 1,
            y: side * 3 + 30,
        },
        direction: Direction::Right,
        expect: BoardPoint { x: 0, y: 0 },
    });
    tests.push(TestCase {
        name: "6 bottom -> 2 top".to_string(),
        start: BoardPoint {
            x: side - 1,
            y: side * 4 - 1,
        },
        direction: Direction::Right,
        expect: BoardPoint { x: 0, y: 0 },
    });
    // Fold is like
    // _12
    // _3_
    // 45_
    // 6__

    let abc: Vec<char> = ('a'..'z').collect();
    for (i, test) in tests.iter().enumerate() {
        log::debug!("Test {}: {}", abc[i], test.name);
        let mut direction = test.direction;
        let player = day.board.add_player(test.start, abc[i]);
        day.board.print_board_with_players_pretty();
        day.board.step_player(player, direction);
        day.board.print_board_with_players_pretty();
        let mut context = day.board.get_context();
        if context.wrapped {
            direction = context.new_direction;
            context.wrapped = false;
        }
        day.board.step_player(player, direction);
        log::debug!("");
        day.board.print_board_with_players_pretty();
        assert_eq!(test.expect, day.board.get_player_location(player));
    }

    panic!("TODO test all done");
}

impl Puzzle for Day22 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut day = Day22 {
            board: Board::new(),
            board3d: Board3D::new(),
            commands: vec![],
            context: Day22BoardContext {
                horizontal_edge: HashMap::new(),
                vertical_edge: HashMap::new(),
                wrapped: false,
                new_direction: Direction::Down,
            },
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
            if day.board.get_at(BoardPoint { x, y: 0 }) == '.' {
                start_x = x;
                break;
            }
        }
        day.board.add_player(BoardPoint { x: start_x, y: 0 }, '+');
        day.board3d.add_player(0, BoardPoint { x: 0, y: 0 }, '+');

        // Config the board
        day.board.add_wraparound(' ');
        day.board.set_wraparound_mode();
        // day.board.print_board_with_players_pretty();

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

        // Build wrap around support
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

        if test {
            // Fold is like
            // __1_
            // 234_
            // __56
            let side = width as i32 / 4;
            // 1 top -> 2 top, down (inverse)
            day.context.horizontal_edge.insert(
                HorizontalEdge {
                    y: 0,
                    x_range: side * 2..side * 3,
                    direction: Direction::Up,
                    inverse: false,
                },
                Day22Edge::Horizontal(HorizontalEdge {
                    y: side,
                    x_range: 0..side,
                    direction: Direction::Down,
                    inverse: true,
                }),
            );
            // 1 left -> 3 top, down
            day.context.vertical_edge.insert(
                VerticalEdge {
                    x: side * 2,
                    y_range: 0..side,
                    direction: Direction::Left,
                    inverse: false,
                },
                Day22Edge::Horizontal(HorizontalEdge {
                    y: side,
                    x_range: side..side * 2,
                    direction: Direction::Down,
                    inverse: false,
                }),
            );
            // 1 right -> 6 right, left (inverse)
            day.context.vertical_edge.insert(
                VerticalEdge {
                    x: side * 3 - 1,
                    y_range: 0..side,
                    direction: Direction::Right,
                    inverse: false,
                },
                Day22Edge::Vertical(VerticalEdge {
                    x: side * 4 - 1,
                    y_range: side * 2..side * 3,
                    direction: Direction::Left,
                    inverse: true,
                }),
            );
            // 2 top -> 1 top, down (inverse)
            day.context.horizontal_edge.insert(
                HorizontalEdge {
                    y: side,
                    x_range: 0..side,
                    direction: Direction::Up,
                    inverse: false,
                },
                Day22Edge::Horizontal(HorizontalEdge {
                    y: 0,
                    x_range: side * 2..side * 3,
                    direction: Direction::Down,
                    inverse: true,
                }),
            );
            // 2 left -> 6 bottom, up (inverse)
            day.context.vertical_edge.insert(
                VerticalEdge {
                    x: 0,
                    y_range: side..side * 2,
                    direction: Direction::Left,
                    inverse: false,
                },
                Day22Edge::Horizontal(HorizontalEdge {
                    y: side * 3 - 1,
                    x_range: side * 3..side * 4,
                    direction: Direction::Up,
                    inverse: true,
                }),
            );
            // 2 bottom -> 5 bottom, up (inverse)
            day.context.horizontal_edge.insert(
                HorizontalEdge {
                    y: side * 2 - 1,
                    x_range: 0..side,
                    direction: Direction::Down,
                    inverse: false,
                },
                Day22Edge::Horizontal(HorizontalEdge {
                    y: side * 3 - 1,
                    x_range: side * 2..side * 3,
                    direction: Direction::Up,
                    inverse: true,
                }),
            );
            // 3 top -> 1 left, right
            day.context.horizontal_edge.insert(
                HorizontalEdge {
                    y: side,
                    x_range: side..side * 2,
                    direction: Direction::Up,
                    inverse: false,
                },
                Day22Edge::Vertical(VerticalEdge {
                    x: side * 2,
                    y_range: 0..side,
                    direction: Direction::Right,
                    inverse: false,
                }),
            );
            // 3 bottom -> 5 left, right (inverse)
            day.context.horizontal_edge.insert(
                HorizontalEdge {
                    y: side * 2 - 1,
                    x_range: side..side * 2,
                    direction: Direction::Down,
                    inverse: false,
                },
                Day22Edge::Vertical(VerticalEdge {
                    x: side * 2,
                    y_range: side * 2..side * 3,
                    direction: Direction::Right,
                    inverse: true,
                }),
            );
            // 4 right -> 6 top, down (inverse)
            day.context.vertical_edge.insert(
                VerticalEdge {
                    x: side * 3 - 1,
                    y_range: side..side * 2,
                    direction: Direction::Right,
                    inverse: false,
                },
                Day22Edge::Horizontal(HorizontalEdge {
                    y: side * 2,
                    x_range: side * 3..side * 4,
                    direction: Direction::Down,
                    inverse: true,
                }),
            );
            // 5 left -> 3 bottom, up (inverse)
            day.context.vertical_edge.insert(
                VerticalEdge {
                    x: side * 2,
                    y_range: side * 2..side * 3,
                    direction: Direction::Left,
                    inverse: false,
                },
                Day22Edge::Horizontal(HorizontalEdge {
                    y: side * 2 - 1,
                    x_range: side * 1..side * 2,
                    direction: Direction::Up,
                    inverse: true,
                }),
            );
            // 5 botom -> 2 bottom, up (inverse)
            day.context.horizontal_edge.insert(
                HorizontalEdge {
                    y: side * 3 - 1,
                    x_range: side * 2..side * 3,
                    direction: Direction::Down,
                    inverse: false,
                },
                Day22Edge::Horizontal(HorizontalEdge {
                    y: side * 2 - 1,
                    x_range: 0..side,
                    direction: Direction::Up,
                    inverse: true,
                }),
            );
            // 6 top -> 4 right, left (inverse)
            day.context.horizontal_edge.insert(
                HorizontalEdge {
                    y: side * 2,
                    x_range: side * 3..side * 4,
                    direction: Direction::Up,
                    inverse: false,
                },
                Day22Edge::Vertical(VerticalEdge {
                    x: side * 3 - 1,
                    y_range: side..side * 2,
                    direction: Direction::Left,
                    inverse: true,
                }),
            );
            // 6 right -> 1 right, left (inverse)
            day.context.vertical_edge.insert(
                VerticalEdge {
                    x: side * 4 - 1,
                    y_range: side * 2..side * 3,
                    direction: Direction::Right,
                    inverse: false,
                },
                Day22Edge::Vertical(VerticalEdge {
                    x: side * 3 - 1,
                    y_range: 0..side,
                    direction: Direction::Left,
                    inverse: true,
                }),
            );
            // 6 bottom -> 2 left, right (inverse)
            day.context.horizontal_edge.insert(
                HorizontalEdge {
                    y: side * 3 - 1,
                    x_range: side * 3..side * 4,
                    direction: Direction::Down,
                    inverse: false,
                },
                Day22Edge::Vertical(VerticalEdge {
                    x: 0,
                    y_range: side..side * 2,
                    direction: Direction::Right,
                    inverse: true,
                }),
            );
        } else {
            // Fold is like
            // _12
            // _3_
            // 45_
            // 6__
            // 1 top -> 6 left
            // 1 left -> 4 left (inverse)
            // 2 top -> 6 bottom
            // 2 right -> 5 right (inverse)
            // 2 bottom -> 3 right
            // 3 left -> 4 top
            // 3 right -> 2 bottom
            // 4 top -> 3 left
            // 4 left -> 1 left (inverse)
            // 5 right -> 2 right (inverse)
            // 5 bottom -> 6 right
            // 6 left -> 1 top
            // 6 right -> 5 bottom
            // 6 bottom -> 2 top
        }

        day.board.set_context(&day.context);
        // test_test_input(&mut day);
        // test_real_input(&mut day);

        day.board.add_wall('#');
        day.board3d.add_wall('#');

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
        self.board.set_wraparound_custom_mode(custom_wraparound);
        let direction = navigate3d(self);
        let real_direction = self.board3d.get_player_direction(0, direction);
        let (board_id, point) = self.board3d.get_player_location(0);
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
            false => None,
        }
    }
}
