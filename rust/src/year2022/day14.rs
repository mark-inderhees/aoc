// 2022 Day 14
// https://adventofcode.com/2022/day/14
// --- Day 14: Regolith Reservoir ---
// Sand dropping into a pit

use anyhow::Result;

use crate::puzzle::Puzzle;
use crate::utils::board::*;
use crate::utils::utils::*;

pub struct Day14 {
    grid: Board<char>,
    offset: BoardPoint,
    min: BoardPoint,
    max: BoardPoint,
}

fn drop_sand(day: &mut Day14) -> u32 {
    let origin = BoardPoint {
        x: 500 - day.offset.x,
        y: 0,
    };
    let directions = vec![Direction::Down, Direction::DownLeft, Direction::DownRight];
    let mut count = 0;

    loop {
        let id = day.grid.add_player(origin, 'o');
        let mut okay;
        loop {
            okay = false;
            for direction in directions.iter() {
                if day.grid.step_player(id, *direction).is_some() {
                    okay = true;
                    break;
                }
            }
            if !okay {
                // Player can no longer move
                day.grid.set_at(day.grid.get_player_location(id), 'o');
                break;
            }
        }
        if !okay {
            // Check if player fell off
            let location = day.grid.get_player_location(id);
            if location.x == 0
                || location.x == day.grid.width() - 1
                || location.y == day.grid.height() - 1
                || (location.x == origin.x && location.y == origin.y)
            {
                break;
            }
        }
        count += 1;
    }

    count
}

impl Puzzle for Day14 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut day = Day14 {
            grid: Board::new(),
            offset: BoardPoint { x: 0, y: 0 },
            min: BoardPoint { x: 1000, y: 1000 },
            max: BoardPoint { x: 0, y: 0 },
        };

        let test = input.lines().count() < 10;
        if test {
            // 11x11 grid
            // min, max: Point { x: 494, y: 4 }, Point { x: 503, y: 9 }
            day.offset.x = 400;
            let row = vec!['.'; 200];
            for _ in 0..13 {
                day.grid.push_row(row.clone());
            }
        } else {
            // min, max: Point { x: 483, y: 16 }, Point { x: 544, y: 164 }
            day.offset.x = 330;
            let row = vec!['.'; 340];
            for _ in 0..167 {
                day.grid.push_row(row.clone());
            }
        }

        // Add wall types
        day.grid.add_wall('#');
        day.grid.add_wall('o');
        day.grid.set_players_as_walls();

        for line in input.lines() {
            let mut started = false;
            let mut first = BoardPoint { x: 0, y: 0 };
            for point_str in line.split("->") {
                let values: Vec<i32> = get_vals(point_str);
                let second = BoardPoint {
                    x: values[0],
                    y: values[1],
                };
                if second.x > day.max.x {
                    day.max.x = second.x;
                }
                if second.x < day.min.x {
                    day.min.x = second.x;
                }
                if second.y > day.max.y {
                    day.max.y = second.y;
                }
                if second.y < day.min.y {
                    day.min.y = second.y;
                }
                if started {
                    // connect lines
                    let mut begin = BoardPoint {
                        x: first.x,
                        y: first.y,
                    };
                    let mut end = BoardPoint {
                        x: second.x,
                        y: second.y,
                    };
                    if first.x == second.x {
                        if first.y > second.y {
                            begin.y = second.y;
                            end.y = first.y;
                        }
                        for y in begin.y..=end.y {
                            day.grid.set_at(
                                BoardPoint {
                                    x: begin.x - day.offset.x,
                                    y,
                                },
                                '#',
                            );
                        }
                    } else if first.y == second.y {
                        if first.x > second.x {
                            begin.x = second.x;
                            end.x = first.x;
                        }
                        for x in begin.x..=end.x {
                            day.grid.set_at(
                                BoardPoint {
                                    x: x - day.offset.x,
                                    y: begin.y,
                                },
                                '#',
                            );
                        }
                    } else {
                        panic!("Unexpected line connection");
                    }
                }
                started = true;
                first.x = second.x;
                first.y = second.y;
            }
        }

        log::debug!("{:?}, {:?}", day.min, day.max);

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        let count = drop_sand(self);
        Ok(count.to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(24.to_string()),
            false => Some(614.to_string()),
        }
    }

    fn solve_part2(&mut self) -> Result<String> {
        for x in 0..self.grid.width() {
            self.grid.set_at(
                BoardPoint {
                    x,
                    y: self.max.y + 2,
                },
                '#',
            );
        }
        let count = drop_sand(self) + 1;
        Ok(count.to_string())
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(93.to_string()),
            false => Some(26170.to_string()),
        }
    }
}
