use anyhow::Result;
use core::panic;
use std::cmp::Eq;
use std::collections::HashMap;
use std::ops::Range;
use std::vec;

use crate::puzzle::Puzzle;
use crate::utils::board::*;

#[allow(unused_imports)]
use crate::utils::utils::*;

#[allow(unused_imports)]
use std::collections::VecDeque;

pub struct Day22 {
    board: Board<char, Day22BoardContext>,
    commands: Vec<Command>,
    context: Day22BoardContext,
}

#[derive(Debug, Clone, Copy)]
enum Command {
    Move(u32),
    TurnClockwise,        // R
    TurnCounterClockwise, // L
}

#[derive(Debug, Clone)]
enum Edge {
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
    horizontal_edge: HashMap<HorizontalEdge, Edge>,
    vertical_edge: HashMap<VerticalEdge, Edge>,
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
    directions[new_index]
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
                    Edge::Horizontal(edge_h) => {
                        new_point = BoardPoint {
                            x: edge_h.get_at(index),
                            y: edge_h.y,
                        };
                        context.new_direction = edge_h.direction;
                    }
                    Edge::Vertical(edge_v) => {
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
                    Edge::Horizontal(edge_h) => {
                        new_point = BoardPoint {
                            x: edge_h.get_at(index),
                            y: edge_h.y,
                        };
                        context.new_direction = edge_h.direction;
                    }
                    Edge::Vertical(edge_v) => {
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

impl Puzzle for Day22 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut day = Day22 {
            board: Board::new(),
            commands: vec![],
            context: Day22BoardContext {
                horizontal_edge: HashMap::new(),
                vertical_edge: HashMap::new(),
                wrapped: false,
                new_direction: Direction::Down,
            },
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
        let test = width < 20;
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
                Edge::Horizontal(HorizontalEdge {
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
                Edge::Horizontal(HorizontalEdge {
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
                Edge::Vertical(VerticalEdge {
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
                Edge::Horizontal(HorizontalEdge {
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
                Edge::Horizontal(HorizontalEdge {
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
                Edge::Horizontal(HorizontalEdge {
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
                Edge::Vertical(VerticalEdge {
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
                Edge::Vertical(VerticalEdge {
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
                Edge::Horizontal(HorizontalEdge {
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
                Edge::Horizontal(HorizontalEdge {
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
                Edge::Horizontal(HorizontalEdge {
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
                Edge::Vertical(VerticalEdge {
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
                Edge::Vertical(VerticalEdge {
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
                Edge::Vertical(VerticalEdge {
                    x: 0,
                    y_range: side..side * 2,
                    direction: Direction::Right,
                    inverse: true,
                }),
            );
        } else {
            // TODO support real input
        }

        day.board.set_context(&day.context);
        // test_test_input(&mut day);

        day.board.add_wall('#');

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
        let direction = navigate(self);
        let point = self.board.get_player_location(0);
        log::debug!("Ended at {:?}", point);
        let answer = (point.y + 1) * 1000 + (point.x + 1) * 4 + direction_value(direction);
        Ok(answer.to_string())
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(5031.to_string()),
            false => None,
        }
    }
}
