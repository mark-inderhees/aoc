use std::vec;
use strum_macros::EnumIter;

use crate::utils::board::*;

/// A game of tetris. It is played on a grid with a couple of shape types.
/// Moves are down, left, right. Rotation is not supported.
/// A collison will prevent shape from moving.
/// The board grows infinitely tall.
/// Shapes start a configurable number of spaces above the highest current shape.
/// Shapes start 2 spaces from the left.
pub struct Tetris {
    /// The game board.
    grid: Board<char>,

    /// The shapes currently on the grid.
    shapes: Vec<Shape>,

    /// The width of the board.
    width: i32,

    /// Rows of air between top of tower and new shape
    new_shape_air_gap: i32,
}

/// The supported Tetris shape types.
#[derive(Debug, Clone, Copy, EnumIter, PartialEq)]
pub enum Shapes {
    // ####
    Flat,

    //  #
    // ###
    //  #
    Plus,

    //   #
    //   #
    // ###
    L,

    // #
    // #
    // #
    // #
    Tall,

    // ##
    // ##
    Square,
}

/// A unique ID for a shape on the board.
pub type ShapeId = usize;

impl Tetris {
    /// Create a new tetris board with width 7 and air cap for new shapes 3.
    pub fn new() -> Tetris {
        let mut me = Tetris {
            grid: Board::new(),
            shapes: vec![],
            width: 7,
            new_shape_air_gap: 3,
        };
        me.grid.push_row(vec!['.'; me.width as usize]);
        me.grid.set_players_as_walls();
        me
    }

    /// Print what the game looks like.
    #[allow(dead_code)]
    pub fn print(&mut self) {
        self.grid.print_board_with_players_pretty();
        println!("");
    }

    /// Add a new shape to the board at the default start location.
    pub fn add_shape(&mut self, shape_type: Shapes) -> ShapeId {
        let shape = Shape::new(shape_type, &mut self.grid, self.new_shape_air_gap);
        self.shapes.push(shape);
        self.shapes.len() - 1
    }

    /// Move the shape, return true if moved or false if it could not move.
    pub fn move_shape(&mut self, id: ShapeId, direction: Direction) -> bool {
        self.shapes[id].move_shape(direction, &mut self.grid)
    }

    /// Get how tall the shape tower is.
    pub fn stack_height(&self) -> u32 {
        let min_player_y = self.grid.player_minimum_height();
        let stack_height = (self.grid.height() - min_player_y) as u32;
        stack_height
    }

    /// Does the top most line have a full row of blocks.
    #[allow(dead_code)]
    pub fn is_top_line_full(&self) -> bool {
        let min_player_y = self.grid.player_minimum_height();
        for x in 0..self.width {
            let value = self.grid.value_at(BoardPoint { x, y: min_player_y });
            if value != '#' {
                return false;
            }
        }

        true
    }

    /// Output a string version of the specified rows.
    pub fn rows_as_string(&self, rows: u32) -> String {
        let min_player_y = self.grid.player_minimum_height();
        let mut output = String::new();
        for x in 0..self.width {
            for y in min_player_y..min_player_y + rows as i32 {
                output.push(self.grid.value_at_with_player(BoardPoint { x, y }));
            }
        }
        output
    }
}

/// A shape consists of a couple of players on the grid.
struct Shape {
    shape_type: Shapes,
    players: Vec<PlayerId>,
}

/// A private helper struct to manage shape logic. Create and move shapes.
impl Shape {
    /// Create a new shape and add it to the grid
    fn new(shape_type: Shapes, grid: &mut Board<char>, new_shape_air_gap: i32) -> Shape {
        let locations = Shape::shape_locations(shape_type);
        let rows_for_shape = match shape_type {
            Shapes::Flat => 1,
            Shapes::Plus => 3,
            Shapes::L => 3,
            Shapes::Tall => 4,
            Shapes::Square => 2,
        };

        // Add new rows to the grid based of the current highest block, the
        // rows this shape needs, and configurable rows between new block and
        // current highest block
        let min_player_y = if grid.height() > 1 {
            grid.player_minimum_height()
        } else {
            // Special case for first shape where there are no other shapes
            1
        };
        let air_rows_to_add = new_shape_air_gap - min_player_y;
        let new_rows = rows_for_shape + air_rows_to_add;
        let width = grid.width() as usize;
        let mut y_offset = 0;
        log::debug!("Adding {} rows of air", new_rows);
        if new_rows > 0 {
            for _ in 0..new_rows {
                grid.push_front_row(vec!['.'; width]);
            }
        } else {
            // Remove rows! Instead of modifying the grid, just place the new
            // shape a little lower
            log::debug!("Special case where we need to 'remove' rows");
            y_offset = new_rows.abs();
        }

        // Add the new shape to the board
        log::debug!("Adding {:?} at {},{}", shape_type, 2, y_offset);
        let mut players = vec![];
        for location in locations {
            let player = grid.add_player(
                BoardPoint {
                    x: location.x + 2,
                    y: location.y + y_offset,
                },
                '#',
            );
            players.push(player);
        }

        Shape {
            shape_type,
            players,
        }
    }

