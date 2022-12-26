use core::fmt::Debug;

use crate::utils::board::*;

/// A 3D board for walking a player around. It's a 3D cube with 6 sides.
/// The player walks around the sides. Walls are supported.
pub struct Board3D<T>
where
    T: Clone + Copy + Debug + PartialEq + std::fmt::Display,
{
    /// The 6 boards
    boards: Vec<Board<T>>,

    /// Configs of how the 6 board edges are connected
    configs: Vec<BoardConfig>,

    /// The players on the 3D map. The player is always present on all 6 boards
    /// but hidden when not on that board.
    players: Vec<Player3D>,
}

/// A unique id for each of the 6 faces on the cube
pub type BoardId = usize;

/// The 3D player. Contains which board the player is currently on. And the
/// current rotation offset of the player to convert from user commands to
/// board commands.
struct Player3D {
    board_id: BoardId,
    direction_offset: i32,
}

/// Each of the 4 board edges connects to another board
#[derive(Clone)]
struct BoardConfig {
    connections: Vec<BoardEdgeConnection>,
}

/// The 4 types of edges with a default of none. Index of none is set high to
/// break bad code.
#[derive(Clone, Default, Debug)]
pub enum Edge {
    Top = 0,
    Right,
    Bottom,
    Left,
    #[default]
    None = 100,
}

