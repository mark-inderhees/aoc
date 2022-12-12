use std::fmt::Debug;
use std::iter::zip;

use grid::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy)]
struct Player<T>
where
    T: Clone,
    T: Copy,
    T: Debug,
{
    x: i32,
    y: i32,
    id: T,
}

#[derive(Debug)]
pub struct Board<T>
where
    T: Clone,
    T: Copy,
    T: Debug,
{
    grid: Grid<T>,
    players: Vec<Player<T>>,
}

#[derive(Debug, EnumIter, Clone, Copy, PartialEq)]
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
    #[allow(dead_code)]
    pub fn iterator() -> DirectionIter {
        Direction::iter()
    }

    pub fn straight_iterator() -> Vec<Direction> {
        vec![
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
    }
}

impl<T> Board<T>
where
    T: Clone,
    T: Copy,
    T: Debug,
{
    pub fn new() -> Board<T> {
        Board {
            grid: grid![],
            players: vec![],
        }
    }

    pub fn grid(&self) -> &Grid<T> {
        &self.grid
    }

    pub fn push_row(&mut self, row: Vec<T>) {
        self.grid.push_row(row);
    }

    pub fn add_player(&mut self, x: i32, y: i32, id: T) -> usize {
        self.players.push(Player { x, y, id });
        self.players.len() - 1
    }

    pub fn width(&self) -> i32 {
        self.grid.cols() as i32
    }

    pub fn height(&self) -> i32 {
        self.grid.rows() as i32
    }

    pub fn set_at(&mut self, x: i32, y: i32, value: T) {
        let x_: usize = x.try_into().unwrap();
        let y_: usize = y.try_into().unwrap();
        self.grid[y_][x_] = value;
    }

    pub fn set_location(&mut self, x: i32, y: i32) {
        let player = 0;
        self.set_player_location(player, x, y);
    }

    pub fn set_player_location(&mut self, player: usize, x: i32, y: i32) {
        self.players[player].x = x;
        self.players[player].y = y;
    }

    pub fn get_player_location(&self, player: usize) -> (i32, i32) {
        (self.players[player].x, self.players[player].y)
    }

    pub fn get_current_value(&self) -> T {
        let player = 0;
        self.get_player_value(player)
    }

    pub fn get_player_value(&self, player: usize) -> T {
        let x: usize = self.players[player].x.try_into().unwrap();
        let y: usize = self.players[player].y.try_into().unwrap();
        self.grid[y][x]
    }

    pub fn step(&mut self, direction: Direction) -> Option<T> {
        let player = 0;
        self.step_player(player, direction)
    }

    pub fn step_player(&mut self, player: usize, direction: Direction) -> Option<T> {
        let (step_x, step_y) = match direction {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::UpLeft => (-1, -1),
            Direction::UpRight => (1, -1),
            Direction::DownLeft => (-1, 1),
            Direction::DownRight => (1, 1),
        };

        let new_location = Player {
            x: self.players[player].x + step_x,
            y: self.players[player].y + step_y,
            id: self.players[player].id,
        };

        let x_max = self.width();
        let y_max = self.height();
        match new_location {
            _ if new_location.x == -1 => None,
            _ if new_location.y == -1 => None,
            _ if new_location.x == x_max => None,
            _ if new_location.y == y_max => None,
            _ => {
                self.players[player] = new_location;
                let x: usize = new_location.x.try_into().unwrap();
                let y: usize = new_location.y.try_into().unwrap();
                Some(self.grid[y][x])
            }
        }
    }

    pub fn is_nearby(&self, player1: usize, player2: usize) -> bool {
        let p1 = self.players[player1];
        let p2 = self.players[player2];

        if (p1.x - 1..=p1.x + 1).contains(&p2.x) && (p1.y - 1..=p1.y + 1).contains(&p2.y) {
            return true;
        }

        false
    }

    pub fn get_nearby_squares(&mut self, player: usize) -> Vec<Direction> {
        let mut values = vec![];
        let orig_point = self.get_player_location(player);
        for direction in Direction::straight_iterator() {
            if let Some(value) = self.step_player(player, direction) {
                values.push(direction);
            }
            self.set_player_location(player, orig_point.0, orig_point.1);
        }

        values
    }

    pub fn where_to_move(&self, start: usize, destination: usize) -> Direction {
        let s = self.players[start];
        let d = self.players[destination];

        match s {
            // Move straight
            s if s.x == d.x && s.y > d.y => Direction::Up,
            s if s.x == d.x && s.y < d.y => Direction::Down,
            s if s.y == d.y && s.x > d.x => Direction::Left,
            s if s.y == d.y && s.x < d.x => Direction::Right,
            s if s.x > d.x && s.y > d.y => Direction::UpLeft,
            s if s.x > d.x && s.y < d.y => Direction::DownLeft,
            s if s.x < d.x && s.y > d.y => Direction::UpRight,
            s if s.x < d.x && s.y < d.y => Direction::DownRight,
            _ => panic!("Fix me"),
        }
    }

    pub fn where_to_move_straight(&self, start: usize, destination: usize) -> Direction {
        let s = self.players[start];
        let d = self.players[destination];

        let dx = d.x - s.x;
        let dy = d.y - s.y;

        match s {
            // Move straight
            _ if dx >= 0 && i32::abs(dx) >= i32::abs(dy) => Direction::Right,
            _ if dx < 0 && i32::abs(dx) > i32::abs(dy) => Direction::Left,
            _ if dy >= 0 => Direction::Up,
            _ if dy < 0 => Direction::Down,
            _ => panic!("Fix me"),
        }
    }

    pub fn print_board_with_players(&mut self) {
        let orig: Vec<T> = self
            .players
            .iter()
            .enumerate()
            .map(|(i, _)| self.get_player_value(i).clone())
            .collect();
        for player in self.players.clone().iter().rev() {
            self.set_at(player.x, player.y, player.id);
        }
        log::debug!("{:#?}", self.grid);
        for (player, id) in zip(self.players.clone(), orig) {
            self.set_at(player.x, player.y, id);
        }
    }
}
