use std::vec;
use strum_macros::EnumIter;

use crate::utils::board::*;

#[derive(Debug, Clone, Copy)]
pub enum Command {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, EnumIter)]
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

struct Shape {
    shape_type: Shapes,
    players: Vec<PlayerId>,
}

impl Shape {
    pub fn new(shape_type: Shapes, grid: &mut Board<char>) -> Shape {
        let locations = match shape_type {
            Shapes::Flat => vec![
                BoardPoint { x: 0, y: 0 },
                BoardPoint { x: 1, y: 0 },
                BoardPoint { x: 1, y: 0 },
                BoardPoint { x: 1, y: 0 },
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
        };
        let rows = match shape_type {
            Shapes::Flat => 1,
            Shapes::Plus => 3,
            Shapes::L => 3,
            Shapes::Tall => 4,
            Shapes::Square => 2,
        };

        let mut max_player_y = 0;
        for i in 0..grid.get_players_len() {
            let player_location = grid.get_player_location(i);
            max_player_y = std::cmp::max(max_player_y, player_location.y);
        }

        let air_rows_to_add = 3 - grid.height() - max_player_y;
        assert!(air_rows_to_add >= 0);
        let width = grid.width() as usize;
        for _ in 0..rows + air_rows_to_add {
            grid.push_front_row(vec!['.'; width]);
        }

        let mut players = vec![];
        for location in locations {
            let player = grid.add_player(
                BoardPoint {
                    x: location.x + 2,
                    y: location.y + max_player_y + 3,
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

    pub fn move_shape(&mut self, direction: Direction, grid: &mut Board<char>) -> bool {
        match direction {
            Direction::Left => self.move_left(grid),
            Direction::Right => self.move_right(grid),
            Direction::Down => self.move_down(grid),
            _ => panic!("Unexpected move direction"),
        }
    }

    fn move_left(&mut self, grid: &mut Board<char>) -> bool {
        let indexes: Vec<usize> = match self.shape_type {
            Shapes::Flat => vec![0, 1, 2, 3],
            Shapes::Plus => vec![1, 0, 2, 4, 3],
            Shapes::L => vec![2, 3, 4, 0, 1],
            Shapes::Tall => vec![0, 1, 2, 3],
            Shapes::Square => vec![0, 2, 1, 3],
        };
        assert_eq!(indexes.len(), self.players.len());

        for i in indexes.iter() {
            if !grid.can_step_player(self.players[*i], Direction::Left) {
                log::debug!("Cannot move {:?} left", self.shape_type);
                return false;
            }
        }

        for i in indexes.iter() {
            grid.step_player(self.players[*i], Direction::Left);
        }
        true
    }

    fn move_right(&mut self, grid: &mut Board<char>) -> bool {
        // TODO make this a helper!
        let mut indexes: Vec<usize> = match self.shape_type {
            Shapes::Flat => vec![0, 1, 2, 3],
            Shapes::Plus => vec![1, 0, 2, 4, 3],
            Shapes::L => vec![2, 3, 4, 0, 1],
            Shapes::Tall => vec![0, 1, 2, 3],
            Shapes::Square => vec![0, 2, 1, 3],
        };
        indexes.reverse(); // NOTE this little hack
        assert_eq!(indexes.len(), self.players.len());

        for i in indexes.iter() {
            if !grid.can_step_player(self.players[*i], Direction::Right) {
                log::debug!("Cannot move {:?} right", self.shape_type);
                return false;
            }
        }

        for i in indexes.iter() {
            grid.step_player(self.players[*i], Direction::Right);
        }
        true
    }

    fn move_down(&mut self, grid: &mut Board<char>) -> bool {
        let indexes: Vec<usize> = match self.shape_type {
            Shapes::Flat => vec![0, 1, 2, 3],
            Shapes::Plus => vec![4, 3, 2, 1, 0],
            Shapes::L => vec![4, 3, 2, 1, 0],
            Shapes::Tall => vec![3, 2, 1, 0],
            Shapes::Square => vec![3, 2, 1, 0],
        };
        assert_eq!(indexes.len(), self.players.len());

        for i in indexes.iter() {
            if !grid.can_step_player(self.players[*i], Direction::Down) {
                log::debug!("Cannot move {:?} down", self.shape_type);
                return false;
            }
        }

        for i in indexes.iter() {
            grid.step_player(self.players[*i], Direction::Down);
        }
        true
    }
}

pub struct Tetris {
    grid: Board<char>,
    shapes: Vec<Shape>,
}

pub type ShapeId = usize;

impl Tetris {
    pub fn new() -> Tetris {
        let mut me = Tetris {
            grid: Board::new(),
            shapes: vec![],
        };
        me.grid.set_players_as_walls();
        me
    }

    pub fn add_shape(&mut self, shape_type: Shapes) -> ShapeId {
        let shape = Shape::new(shape_type, &mut self.grid);
        self.shapes.push(shape);
        self.shapes.len() - 1
    }

    pub fn move_shape(&mut self, id: ShapeId, direction: Direction) -> bool {
        self.shapes[id].move_shape(direction, &mut self.grid)
    }
}