impl<T> Board3D<T>
where
    T: Clone + Copy + Debug + PartialEq + std::fmt::Display,
{
    /// Create a new 3D board with 6 empty boards
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

    /// Populate a row on a specific board
    pub fn push_row(&mut self, board_id: BoardId, row: Vec<T>) {
        self.boards[board_id].push_row(row);
    }

    /// Get the width of the cube
    pub fn width(&self) -> i32 {
        self.boards[0].width()
    }

    /// Get the height of the cube
    pub fn height(&self) -> i32 {
        self.boards[0].height()
    }

    /// Add a player to the board. The player is present on all boards but
    /// hidden when not actually there.
    pub fn add_player(&mut self, board_id: BoardId, point: BoardPoint, id: T) -> PlayerId {
        for (this_board_id, board) in self.boards.iter_mut().enumerate() {
            let player_id = board.add_player(point, id);
            if this_board_id != board_id {
                board.set_player_visible(player_id, false);
            }
        }

        let player_id = self.players.len();
        self.players.push(Player3D {
            board_id,
            direction_offset: 0,
        });

        player_id
    }

    /// Add wall type.
    pub fn add_wall(&mut self, wall: T) {
        for board in self.boards.iter_mut() {
            board.add_wall(wall);
        }
    }

    /// Get which board the player is on and the location on that board.
    pub fn player_location(&self, player_id: PlayerId) -> (BoardId, BoardPoint) {
        let player = &self.players[player_id];
        let location = self.boards[player.board_id].player_location(player_id);
        (player.board_id, location)
    }

    /// Get the current direction of the player given the input of the last client direction command.
    /// Converts from client commands into actually board direction.
    pub fn player_direction(&self, player_id: PlayerId, direction: Direction) -> Direction {
        // Lookup the requested direction index
        let directions = vec![
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ];
        let direction_index = directions.iter().position(|&x| x == direction).unwrap();

        // Use offset to get the real index
        let direction_offset = self.players[player_id].direction_offset;
        let new_index = (direction_index as i32 + direction_offset)
            .rem_euclid(directions.len() as i32) as usize;

        // Return the actual direction
        let real_direction = directions[new_index];
        real_direction
    }

    /// Configure how boards connect. The connection is applied to both boards.
    pub fn set_edge(&mut self, connection: EdgeConnection) {
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

    /// Helper function when moving between boards. Convert value from one board to another.
    fn convert_location_value(&self, value: i32, inverse: bool) -> i32 {
        assert_eq!(self.width(), self.height()); // This logic assumes cube only
        match inverse {
            false => value,
            true => self.width() - 1 - value,
        }
    }

    /// Helper function when moving between boards. If we are moving to a board
    /// edge that is different than current direction, we need to rotate the
    /// direction to compensate for this edge connection type.
    /// Consider this map folded into a cube:
    /// _01
    /// _2_
    /// 34_
    /// 5__
    /// Board 1 bottom meets 2 right. When going down on 1, new direction is left on 2.
    /// new_edge_index=3
    /// direction_index=0
    /// So this function returns offset of 3.
    /// So for Edge enum
    ///     Top = 0,    <--- would be here, but now +3
    ///     Right,
    ///     Bottom,
    ///     Left,       <--- ends up here like we need it
    fn determine_direction_offset(&self, direction: Direction, new_edge: Edge) -> i32 {
        let new_edge_index = new_edge as i32;

        // Get expected index, it is the opposite the direction. For example,
        // when moving down we come to the top edge of the new board.
        let expected_edge = Direction::opposite_direction(direction);
        let directions = vec![
            // This order must match order in Edge::
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ];
        let expected_index = directions.iter().position(|&x| x == expected_edge).unwrap() as i32;

        // Direction and edge type match, no offset needed
        if new_edge_index == expected_index {
            return 0;
        }

        // Ensure positive offsets only. Will make mod easier.
        if new_edge_index > expected_index {
            return new_edge_index - expected_index;
        } else {
            return directions.len() as i32 + new_edge_index - expected_index;
        }
    }

    /// Move the player on the 3d board. If they move off one board move the player to the correct new board.
    pub fn step_player(&mut self, player_id: PlayerId, direction: Direction) -> Option<T> {
        let (board_id, location) = self.player_location(player_id);

        // Use requested direction with current board direction offset to find real direction
        let real_direction = self.player_direction(player_id, direction);

        // Test for moving off board condition
        let mut moved_to_new_board = true;
        let mut connection = BoardEdgeConnection {
            ..Default::default()
        };
        let mut value = 0; // The location value to use on the new board
        if real_direction == Direction::Left && location.x == 0 {
            connection = self.configs[board_id].connections[Edge::Left as usize].clone();
            value = location.y;
        } else if real_direction == Direction::Right && location.x == self.width() - 1 {
            connection = self.configs[board_id].connections[Edge::Right as usize].clone();
            value = location.y;
        } else if real_direction == Direction::Up && location.y == 0 {
            connection = self.configs[board_id].connections[Edge::Top as usize].clone();
            value = location.x;
        } else if real_direction == Direction::Down && location.y == self.height() - 1 {
            connection = self.configs[board_id].connections[Edge::Bottom as usize].clone();
            value = location.x;
        } else {
            moved_to_new_board = false;
        }

        // Need to move boards
        if moved_to_new_board {
            let new_board_id = connection.board_edge.id;
            let new_board_edge = connection.board_edge.edge;

            // Get location on the new board
            let new_location = match new_board_edge {
                Edge::Left => BoardPoint {
                    x: 0,
                    y: self.convert_location_value(value, connection.inverse),
                },
                Edge::Right => BoardPoint {
                    x: self.width() - 1,
                    y: self.convert_location_value(value, connection.inverse),
                },
                Edge::Top => BoardPoint {
                    x: self.convert_location_value(value, connection.inverse),
                    y: 0,
                },
                Edge::Bottom => BoardPoint {
                    x: self.convert_location_value(value, connection.inverse),
                    y: self.height() - 1,
                },
                _ => panic!("Unsupported edge {new_board_edge:?}"),
            };

            if !self.boards[new_board_id].is_wall_here(new_location) {
                // Move to new board, updating player location and visibility
                log::debug!("Moving to new board {board_id} -> {new_board_id}");
                self.boards[new_board_id].set_player_location(player_id, new_location);
                self.boards[board_id].set_player_visible(player_id, false);
                self.boards[new_board_id].set_player_visible(player_id, true);
                self.players[player_id].board_id = new_board_id;

                // Update direction compenstation for moving between boards
                self.players[player_id].direction_offset +=
                    self.determine_direction_offset(real_direction, new_board_edge);

                // Return the value of the grid at this location
                return Some(self.boards[new_board_id].value_at(new_location));
            } else {
                // Cannot move to new board, stay on current board
                log::debug!("Cannot move to new board, there is a wall on {new_board_id} at {new_location:?}");
                return None;
            }
        }

        // Do not need to move boards, do a simple move on current board
        self.boards[board_id].step_player(player_id, real_direction)
    }

    /// Print each of the 6 boards
    #[allow(dead_code)]
    pub fn print_board3d_with_players_pretty(&mut self) {
        for (i, board) in self.boards.iter_mut().enumerate() {
            println!("Board {i}");
            board.print_board_with_players_pretty();
            println!("");
        }
    }
}

impl BoardConfig {
    /// Create a default config, with empty connects for each of the 4 edges
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

/// How two boards connect together on their two edges. Use EdgeConnection::new() to create.
#[derive(Clone, Default)]
pub struct EdgeConnection {
    board_edge1: BoardEdge,
    board_edge2: BoardEdge,
    inverse: bool,
}

impl EdgeConnection {
    /// Create a connection between two boards. Specify the board ids, edge
    /// types, and if these boards need inverse connection due to flipped board.
    pub fn new(
        id1: BoardId,
        edge1: Edge,
        id2: BoardId,
        edge2: Edge,
        inverse: bool,
    ) -> EdgeConnection {
        EdgeConnection {
            board_edge1: BoardEdge {
                id: id1,
                edge: edge1,
            },
            board_edge2: BoardEdge {
                id: id2,
                edge: edge2,
            },
            inverse,
        }
    }
}

/// Internal use only.
/// The board edge, used to define board connections. Has the id of this board
/// and the edge of this board.
#[derive(Clone, Default)]
struct BoardEdge {
    id: BoardId,
    edge: Edge,
}

/// Internal use only.
/// The edge connection info. Has id of board, type of edge, and if the value
/// needs to be inverted due to flipped board connections.
#[derive(Clone, Default)]
struct BoardEdgeConnection {
    board_edge: BoardEdge,
    inverse: bool,
}
