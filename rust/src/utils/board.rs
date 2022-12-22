use grid::*;
use rusttype::Point;
use std::fmt::Debug;
use std::iter::zip;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub type BoardPoint = Point<i32>;
pub type PlayerId = usize;
pub type BoardDefaultContext = u32;
pub const INVALID_PLAYER: usize = usize::MAX;

#[derive(Debug, Clone, Copy)]
struct Player<T>
where
    T: Clone + Copy + Debug + PartialEq + std::fmt::Display,
{
    point: BoardPoint,
    id: T,
    player_id: PlayerId,
}

// State about this current square in the gird
#[derive(Debug, Clone, Copy)]
struct State {
    step_count: u32, // Most optimized step count so far at this square
}

pub struct Board<T, Context>
where
    T: Clone + Copy + Debug + PartialEq + std::fmt::Display,
    Context: Clone + Debug,
{
    grid: Grid<T>,
    grid_state: Grid<State>,
    players: Vec<Player<T>>,
    walls: Vec<T>,
    players_are_walls: bool,
    wraparound: Vec<T>,
    wraparound_mode: bool,
    wraparound_custom_mode: bool,
    wraparound_custom: fn(&mut Context, BoardPoint, Direction) -> BoardPoint,
    context: Option<Context>,
}

#[derive(Debug, EnumIter, Clone, Copy, PartialEq, Eq, Hash)]
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

    pub fn opposite_direction(direction: Direction) -> Direction {
        match direction {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::UpLeft => Direction::DownRight,
            Direction::UpRight => Direction::DownLeft,
            Direction::DownLeft => Direction::UpRight,
            Direction::DownRight => Direction::UpLeft,
        }
    }
}

fn default_custom_wraparound<Context>(
    _context: &mut Context,
    _point: BoardPoint,
    _direction: Direction,
) -> BoardPoint {
    panic!("Custom wraparound not implemented")
}

