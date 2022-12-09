use grid::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
pub struct Board<T> {
    grid: Grid<T>,
    // location: Point,
    players: Vec<Point>,
}

#[derive(Debug, EnumIter, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl Direction {
    pub fn iterator() -> DirectionIter {
        Direction::iter()
    }
}

impl<T> Board<T> {
    pub fn new() -> Board<T> {
        Board {
            grid: grid![],
            // location: Point { x: 0, y: 0 },
            players: vec![Point { x: 0, y: 0 }],
        }
    }

    pub fn grid(&self) -> &Grid<T> {
        &self.grid
    }

    pub fn push_row(&mut self, row: Vec<T>) {
        self.grid.push_row(row);
    }

    pub fn width(&self) -> i32 {
        self.grid.size().0.try_into().unwrap()
    }

    pub fn height(&self) -> i32 {
        self.grid.size().1.try_into().unwrap()
    }

    pub fn set_at(&mut self, x: i32, y: i32, value: T) {
        let x_: usize = x.try_into().unwrap();
        let y_: usize = y.try_into().unwrap();
        self.grid[y_][x_] = value;
    }

    pub fn set_location(&mut self, x: i32, y: i32) {
        self.players[0].x = x;
        self.players[0].y = y;
    }

    pub fn get_current_value(&self) -> &T {
        let x: usize = self.players[0].x.try_into().unwrap();
        let y: usize = self.players[0].y.try_into().unwrap();
        self.grid.get(y, x).unwrap()
    }

    pub fn step(&mut self, direction: Direction) -> Option<&T> {
        let (step_x, step_y) = match direction {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            _ => panic!("todo"),
        };

        let new_location = Point {
            x: self.players[0].x + step_x,
            y: self.players[0].y + step_y,
        };

        let x_max: i32 = self.grid.size().0.try_into().unwrap();
        let y_max: i32 = self.grid.size().1.try_into().unwrap();
        match new_location {
            _ if new_location.x == -1 => None,
            _ if new_location.y == -1 => None,
            _ if new_location.x == x_max => None,
            _ if new_location.y == y_max => None,
            _ => {
                self.players[0] = new_location;
                let x: usize = new_location.x.try_into().unwrap();
                let y: usize = new_location.y.try_into().unwrap();
                Some(self.grid.get(y, x).unwrap())
            }
        }
    }

    pub fn is_nearby(&self, player1: usize, player2: usize) -> bool {
        let p1 = self.players[player1];
        let p2 = self.players[player2];


    }
}
