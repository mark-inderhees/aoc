use grid::*;
use rusttype::Point;
use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;
use std::iter::zip;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

/// A 2d game board for players to navigate.
pub struct Board<T>
where
    T: Clone + Copy + Debug + PartialEq + std::fmt::Display,
{
    /// The grid itself
    grid: Grid<T>,
    grid_state: Grid<State>,

    /// Players on the grid
    players: Vec<Player<T>>,

    /// Types of walls on the grid
    walls: Vec<T>,
    players_are_walls: bool,

    /// Wraparound supports when reach certain grid value, move to other side
    /// of row or column
    wraparound: Vec<T>,
    wraparound_mode: bool,
}

/// Simple way to use Point
pub type BoardPoint = Point<i32>;

/// Unique id for each player on grid
pub type PlayerId = usize;

/// Supported directions on the board
#[derive(Debug, EnumIter, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Direction {
    #[default]
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
    /// Iterate through all directions
    pub fn iterator() -> DirectionIter {
        Direction::iter()
    }

    /// Iterate through just straight directions
    pub fn straight_iterator() -> Vec<Direction> {
        vec![
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
    }

    /// Given a direction, give the opposite direction
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

impl ToString for Direction {
    /// Impl ToString trait to pretty print direction dir in small text format
    fn to_string(&self) -> String {
        let dir_to_string = match self {
            Direction::Up => "U",
            Direction::Down => "D",
            Direction::Left => "L",
            Direction::Right => "R",
            Direction::UpLeft => "UL",
            Direction::UpRight => "UR",
            Direction::DownLeft => "DL",
            Direction::DownRight => "DR",
        };
        dir_to_string.to_string()
    }
}

impl<T> Board<T>
where
    T: Clone + Copy + Debug + PartialEq + std::fmt::Display,
{
    /// Create a new, empty board.
    pub fn new() -> Board<T> {
        Board {
            grid: grid![],
            grid_state: grid![],
            players: vec![],
            walls: vec![],
            players_are_walls: false,
            wraparound: vec![],
            wraparound_mode: false,
        }
    }

    /// Return immutable ref to grid itself.
    pub fn grid(&self) -> &Grid<T> {
        &self.grid
    }

    /// Add a new row to the board
    pub fn push_row(&mut self, row: Vec<T>) {
        let len = row.len();
        self.grid.push_row(row);

        // Push in empty state for this row
        let empty = vec![
            State {
                step_count: u32::MAX,
                players_here: HashMap::new(),
            };
            len
        ];
        self.grid_state.push_row(empty);
    }

    /// Add a row to the top of the board. Useful for things like tetris that grow up.
    pub fn push_front_row(&mut self, row: Vec<T>) {
        let len = row.len();
        self.grid.insert_row(0, row);

        for player in self.players.iter_mut() {
            player.point.y += 1;
        }

        // Push in empty state for this row
        let empty = vec![
            State {
                step_count: u32::MAX,
                players_here: HashMap::new(),
            };
            len
        ];
        self.grid_state.insert_row(0, empty);
    }

    /// Add in a new player to the board.
    pub fn add_player(&mut self, point: BoardPoint, id: T) -> PlayerId {
        assert!(point.x < self.width() && point.y < self.height());
        let player_id = self.players.len();
        self.players.push(Player {
            point,
            id,
            player_id,
            visible: true,
        });

        // Update state
        let x_: usize = point.x as usize;
        let y_: usize = point.y as usize;
        self.grid_state[y_][x_].players_here.insert(player_id, true);

        player_id
    }

    /// Turn a specific board value into players.
    /// Convert the value on the board to a specific background.
    /// Useful when input has a whole bunch of preset players that need to start moving.
    pub fn add_players_from_value(&mut self, player_value: T, background_value: T) {
        for y in 0..self.height() {
            for x in 0..self.width() {
                let location = BoardPoint { x, y };
                let value = self.value_at(location);
                if value == player_value {
                    self.add_player(location, player_value);
                    self.set_at(location, background_value);
                }
            }
        }
    }

    /// Players can be visible. Useful for when players jump on and off board.
    pub fn set_player_visible(&mut self, id: PlayerId, visible: bool) {
        self.players[id].visible = visible;
    }

    /// How many players there are.
    #[allow(dead_code)]
    pub fn players_len(&self) -> usize {
        self.players.len()
    }

    /// Add a new wall type.
    pub fn add_wall(&mut self, wall: T) {
        self.walls.push(wall);
    }

    /// Prevent player collisions by making them walls. Else can flow over each other.
    pub fn set_players_as_walls(&mut self) {
        self.players_are_walls = true;
    }

    /// Get the grid state at this location.
    fn state(&self, location: BoardPoint) -> State {
        let x_: usize = location.x as usize;
        let y_: usize = location.y as usize;
        self.grid_state[y_][x_].clone()
    }

    /// Wrap around mode will move to the other end of the col or row when this
    /// type is encountered.
    pub fn add_wraparound(&mut self, wraparound: T) {
        self.wraparound.push(wraparound);
    }

    /// Wrap around mode will move to the other end of the col or row when this
    /// type is encountered.
    pub fn set_wraparound_mode(&mut self) {
        self.wraparound_mode = true;
    }

    /// Is there a player at this location?
    pub fn is_player_here(&self, location: BoardPoint) -> bool {
        let state = self.state(location);
        state.players_here.len() > 0
    }

    /// Return the first player found at this location.
    pub fn which_player_is_here(&self, location: BoardPoint) -> Option<PlayerId> {
        let state = self.state(location);
        if state.players_here.len() > 0 {
            let player_id = state.players_here.keys().collect::<Vec<&PlayerId>>()[0];
            return Some(*player_id);
        }

        None
    }

    /// Width of grid.
    pub fn width(&self) -> i32 {
        self.grid.cols() as i32
    }

    /// Height of grid.
    pub fn height(&self) -> i32 {
        self.grid.rows() as i32
    }

    /// Set the board value here.
    pub fn set_at(&mut self, point: BoardPoint, value: T) {
        let x_: usize = point.x as usize;
        let y_: usize = point.y as usize;
        self.grid[y_][x_] = value;
    }

    /// Get the grid value at this location.
    /// This does not include players.
    #[allow(dead_code)]
    pub fn value_at(&self, point: BoardPoint) -> T {
        let x_: usize = point.x as usize;
        let y_: usize = point.y as usize;
        self.grid[y_][x_]
    }

    /// Get the grid value at this location. Or if a player is here, give the player value.
    pub fn value_at_with_player(&self, point: BoardPoint) -> T {
        let mut value = self.value_at(point);
        if let Some(player_id) = self.which_player_is_here(point) {
            value = self.players[player_id].id;
        }
        value
    }

    /// Set the location for a player.
    pub fn set_player_location(&mut self, player: PlayerId, point: BoardPoint) {
        // Update grid state
        let old_point = self.players[player].point;
        let old_x: usize = old_point.x as usize;
        let old_y: usize = old_point.y as usize;
        self.grid_state[old_y][old_x].players_here.remove(&player);
        let new_x: usize = point.x as usize;
        let new_y: usize = point.y as usize;
        self.grid_state[new_y][new_x]
            .players_here
            .insert(player, true);

        self.players[player].point = point;
    }

    /// Get the location of a player.
    pub fn player_location(&self, player: PlayerId) -> BoardPoint {
        self.players[player].point
    }

    /// Get the grid value where this player is.
    pub fn player_value(&self, player: PlayerId) -> T {
        let x: usize = self.players[player].point.x as usize;
        let y: usize = self.players[player].point.y as usize;
        self.grid[y][x]
    }

    // Search all players, finding the smallest y value.
    pub fn player_minimum_height(&self) -> i32 {
        let mut min_player_y = self.height() - 1;
        for player in &self.players {
            min_player_y = std::cmp::min(min_player_y, player.point.y);
        }
        min_player_y
    }

    /// Step a player in this direction.
    pub fn step_player(&mut self, player: PlayerId, direction: Direction) -> Option<T> {
        self.step_player_optionally(player, direction, true)
    }

    /// Can this player step in this direction? Does not move the player.
    #[allow(dead_code)]
    pub fn can_step_player(&mut self, player: PlayerId, direction: Direction) -> bool {
        self.step_player_optionally(player, direction, false)
            .is_some()
    }

    /// Is there a wall at this location?
    pub fn is_wall_here(&self, point: BoardPoint) -> bool {
        let value = self.value_at(point);
        if self.walls.contains(&value) {
            return true;
        }
        if self.players_are_walls && self.is_player_here(point) {
            return true;
        }
        false
    }

    /// From a given location, move one step in a certain direction to give a new location
    pub fn new_location_from_direction(
        &self,
        location: &BoardPoint,
        direction: Direction,
    ) -> BoardPoint {
        let offset = match direction {
            Direction::Up => BoardPoint { x: 0, y: -1 },
            Direction::Down => BoardPoint { x: 0, y: 1 },
            Direction::Left => BoardPoint { x: -1, y: 0 },
            Direction::Right => BoardPoint { x: 1, y: 0 },
            Direction::UpLeft => BoardPoint { x: -1, y: -1 },
            Direction::UpRight => BoardPoint { x: 1, y: -1 },
            Direction::DownLeft => BoardPoint { x: -1, y: 1 },
            Direction::DownRight => BoardPoint { x: 1, y: 1 },
        };
        let new_location = BoardPoint {
            x: location.x + offset.x,
            y: location.y + offset.y,
        };

        new_location
    }

    fn is_valid_location(&self, location: &BoardPoint) -> bool {
        if location.x < 0
            || location.y < 0
            || location.x >= self.width()
            || location.y >= self.height()
        {
            return false;
        }
        true
    }

    /// Move a player one step in a direction. Or check if it's possible to do the move.
    /// If possible, returns the grid value at the new location. If requested, player is actually moved.
    /// If not possible, returns None. Happens when walls are hit or moving off grid.
    fn step_player_optionally(
        &mut self,
        player: PlayerId,
        direction: Direction,
        do_step: bool,
    ) -> Option<T> {
        let start_location = self.players[player].point;
        let mut new_location = self.new_location_from_direction(&start_location, direction);

        // Check for moving off grid case
        if !self.is_valid_location(&new_location) {
            if self.wraparound_mode {
                // Wrap around to other side
                match direction {
                    Direction::Up => new_location.y = self.height() - 1,
                    Direction::Down => new_location.y = 0,
                    Direction::Left => new_location.x = self.width() - 1,
                    Direction::Right => new_location.x = 0,
                    _ => panic!("Unsupported wrap around direction"),
                }
            } else {
                // Fail this move if trying to move off grid
                return None;
            }
        }

        let mut value = self.value_at(new_location);

        if self.wraparound.contains(&value) {
            // Special wrap around square
            value = self.step_player_wraparound(&mut new_location, direction);
        }

        // Check for walls
        if self.walls.contains(&value) {
            return None;
        }
        if self.players_are_walls && self.is_player_here(new_location) {
            return None;
        }

        if do_step {
            // Actually move the player
            self.set_player_location(player, new_location);
        }

        Some(value)
    }

    /// When a wrap around happens, need to find the first non wrap around
    /// value on the other side of the row or column.
    fn step_player_wraparound(&self, location: &mut BoardPoint, direction: Direction) -> T {
        let (offset_x, offset_y, start_x, start_y) = match direction {
            // Search the column
            Direction::Up => (0, -1, location.x, self.height() - 1),
            Direction::Down => (0, 1, location.x, 0),

            // Search the row
            Direction::Left => (-1, 0, self.width() - 1, location.y),
            Direction::Right => (1, 0, 0, location.y),
            _ => panic!("Unexpected"),
        };

        // Search the row or column
        location.x = start_x;
        location.y = start_y;
        loop {
            let value = self.grid[location.y as usize][location.x as usize];
            if !self.wraparound.contains(&value) {
                return value;
            }
            location.x += offset_x;
            location.y += offset_y;
        }
    }

    /// Are these two players near each other? Diagonals are searched.
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

    /// Is any player nearby this player? Diagonals are searched.
    pub fn is_any_player_nearby(&mut self, player: PlayerId) -> bool {
        let my_location = self.players[player].point;
        for direction in Direction::iter() {
            let test_location = self.new_location_from_direction(&my_location, direction);
            if self.is_valid_location(&test_location) {
                if self.is_player_here(test_location) {
                    return true;
                }
            }
        }

        // Also test this location
        let state = self.state(my_location);
        if state.players_here.len() > 1 {
            return true;
        }

        false
    }

    /// Get all of the directions this player can move. A list of valid
    /// directions is returned. Only straight steps are checked.
    #[allow(dead_code)]
    pub fn directions_player_can_move(&mut self, player: PlayerId) -> Vec<Direction> {
        let mut values = vec![];
        let orig_point = self.player_location(player);
        for direction in Direction::straight_iterator() {
            if let Some(_value) = self.step_player(player, direction) {
                values.push(direction);
            }
            self.set_player_location(player, orig_point);
        }

        values
    }

    /// Get the value of nearby squares in all directions including diagonal, near a player
    #[allow(dead_code)]
    pub fn nearby_values(&mut self, player: PlayerId) -> Vec<T> {
        let mut values = vec![];
        let orig_point = self.player_location(player);
        for direction in Direction::iter() {
            if let Some(value) = self.step_player(player, direction) {
                values.push(value);
            }
            self.set_player_location(player, orig_point);
        }

        values
    }

    /// Get the value of nearby squares in all directions including diagonal, near a point
    pub fn values_near_point(&mut self, point: BoardPoint) -> Vec<T> {
        let mut values = vec![];
        for direction in Direction::iter() {
            let new_point = self.new_location_from_direction(&point, direction);
            if self.is_valid_location(&new_point) {
                values.push(self.value_at(new_point));
            }
        }

        values
    }

    /// Return how a player should move to move towards a destination.
    /// Diagonals moves are included.
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

    /// Return how a player should move to move towards a destination. Only
    /// straight moves allowed.
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

    /// Print the board with players.
    pub fn print_board_with_players(&mut self) {
        self.print_board_with_players_helper(false);
    }

    /// Print the boards with players using println.
    #[allow(dead_code)]
    pub fn print_board_with_players_pretty(&mut self) {
        self.print_board_with_players_helper(true);
    }

    /// Print the board with players.
    fn print_board_with_players_helper(&mut self, pretty: bool) {
        let orig: Vec<T> = self
            .players
            .iter()
            .enumerate()
            .map(|(i, _)| self.player_value(i).clone())
            .collect();
        for player in self.players.clone().iter().rev() {
            if player.visible {
                self.set_at(player.point, player.id);
            }
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
        let mut jobs = VecDeque::new();

        // Start at the current location
        jobs.push_front(PathWork {
            location: self.player_location(from_player),
            count: 0,
        });

        let mut shortest_path = u32::MAX; // Best answer so far
        let taget = self.player_location(taget_player);

        while jobs.len() > 0 {
            let job = jobs.pop_front().unwrap();

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
                let my_char = self.player_value(from_player);

                // Try this move
                if let Some(near_char) = self.step_player(from_player, direction) {
                    let new_location = self.player_location(from_player);

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
                        jobs.push_back(PathWork {
                            location: new_location,
                            count: job.count + 1,
                        });
                    }
                }
            }
        }

        shortest_path
    }

    /// Draw a manhattan circle on the board.
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

/// Internal only.
/// A player that is on the board.
#[derive(Debug, Clone, Copy)]
struct Player<T>
where
    T: Clone + Copy + Debug + PartialEq + std::fmt::Display,
{
    point: BoardPoint,

    /// How the player looks when the board is printed
    id: T,

    #[allow(dead_code)]
    player_id: PlayerId,

    visible: bool,
}

/// Internal only.
/// State about this current square in the gird
#[derive(Debug, Clone)]
struct State {
    /// Most optimized step count so far at this square
    step_count: u32,

    /// Which players are in this square, hashmap used for easy remove, value is nothing
    players_here: HashMap<PlayerId, bool>,
}
