use anyhow::Result;
use grid::*;
use std::collections::HashMap;
use std::vec;

use crate::puzzle::Puzzle;
use crate::utils::board::*;

#[allow(unused_imports)]
use crate::utils::utils::*;

#[allow(unused_imports)]
use std::collections::VecDeque;

pub struct Day24 {
    grid: Board<char, BoardDefaultContext>,
    blizzards: Vec<Blizzard>,
    width: i32,
    height: i32,
}

#[derive(Default)]
struct Blizzard {
    id: PlayerId,
    start: BoardPoint,
    direction: Direction,
}

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

fn offset_location(location: &BoardPoint, direction: Direction) -> BoardPoint {
    let step_offsets = HashMap::from([
        (Direction::Up, BoardPoint { x: 0, y: -1 }),
        (Direction::Down, BoardPoint { x: 0, y: 1 }),
        (Direction::Left, BoardPoint { x: -1, y: 0 }),
        (Direction::Right, BoardPoint { x: 1, y: 0 }),
    ]);
    let offset = step_offsets[&direction];
    let new_location = BoardPoint {
        x: location.x + offset.x,
        y: location.y + offset.y,
    };

    new_location
}

fn search(day: &mut Day24, time: i32, start: BoardPoint, end: BoardPoint) -> i32 {
    struct Work {
        time: i32,
        location: BoardPoint,
        // path: Vec<String>,
    }
    let mut jobs = VecDeque::new();
    jobs.push_back(Work {
        time,
        location: start,
        // path: vec![],
    });
    let mut lowest_time = 1000; // kinda magic :)

    let mut lowest_grid: Grid<Vec<i32>> = grid![];
    let row = vec![vec![]; day.grid.width() as usize];
    for _ in 0..day.grid.height() {
        lowest_grid.push_row(row.clone());
    }
    let mut lcm = day.width;
    loop {
        if lcm % day.height == 0 {
            break;
        }
        lcm += day.width;
    }
    log::info!("LCM is {lcm}");

    let dir_to_string = HashMap::from([
        (Direction::Up, "U"),
        (Direction::Down, "D"),
        (Direction::Left, "L"),
        (Direction::Right, "R"),
    ]);

    while jobs.len() > 0 {
        let job = jobs.pop_front().unwrap();

        // Bail if this is a bad path
        if job.time >= lowest_time {
            continue;
        }

        // log::debug!("Path is {} at {}", job.path.join(""), job.time);

        if job.location == end {
            log::info!("Found end in {} steps from", job.time);
            lowest_time = job.time;
            continue;
        }

        // Bail if this is a really bad path
        let x_ = job.location.x as usize;
        let y_ = job.location.y as usize;
        let counts = &lowest_grid[y_][x_];
        let my_time = job.time % lcm;
        if counts.contains(&my_time) {
            continue;
        }
        lowest_grid[y_][x_].push(my_time);

        // Move the blizzards
        set_blizzards_location(day, job.time);

        // Set our location
        day.grid.set_player_location(0, job.location);

        // Figure out where we can move and do it
        for direction in Direction::straight_iterator() {
            if day.grid.can_step_player(0, direction) {
                // Schedule this work
                // let mut path = job.path.clone();
                // path.push(dir_to_string[&direction].to_string());
                jobs.push_back(Work {
                    time: job.time + 1,
                    location: offset_location(&job.location, direction),
                    // path,
                });
                log::trace!(
                    "Moving {:?} from {:?} at {}",
                    direction,
                    job.location,
                    job.time
                );
            }
        }

        // We could also do nothing if no blizzard here
        let mut wait = true;
        for blizzard_id in 1..day.grid.get_players_len() {
            let blizzard_location = day.grid.get_player_location(blizzard_id);
            if blizzard_location == job.location {
                wait = false;
                break;
            }
        }
        if wait {
            // let mut path = job.path.clone();
            // path.push("W".to_string());
            jobs.push_back(Work {
                time: job.time + 1,
                location: job.location,
                // path,
            });
            log::trace!("Waiting at {:?} at {}", job.location, job.time);
        }
    }

    lowest_time - 1
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

        let mut width = 0;
        for line in input.lines() {
            width = std::cmp::max(width, line.chars().count());
        }
        let height = input.lines().count();
        day.width = width as i32 - 2;
        day.height = height as i32 - 2;
        log::info!("Play area {} by {}", day.width, day.height);

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

        day.grid.add_player(BoardPoint { x: 1, y: 0 }, 'E');

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

        // day.grid.print_board_with_players_pretty();

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
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
