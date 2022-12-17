use std::vec;
use strum_macros::EnumIter;

use crate::utils::board::*;

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
        };
        let rows = match shape_type {
            Shapes::Flat => 1,
            Shapes::Plus => 3,
            Shapes::L => 3,
            Shapes::Tall => 4,
            Shapes::Square => 2,
        };

        let mut min_player_y = grid.height();
        for i in 0..grid.get_players_len() {
            let player_location = grid.get_player_location(i);
            min_player_y = std::cmp::min(min_player_y, player_location.y);
        }

        let new_shape_air_gap = 3;
        let air_rows_to_add = new_shape_air_gap - min_player_y;
        let new_rows = rows + air_rows_to_add;
        let width = grid.width() as usize;
        let mut y_offset = 0;
        log::debug!("Adding {} rows of air", new_rows);
        if new_rows > 0 {
            for _ in 0..new_rows {
                grid.push_front_row(vec!['.'; width]);
            }
        } else {
            // Remove rows!
            log::debug!("Special case where we need to 'remove' rows");
            y_offset = new_rows.abs();
        }

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

    pub fn move_shape(&mut self, direction: Direction, grid: &mut Board<char>) -> bool {
        let indexes = match direction {
            Direction::Left => self.move_left_indexes(),
            Direction::Right => self.move_right_indexes(),
            Direction::Down => self.move_down_indexes(),
            _ => panic!("Unexpected move direction"),
        };
        assert_eq!(indexes.len(), self.players.len());

        let mut players_moved = vec![];
        for i in indexes.iter() {
            let player = self.players[*i];
            if grid.step_player(self.players[*i], direction).is_some() {
                players_moved.push(player);
            } else {
                log::debug!("Cannot move {:?} {:?}", self.shape_type, direction);
                // Need to unmove any moved parts
                let opposite_direction = Direction::opposite_direction(direction);
                for p in &players_moved {
                    grid.step_player(*p, opposite_direction);
                }
                return false;
            }
        }
        log::debug!("Moved {:?} {:?}", self.shape_type, direction);
        true
    }

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

    fn move_right_indexes(&mut self) -> Vec<usize> {
        let mut indexes = self.move_left_indexes();
        indexes.reverse();
        indexes
    }

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
        me.grid.push_row(vec!['.'; 7]);
        me.grid.set_players_as_walls();
        me
    }

    #[allow(dead_code)]
    pub fn print(&mut self) {
        self.grid.print_board_with_players_pretty();
        println!("");
    }

    pub fn add_shape(&mut self, shape_type: Shapes) -> ShapeId {
        let shape = Shape::new(shape_type, &mut self.grid);
        self.shapes.push(shape);
        self.shapes.len() - 1
    }

    pub fn move_shape(&mut self, id: ShapeId, direction: Direction) -> bool {
        self.shapes[id].move_shape(direction, &mut self.grid)
    }

    pub fn get_stack_height(&self) -> u32 {
        let mut min_player_y = self.grid.height() - 1;
        for i in 0..self.grid.get_players_len() {
            let player_location = self.grid.get_player_location(i);
            min_player_y = std::cmp::min(min_player_y, player_location.y);
        }
        let answer = (self.grid.height() - min_player_y) as u32;
        log::info!(
            "Answer {answer} = {} - {}",
            self.grid.height(),
            min_player_y
        );
        answer
    }
}
