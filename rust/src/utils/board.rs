use grid::*;
use rusttype::Point;
use std::fmt::Debug;
use std::iter::zip;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub type BoardPoint = Point<i32>;
pub type PlayerId = usize;
pub const INVALID_PLAYER: usize = usize::MAX;

#[derive(Debug, Clone, Copy)]
struct Player<T>
where
    T: Clone + Copy + Debug + PartialEq + std::fmt::Display,
{
    point: BoardPoint,
    id: T,
}

// State about this current square in the gird
#[derive(Debug, Clone, Copy)]
struct State {
    step_count: u32, // Most optimized step count so far at this square
}

#[derive(Debug)]
pub struct Board<T>
where
    T: Clone + Copy + Debug + PartialEq + std::fmt::Display,
{
    grid: Grid<T>,
    grid_state: Grid<State>,
    players: Vec<Player<T>>,
    walls: Vec<T>,
    players_are_walls: bool,
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
    T: Clone + Copy + Debug + PartialEq + std::fmt::Display,
{
    pub fn new() -> Board<T> {
        Board {
            grid: grid![],
            grid_state: grid![],
            players: vec![],
            walls: vec![],
            players_are_walls: false,
        }
    }

    pub fn grid(&self) -> &Grid<T> {
        &self.grid
    }

    pub fn push_row(&mut self, row: Vec<T>) {
        let len = row.len();
        self.grid.push_row(row);

        // Push in empty state for this row
        let empty = vec![
            State {
                step_count: u32::MAX
            };
            len
        ];
        self.grid_state.push_row(empty);
    }

    pub fn add_player(&mut self, point: BoardPoint, id: T) -> PlayerId {
        self.players.push(Player { point, id });
        self.players.len() - 1
    }

    pub fn add_wall(&mut self, wall: T) {
        self.walls.push(wall);
    }

    pub fn set_players_as_walls(&mut self) {
        self.players_are_walls = true;
    }

    pub fn player_is_here(&self, location: BoardPoint) -> bool {
        for player in &self.players {
            if player.point == location {
                return true;
            }
        }

        false
    }

    pub fn width(&self) -> i32 {
        self.grid.cols() as i32
    }

    pub fn height(&self) -> i32 {
        self.grid.rows() as i32
    }

    pub fn set_at(&mut self, point: BoardPoint, value: T) {
        let x_: usize = point.x as usize;
        let y_: usize = point.y as usize;
        self.grid[y_][x_] = value;
    }

    #[allow(dead_code)]
    pub fn get_at(&self, point: BoardPoint) -> T {
        let x_: usize = point.x as usize;
        let y_: usize = point.y as usize;
        self.grid[y_][x_]
    }

    pub fn set_location(&mut self, point: BoardPoint) {
        let player = 0;
        self.set_player_location(player, point);
    }

    pub fn set_player_location(&mut self, player: PlayerId, point: BoardPoint) {
        self.players[player].point = point;
    }

    pub fn get_player_location(&self, player: PlayerId) -> BoardPoint {
        self.players[player].point
    }

    pub fn get_current_value(&self) -> T {
        let player = 0;
        self.get_player_value(player)
    }

    pub fn get_player_value(&self, player: PlayerId) -> T {
        let x: usize = self.players[player].point.x as usize;
        let y: usize = self.players[player].point.y as usize;
        self.grid[y][x]
    }

    pub fn step(&mut self, direction: Direction) -> Option<T> {
        let player = 0;
        self.step_player(player, direction)
    }

    pub fn step_player(&mut self, player: PlayerId, direction: Direction) -> Option<T> {
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
            point: BoardPoint {
                x: self.players[player].point.x + step_x,
                y: self.players[player].point.y + step_y,
            },
            id: self.players[player].id,
        };

        let x_max = self.width();
        let y_max = self.height();
        match new_location {
            _ if new_location.point.x == -1 => None,
            _ if new_location.point.y == -1 => None,
            _ if new_location.point.x == x_max => None,
            _ if new_location.point.y == y_max => None,
            _ => {
                let x: usize = new_location.point.x as usize;
                let y: usize = new_location.point.y as usize;
                let value = self.grid[y][x];
                if self.walls.contains(&value) {
                    return None;
                }
                if self.players_are_walls && self.player_is_here(new_location.point) {
                    return None;
                }
                self.players[player] = new_location;
                Some(value)
            }
        }
    }

    pub fn is_nearby(&self, player1: PlayerId, player2: PlayerId) -> bool {
        let p1 = self.players[player1];
        let p2 = self.players[player2];

        if (p1.point.x - 1..=p1.point.x + 1).contains(&p2.point.x)
            && (p1.point.y - 1..=p1.point.y + 1).contains(&p2.point.y)
        {
            return true;
        }

        false
    }

    #[allow(dead_code)]
    pub fn get_nearby_squares(&mut self, player: PlayerId) -> Vec<Direction> {
        let mut values = vec![];
        let orig_point = self.get_player_location(player);
        for direction in Direction::straight_iterator() {
            if let Some(_value) = self.step_player(player, direction) {
                values.push(direction);
            }
            self.set_player_location(player, orig_point);
        }

        values
    }

    pub fn where_to_move(&self, start: PlayerId, destination: PlayerId) -> Direction {
        let s = self.players[start];
        let d = self.players[destination];

        match s {
            // Move straight
            s if s.point.x == d.point.x && s.point.y > d.point.y => Direction::Up,
            s if s.point.x == d.point.x && s.point.y < d.point.y => Direction::Down,
            s if s.point.y == d.point.y && s.point.x > d.point.x => Direction::Left,
            s if s.point.y == d.point.y && s.point.x < d.point.x => Direction::Right,
            s if s.point.x > d.point.x && s.point.y > d.point.y => Direction::UpLeft,
            s if s.point.x > d.point.x && s.point.y < d.point.y => Direction::DownLeft,
            s if s.point.x < d.point.x && s.point.y > d.point.y => Direction::UpRight,
            s if s.point.x < d.point.x && s.point.y < d.point.y => Direction::DownRight,
            _ => panic!("Fix me"),
        }
    }

    #[allow(dead_code)]
    pub fn where_to_move_straight(&self, start: PlayerId, destination: PlayerId) -> Direction {
        let s = self.players[start];
        let d = self.players[destination];

        let dx = d.point.x - s.point.x;
        let dy = d.point.y - s.point.y;

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
        self.print_board_with_players_helper(false);
    }

    #[allow(dead_code)]
    pub fn print_board_with_players_pretty(&mut self) {
        self.print_board_with_players_helper(true);
    }

    fn print_board_with_players_helper(&mut self, pretty: bool) {
        let orig: Vec<T> = self
            .players
            .iter()
            .enumerate()
            .map(|(i, _)| self.get_player_value(i).clone())
            .collect();
        for player in self.players.clone().iter().rev() {
            self.set_at(player.point, player.id);
        }
        if pretty {
            for row in 0..self.height() as usize {
                for t in self.grid.iter_row(row) {
                    print!("{}", t);
                }
                println!("");
            }
        } else {
            log::debug!("{:#?}", self.grid);
        }
        for (player, id) in zip(self.players.clone(), orig) {
            self.set_at(player.point, id);
        }
    }

    /// Find the shortest path from a point to a player
    /// AKA find the quickest path or route, find the fastest path or route
    pub fn find_shortest_path(
        &mut self,
        from_player: PlayerId,
        taget_player: PlayerId,
        valid_move: fn(T, T) -> bool,
    ) -> u32 {
        struct PathWork {
            location: BoardPoint,
            count: u32,
        }
        let mut jobs = vec![PathWork {
            location: self.get_player_location(from_player),
            count: 0,
        }];
        let mut shortest_path = u32::MAX;
        let taget = self.get_player_location(taget_player);

        while jobs.len() > 0 {
            let job = jobs.pop().unwrap();

            // Check if this count is already too long
            if job.count >= shortest_path {
                continue;
            }

            // Check if we've ever been here at a more optimized path
            let x: usize = job.location.x as usize;
            let y: usize = job.location.y as usize;
            let step_count = self.grid_state[y][x].step_count;
            if job.count >= step_count {
                continue;
            }
            self.grid_state[y][x].step_count = job.count;

            // Try all new locations
            for direction in Direction::straight_iterator() {
                // Force current location
                self.set_player_location(from_player, job.location);
                let my_char = self.get_player_value(from_player);

                // Try this move
                if let Some(near_char) = self.step_player(from_player, direction) {
                    let new_location = self.get_player_location(from_player);

                    // See if we are allowed to move here
                    if valid_move(my_char, near_char) {
                        // Check if we are done
                        if new_location.x == taget.x && new_location.y == taget.y {
                            log::debug!("THIS IS THE END = {}", job.count);
                            let final_count = job.count + 1;
                            if final_count < shortest_path {
                                shortest_path = final_count;
                            }
                            continue;
                        }

                        // We can move, so do it!
                        jobs.push(PathWork {
                            location: new_location,
                            count: job.count + 1,
                        });
                    }
                }
            }
        }

        shortest_path
    }

    #[allow(dead_code)]
    pub fn draw_manhattan_radius(&mut self, point: BoardPoint, dist: i32, value: T) {
        let mut x_offset = 0;
        let y_min = point.y - dist;
        let y_max = point.y + dist;
        for y in y_min..point.y {
            for x in (point.x - x_offset)..=(point.x + x_offset) {
                self.set_at(BoardPoint { x, y }, value);
            }
            x_offset += 1;
        }

        for y in point.y..=y_max {
            for x in (point.x - x_offset)..=(point.x + x_offset) {
                self.set_at(BoardPoint { x, y }, value);
            }
            x_offset -= 1;
        }
    }
}
