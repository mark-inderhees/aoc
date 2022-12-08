use grid::*;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
pub struct Board<T> {
    pub grid: Grid<T>,
    location: Point,
}

#[derive(Debug, EnumIter, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl<T> Board<T> {
    pub fn new() -> Board<T> {
        Board {
            grid: grid![],
            location: Point { x: 0, y: 0 },
        }
    }

    pub fn set_location(&mut self, x: i32, y: i32) {
        self.location.x = x;
        self.location.y = y;
    }

    pub fn get_current_value(&self) -> &T {
        let x: usize = self.location.x.try_into().unwrap();
        let y: usize = self.location.y.try_into().unwrap();
        self.grid.get(y, x).unwrap()
    }

    pub fn step(&mut self, direction: Direction) -> Option<&T> {
        let (step_x, step_y) = match direction {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };

        let new_location = Point {
            x: self.location.x + step_x,
            y: self.location.y + step_y,
        };

        let x_max: i32 = self.grid.size().0.try_into().unwrap();
        let y_max: i32 = self.grid.size().1.try_into().unwrap();
        match new_location {
            _ if new_location.x == -1 => None,
            _ if new_location.y == -1 => None,
            _ if new_location.x == x_max => None,
            _ if new_location.y == y_max => None,
            _ => {
                self.location = new_location;
                let x: usize = new_location.x.try_into().unwrap();
                let y: usize = new_location.y.try_into().unwrap();
                Some(self.grid.get(y, x).unwrap())
            }
        }
    }
}