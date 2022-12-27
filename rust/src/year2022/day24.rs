// 2022 Day 24
// https://adventofcode.com/2022/day/24
// --- Day 24: Blizzard Basin ---
// There's a blizzard! Used a game board and find shortest path.
// Instead of using moves, need to place all blizzard pieces based on time.
// Instead of a search, just track all possible locations based on time.

use anyhow::Result;
use std::collections::HashMap;
use std::vec;

use crate::puzzle::Puzzle;
use crate::utils::board::*;

pub struct Day24 {
    grid: Board<char>,
    blizzards: Vec<Blizzard>,

    /// width of blizzard movable area
    width: i32,

    /// height of blizzard movable area
    height: i32,
}

#[derive(Default)]
struct Blizzard {
    id: PlayerId,
    start: BoardPoint,
    direction: Direction,
}

/// Blizzards move based on current time, they have a predictable pattern
fn set_blizzards_location(day: &mut Day24, time: i32) {
    let step_offsets = HashMap::from([
        (Direction::Up, BoardPoint { x: 0, y: -1 }),
        (Direction::Down, BoardPoint { x: 0, y: 1 }),
        (Direction::Left, BoardPoint { x: -1, y: 0 }),
        (Direction::Right, BoardPoint { x: 1, y: 0 }),
    ]);

    for blizzard in &day.blizzards {
        let mut location = blizzard.start;
        let offset = step_offsets[&blizzard.direction];
        location.x = (location.x - 1 + time * offset.x).rem_euclid(day.width) + 1;
        location.y = (location.y - 1 + time * offset.y).rem_euclid(day.height) + 1;
        day.grid.set_player_location(blizzard.id, location)
    }
}

/// Look for the best path through the blizzard. Iterate through each time
/// instance and track all possible locations for that time instance. Until end
/// is found.
fn search(day: &mut Day24, time_input: i32, start: BoardPoint, end: BoardPoint) -> i32 {
    let mut time = time_input;
    let mut previous_locations = vec![start];

    // Keep looping until end is found
    while !previous_locations.contains(&end) {
        // Move the blizzards
        set_blizzards_location(day, time);

        // Based on all previous locations, find all possible new locations
        let mut new_locations = vec![];
        for location in previous_locations.iter() {
            // Set our location
            day.grid.set_player_location(0, *location);

            // Figure out where we can move and add it to the list
            for direction in Direction::straight_iterator() {
                if day.grid.can_step_player(0, direction) {
                    new_locations.push(day.grid.new_location_from_direction(location, direction));
                }
            }

            // We could also do nothing if no blizzard here
            let mut wait = true;
            for blizzard_id in 1..day.grid.players_len() {
                let blizzard_location = day.grid.player_location(blizzard_id);
                if blizzard_location == *location {
                    wait = false;
                    break;
                }
            }
            if wait {
                new_locations.push(*location);
            }

            // Remove duplicates
            new_locations.sort();
            new_locations.dedup();
        }

        log::debug!("Round {time} done");
        previous_locations = new_locations;
        time += 1;
    }

    time - 1
}

impl Puzzle for Day24 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut day = Day24 {
            grid: Board::new(),
            blizzards: vec![],
            width: 0,
            height: 0,
        };

        // First find grid size and blizzard grid area
        let mut width = 0;
        for line in input.lines() {
            width = std::cmp::max(width, line.chars().count());
        }
        let height = input.lines().count();
        day.width = width as i32 - 2;
        day.height = height as i32 - 2;
        log::info!("Play area {} by {}", day.width, day.height);

        // Draw the map. We know how it looks, start at top left, end at bottom right.
        let mut row1 = vec!['#'; width];
        row1[1] = '.';
        day.grid.push_row(row1.clone());
        let mut row_mid = vec!['.'; width];
        row_mid[0] = '#';
        row_mid[width - 1] = '#';
        for _ in 2..height {
            day.grid.push_row(row_mid.clone());
        }
        row1[1] = '#';
        row1[width - 2] = '.';
        day.grid.push_row(row1);
        day.grid.add_wall('#');
        day.grid.set_players_as_walls();

        // Add our main player, the expedition
        day.grid.add_player(BoardPoint { x: 1, y: 0 }, 'E');

        // Scan the input for the blizzrds and add players for each one
        for (y, line) in input.lines().enumerate() {
            for (x, char) in line.char_indices() {
                let mut blizzard = match char {
                    '^' => Blizzard {
                        direction: Direction::Up,
                        ..Default::default()
                    },
                    'v' => Blizzard {
                        direction: Direction::Down,
                        ..Default::default()
                    },
                    '<' => Blizzard {
                        direction: Direction::Left,
                        ..Default::default()
                    },
                    '>' => Blizzard {
                        direction: Direction::Right,
                        ..Default::default()
                    },
                    _ => continue,
                };
                blizzard.start = BoardPoint {
                    x: x as i32,
                    y: y as i32,
                };
                blizzard.id = day.grid.add_player(blizzard.start, char);
                day.blizzards.push(blizzard);
            }
        }

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        // Find how long it takes to go from start to end
        let start = BoardPoint { x: 1, y: 0 };
        let end = BoardPoint {
            x: self.grid.width() - 2,
            y: self.grid.height() - 1,
        };
        let answer = search(self, 1, start, end);
        Ok(answer.to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(18.to_string()),
            false => Some(230.to_string()),
        }
    }

    fn solve_part2(&mut self) -> Result<String> {
        // Go from start -> end -> start -> end
        let start = BoardPoint { x: 1, y: 0 };
        let end = BoardPoint {
            x: self.grid.width() - 2,
            y: self.grid.height() - 1,
        };
        let mut time = search(self, 1, start, end);
        time = search(self, time, end, start);
        time = search(self, time, start, end);
        Ok(time.to_string())
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(54.to_string()),
            false => Some(713.to_string()),
        }
    }
}
