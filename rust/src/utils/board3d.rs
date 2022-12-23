use core::fmt::Debug;

use crate::utils::board::*;

pub type BoardId = usize;

pub struct Board3D<T>
where
    T: Clone + Copy + Debug + PartialEq + std::fmt::Display,
{
    boards: Vec<Board<T, BoardDefaultContext>>,
    configs: Vec<BoardConfig>,
    players: Vec<Player3D>,
}

struct Player3D {
    id: PlayerId,
    board_id: BoardId,
    direction_offset: i32,
}

#[derive(Clone)]
struct BoardConfig {
    connections: Vec<BoardEdgeConnection>,
}

impl BoardConfig {
    pub fn new() -> BoardConfig {
        BoardConfig {
            connections: vec![
                BoardEdgeConnection {
                    ..Default::default()
                };
                4
            ],
        }
    }
}

#[derive(Clone, Default)]
pub enum Edge {
    Top = 0,
    Right,
    Bottom,
    Left,
    #[default]
    None = 100,
}

#[derive(Clone, Default)]
pub struct BoardEdge {
    id: BoardId,
    edge: Edge,
}

#[derive(Clone, Default)]
struct BoardEdgeConnection {
    board_edge: BoardEdge,
    inverse: bool,
}

#[derive(Clone, Default)]
pub struct EdgeConnection {
    board_edge1: BoardEdge,
    board_edge2: BoardEdge,
    inverse: bool,
}

impl<T> Board3D<T>
where
    T: Clone + Copy + Debug + PartialEq + std::fmt::Display,
{
    pub fn new() -> Board3D<T> {
        Board3D {
            boards: vec![
                Board::new(),
                Board::new(),
                Board::new(),
                Board::new(),
                Board::new(),
                Board::new(),
            ],
            configs: vec![BoardConfig::new(); 6],
            players: vec![],
        }
    }

    pub fn push_row(&mut self, board_id: BoardId, row: Vec<T>) {
        self.boards[board_id].push_row(row);
    }

    pub fn width(&self) -> i32 {
        self.boards[0].width()
    }

    pub fn height(&self) -> i32 {
        self.boards[0].height()
    }

    pub fn add_player(&mut self, board_id: BoardId, point: BoardPoint, id: T) -> PlayerId {
        for (this_board_id, board) in self.boards.iter_mut().enumerate() {
            let player_id = board.add_player(point, id);
            if this_board_id != board_id {
                board.set_player_visible(player_id, false);
            }
        }

        let player_id = self.players.len();
        self.players.push(Player3D {
            id: player_id,
            board_id,
            direction_offset: 0,
        });

        player_id
    }

    pub fn add_wall(&mut self, wall: T) {
        for board in self.boards.iter_mut() {
            board.add_wall(wall);
        }
    }

    pub fn get_player_location(&self, player_id: PlayerId) -> (BoardId, BoardPoint) {
        let player = &self.players[player_id];
        let location = self.boards[player.board_id].get_player_location(player_id);
        (player.board_id, location)
    }

    pub fn set_edge_connection(&mut self, connection: EdgeConnection) {
        self.configs[connection.board_edge1.id].connections
            [connection.board_edge1.edge.clone() as usize] = BoardEdgeConnection {
            board_edge: connection.board_edge2.clone(),
            inverse: connection.inverse,
        };
        self.configs[connection.board_edge2.id].connections
            [connection.board_edge2.edge.clone() as usize] = BoardEdgeConnection {
            board_edge: connection.board_edge1.clone(),
            inverse: connection.inverse,
        };
    }

    fn get_new_value(&self, value: i32, inverse: bool) -> i32 {
        assert_eq!(self.width(), self.height()); // This logic assumes cube only
        match inverse {
            false => value,
            true => self.width() - 1 - value,
        }
    }

    fn get_direction_offset(&self, direction: Direction, new_edge: Edge) -> i32 {
        let directions = vec![
            // This order is intentionally different than Edge:: as it should be offset by index 2
            Direction::Down,
            Direction::Left,
            Direction::Up,
            Direction::Right,
        ];
        let direction_index = directions.iter().position(|&x| x == direction).unwrap() as i32;
        let new_edge_index = new_edge as i32;
        if new_edge_index == direction_index {
            return 0;
        } else if new_edge_index > direction_index {
            return new_edge_index - direction_index;
        } else {
            return directions.len() as i32 + new_edge_index - direction_index;
        }
    }

    pub fn step_player(&mut self, player_id: PlayerId, direction: Direction) -> Option<T> {
        let player3d = &self.players[player_id];
        let (board_id, location) = self.get_player_location(player_id);

        // Use requested direction with current board direction offset to find real direction
        let directions = vec![
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ];
        let direction_index = directions.iter().position(|&x| x == direction).unwrap();
        let new_index = (direction_index as i32 + player3d.direction_offset)
            .rem_euclid(directions.len() as i32) as usize;
        let real_direction = directions[new_index];

        // Test for moving off board condition
        let mut moved_to_new_board = true;
        let mut connection = BoardEdgeConnection {
            ..Default::default()
        };
        if real_direction == Direction::Left && location.x == 0 {
            connection = self.configs[board_id].connections[Edge::Left as usize].clone();
        } else if real_direction == Direction::Right && location.x == self.width() - 1 {
        } else if real_direction == Direction::Up && location.y == 0 {
        } else if real_direction == Direction::Down && location.y == self.height() - 1 {
        } else {
            moved_to_new_board = false;
        }

        if moved_to_new_board {
            let new_board_id = connection.board_edge.id;
            let new_board_edge = connection.board_edge.edge;
            let new_location = match new_board_edge {
                Edge::Left => BoardPoint {
                    x: 0,
                    y: self.get_new_value(location.y, connection.inverse),
                },
                Edge::Right => BoardPoint {
                    x: self.width() - 1,
                    y: self.get_new_value(location.y, connection.inverse),
                },
                Edge::Top => BoardPoint {
                    x: self.get_new_value(location.y, connection.inverse),
                    y: 0,
                },
                Edge::Bottom => BoardPoint {
                    x: self.get_new_value(location.y, connection.inverse),
                    y: self.height() - 1,
                },
                _ => panic!("Unsupported edge"),
            };
            if !self.boards[new_board_id].is_wall_here(new_location) {
                log::debug!("Moving to new board {board_id} -> {new_board_id}");
                self.boards[new_board_id].set_player_location(player_id, new_location);
                self.boards[new_board_id].set_player_visible(player_id, true);
                self.boards[board_id].set_player_visible(player_id, true);
                self.boards[new_board_id].set_player_visible(player_id, false);
                self.players[player_id].direction_offset +=
                    self.get_direction_offset(real_direction, new_board_edge);

                return Some(self.boards[new_board_id].get_at(new_location));
            } else {
                log::debug!("Cannot move to new board, there is a wall");
                return None;
            }
        }

        return self.boards[board_id].step_player(player_id, real_direction);
    }

    pub fn print_board3d_with_players_pretty(&mut self) {
        for (i, board) in self.boards.iter_mut().enumerate() {
            println!("Board {i}");
            board.print_board_with_players_pretty();
            println!("");
        }
    }
}
