use core::fmt::Debug;
use std::default;

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

    pub fn push_row(&mut self, id: BoardId, row: Vec<T>) {
        self.boards[id].push_row(row);
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
}