    /// Return a vector of location points for a new shape
    /// The top left of the shape area will be at 0,0
    /// The locations are from left->right, top->bottom
    fn shape_locations(shape_type: Shapes) -> Vec<BoardPoint> {
        match shape_type {
            Shapes::Flat => vec![
                BoardPoint { x: 0, y: 0 },
                BoardPoint { x: 1, y: 0 },
                BoardPoint { x: 2, y: 0 },
                BoardPoint { x: 3, y: 0 },
            ],
            Shapes::Plus => vec![
                BoardPoint { x: 1, y: 0 },
                BoardPoint { x: 0, y: 1 },
                BoardPoint { x: 1, y: 1 },
                BoardPoint { x: 2, y: 1 },
                BoardPoint { x: 1, y: 2 },
            ],
            Shapes::L => vec![
                BoardPoint { x: 2, y: 0 },
                BoardPoint { x: 2, y: 1 },
                BoardPoint { x: 0, y: 2 },
                BoardPoint { x: 1, y: 2 },
                BoardPoint { x: 2, y: 2 },
            ],
            Shapes::Tall => vec![
                BoardPoint { x: 0, y: 0 },
                BoardPoint { x: 0, y: 1 },
                BoardPoint { x: 0, y: 2 },
                BoardPoint { x: 0, y: 3 },
            ],
            Shapes::Square => vec![
                BoardPoint { x: 0, y: 0 },
                BoardPoint { x: 1, y: 0 },
                BoardPoint { x: 0, y: 1 },
                BoardPoint { x: 1, y: 1 },
            ],
        }
    }

    /// Move the shape the desired direction, returning true if the move was successfull
    /// On failure to move, none of the shape was actually moved
    fn move_shape(&mut self, direction: Direction, grid: &mut Board<char>) -> bool {
        // Find the order to move parts of the shape in so they do not collide with each other
        let indexes = match direction {
            Direction::Left => self.move_left_indexes(),
            Direction::Right => self.move_right_indexes(),
            Direction::Down => self.move_down_indexes(),
            _ => panic!("Unexpected move direction"),
        };
        assert_eq!(indexes.len(), self.players.len());

        // Move the players one at a time
        let mut players_moved = vec![];
        for i in indexes.iter() {
            let player = self.players[*i];
            if grid.step_player(self.players[*i], direction).is_some() {
                players_moved.push(player);
            } else {
                // Cannot move! Need to unmove any moved parts in reverse order
                log::debug!("Cannot move {:?} {:?}", self.shape_type, direction);
                let opposite_direction = Direction::opposite_direction(direction);
                while players_moved.len() > 0 {
                    log::debug!("Unmoving part of piece {:?}", opposite_direction);
                    grid.step_player(players_moved.pop().unwrap(), opposite_direction);
                }
                return false;
            }
        }
        log::debug!("Moved {:?} {:?}", self.shape_type, direction);
        true
    }

    /// Move left helper
    fn move_left_indexes(&mut self) -> Vec<usize> {
        let indexes: Vec<usize> = match self.shape_type {
            Shapes::Flat => vec![0, 1, 2, 3],
            Shapes::Plus => vec![1, 0, 2, 4, 3],
            Shapes::L => vec![2, 3, 4, 0, 1],
            Shapes::Tall => vec![0, 1, 2, 3],
            Shapes::Square => vec![0, 2, 1, 3],
        };
        indexes
    }

    /// Move right helper (the reverse of move left)
    fn move_right_indexes(&mut self) -> Vec<usize> {
        let mut indexes = self.move_left_indexes();
        indexes.reverse();
        indexes
    }

    /// Move down helper
    fn move_down_indexes(&mut self) -> Vec<usize> {
        let indexes: Vec<usize> = match self.shape_type {
            Shapes::Flat => vec![0, 1, 2, 3],
            Shapes::Plus => vec![4, 3, 2, 1, 0],
            Shapes::L => vec![4, 3, 2, 1, 0],
            Shapes::Tall => vec![3, 2, 1, 0],
            Shapes::Square => vec![3, 2, 1, 0],
        };
        indexes
    }
}