impl<T, Context> Board<T, Context>
where
    T: Clone + Copy + Debug + PartialEq + std::fmt::Display,
    Context: Clone + Debug,
{
    pub fn new() -> Board<T, Context> {
        Board {
            grid: grid![],
            grid_state: grid![],
            players: vec![],
            walls: vec![],
            players_are_walls: false,
            wraparound: vec![],
            wraparound_mode: false,
            wraparound_custom_mode: false,
            wraparound_custom: default_custom_wraparound,
            context: None,
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

    pub fn push_front_row(&mut self, row: Vec<T>) {
        let len = row.len();
        self.grid.insert_row(0, row);

        for player in self.players.iter_mut() {
            player.point.y += 1;
        }

        // Push in empty state for this row
        let empty = vec![
            State {
                step_count: u32::MAX
            };
            len
        ];
        self.grid_state.insert_row(0, empty);
    }

    pub fn add_player(&mut self, point: BoardPoint, id: T) -> PlayerId {
        assert!(point.x < self.width() && point.y < self.height());
        self.players.push(Player {
            point,
            id,
            player_id: self.players.len(),
        });
        self.players.len() - 1
    }

    #[allow(dead_code)]
    pub fn get_players_len(&self) -> usize {
        self.players.len()
    }

    pub fn add_wall(&mut self, wall: T) {
        self.walls.push(wall);
    }

    pub fn set_players_as_walls(&mut self) {
        self.players_are_walls = true;
    }

    pub fn add_wraparound(&mut self, wraparound: T) {
        self.wraparound.push(wraparound);
    }

    pub fn set_wraparound_mode(&mut self) {
        self.wraparound_mode = true;
    }

    pub fn set_wraparound_custom_mode(
        &mut self,
        wraparound_custom: fn(&mut Context, BoardPoint, Direction) -> BoardPoint,
    ) {
        self.wraparound_custom_mode = true;
        self.wraparound_custom = wraparound_custom;
    }

    pub fn set_context(&mut self, context: &Context) {
        self.context = Some(context.clone());
    }

    pub fn get_context(&mut self) -> Context {
        self.context.as_ref().unwrap().clone()
    }

    pub fn player_is_here(&self, location: BoardPoint) -> bool {
        for player in &self.players {
            if player.point == location {
                return true;
            }
        }

        false
    }

    pub fn which_player_is_here(&self, location: BoardPoint) -> PlayerId {
        for player in &self.players {
            if player.point == location {
                return player.player_id;
            }
        }

        INVALID_PLAYER
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

    /// Get the grid value at this location.
    /// This does not include players.
    #[allow(dead_code)]
    pub fn get_at(&self, point: BoardPoint) -> T {
        let x_: usize = point.x as usize;
        let y_: usize = point.y as usize;
        self.grid[y_][x_]
    }

    pub fn get_at_with_player(&self, point: BoardPoint) -> T {
        let mut value = self.get_at(point);
        let player_id = self.which_player_is_here(point);
        if player_id != INVALID_PLAYER {
            value = self.players[player_id].id;
        }
        value
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

    // Search all players, finding the smallest y value
    pub fn get_player_minimum_height(&self) -> i32 {
        let mut min_player_y = self.height() - 1;
        for player in &self.players {
            min_player_y = std::cmp::min(min_player_y, player.point.y);
        }
        min_player_y
    }

    pub fn step(&mut self, direction: Direction) -> Option<T> {
        let player = 0;
        self.step_player(player, direction)
    }

    pub fn step_player(&mut self, player: PlayerId, direction: Direction) -> Option<T> {
        self.step_player_optionally(player, direction, true)
    }

    #[allow(dead_code)]
    pub fn can_step_player(&mut self, player: PlayerId, direction: Direction) -> bool {
        self.step_player_optionally(player, direction, false)
            .is_some()
    }

    fn step_player_optionally(
        &mut self,
        player: PlayerId,
        direction: Direction,
        do_step: bool,
    ) -> Option<T> {
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

        let start_location = self.players[player].point;
        let mut new_location = Player {
            point: BoardPoint {
                x: start_location.x + step_x,
                y: start_location.y + step_y,
            },
            id: self.players[player].id,
            player_id: self.players[player].player_id,
        };

        let x_max = self.width();
        let y_max = self.height();
        if self.wraparound_mode {
            // Wrap around when moving off grid
            if new_location.point.x == -1
                || new_location.point.y == -1
                || new_location.point.x == x_max
                || new_location.point.y == y_max
            {
                if self.wraparound_custom_mode {
                    new_location.point = (self.wraparound_custom)(
                        &mut self.context.as_mut().unwrap(),
                        start_location,
                        direction,
                    );
                } else {
                    match direction {
                        Direction::Up => new_location.point.y = y_max - 1,
                        Direction::Down => new_location.point.y = 0,
                        Direction::Left => new_location.point.x = x_max - 1,
                        Direction::Right => new_location.point.x = 0,
                        _ => panic!("Unsupported wrap around direction"),
                    }
                }
            }
        } else {
            // Fail this move if trying to move off grid
            match new_location {
                _ if new_location.point.x == -1 => return None,
                _ if new_location.point.y == -1 => return None,
                _ if new_location.point.x == x_max => return None,
                _ if new_location.point.y == y_max => return None,
                _ => (),
            }
        }

        let mut value = self.get_at(new_location.point);
        if self.wraparound.contains(&value) {
            if self.wraparound_custom_mode {
                new_location.point = (self.wraparound_custom)(
                    &mut self.context.as_mut().unwrap(),
                    start_location,
                    direction,
                );
                value = self.get_at(new_location.point);
            } else {
                value = self.step_player_wraparound(&mut new_location.point, direction);
            }
        }
        if self.walls.contains(&value) {
            return None;
        }
        if self.players_are_walls && self.player_is_here(new_location.point) {
            return None;
        }
        if do_step {
            self.players[player] = new_location;
        }
        Some(value)
    }

    /// When a wrap around happens, need to find the first non wrap around value
    fn step_player_wraparound(&self, location: &mut BoardPoint, direction: Direction) -> T {
        if direction == Direction::Up || direction == Direction::Down {
            // Search the column
            let (offset, start_y) = match direction {
                Direction::Up => (-1, self.height() - 1),
                Direction::Down => (1, 0),
                _ => panic!("Unexpected"),
            };
            location.y = start_y;
            loop {
                let value = self.grid[location.y as usize][location.x as usize];
                if !self.wraparound.contains(&value) {
                    return value;
                }
                location.y += offset;
            }
        } else if direction == Direction::Left || direction == Direction::Right {
            // Search the row
            let (offset, start_x) = match direction {
                Direction::Left => (-1, self.width() - 1),
                Direction::Right => (1, 0),
                _ => panic!("Unexpected"),
            };
            location.x = start_x;
            loop {
                let value = self.grid[location.y as usize][location.x as usize];
                if !self.wraparound.contains(&value) {
                    return value;
                }
                location.x += offset;
            }
        } else {
            panic!("Unsupported wrap around direction");
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
