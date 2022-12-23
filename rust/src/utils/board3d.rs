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
    Bottom,
    Left,
    Right,
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
    edge: BoardEdge,
    inverse: bool,
}

#[derive(Clone, Default)]
pub struct EdgeConnection {
    edge1: BoardEdge,
    edge2: BoardEdge,
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
        });

        player_id
    }

    pub fn get_player_location(&self, player_id: PlayerId) -> (BoardId, BoardPoint) {
        let player = &self.players[player_id];
        let location = self.boards[player.board_id].get_player_location(player_id);
        (player.board_id, location)
    }

    pub fn set_edge_connection(&mut self, connection: EdgeConnection) {
        self.configs[connection.edge1.id].connections[connection.edge1.edge.clone() as usize] =
            BoardEdgeConnection {
                edge: connection.edge2.clone(),
                inverse: connection.inverse,
            };
        self.configs[connection.edge2.id].connections[connection.edge2.edge.clone() as usize] =
            BoardEdgeConnection {
                edge: connection.edge1.clone(),
                inverse: connection.inverse,
            };
    }

    pub fn step_player(&mut self, player_id: PlayerId, direction: Direction) -> Option<T> {
        let (board_id, location) = self.get_player_location(player_id);

        // Test for moving off board condition
        let moved_to_new_board = false;
        // let new_board_id;
        if direction == Direction::Left && location.x == 0 {
        } else if direction == Direction::Right && location.x == self.width() - 1 {
        } else if direction == Direction::Up && location.y == 0 {
        } else if direction == Direction::Down && location.y == self.height() - 1 {
        } else {
            panic!("Unsupported move direction");
        }

        return self.boards[board_id].step_player(player_id, direction);
    }
}
