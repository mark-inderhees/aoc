use anyhow::Result;

use crate::puzzle::Puzzle;
use crate::utils::board::*;

#[allow(unused_imports)]
use crate::utils::utils::*;

pub struct Day14 {
    grid: Board<char>,
    offset: BoardPoint,
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
                break;
            }
        }
        if !okay {
            // Check if player fell off
            let location = day.grid.get_player_location(id);
            if location.x == 0
                || location.x == day.grid.width() - 1
                || location.y == day.grid.height() - 1
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
        };

        let test = input.lines().count() < 10;
        if test {
            // 11x11 grid
            // min, max: Point { x: 494, y: 4 }, Point { x: 503, y: 9 }
            day.offset.x = 493;
            let row = vec!['.'; 11];
            for _ in 0..11 {
                day.grid.push_row(row.clone());
            }
        } else {
            // min, max: Point { x: 483, y: 16 }, Point { x: 544, y: 164 }
            day.offset.x = 480;
            let row = vec!['.'; 100];
            for _ in 0..200 {
                day.grid.push_row(row.clone());
            }
        }

        // Add wall types
        day.grid.add_wall('#');
        day.grid.set_players_as_walls();

        let mut p_max = BoardPoint { x: 0, y: 0 };
        let mut p_min = BoardPoint { x: 1000, y: 1000 };

        for line in input.lines() {
            let mut started = false;
            let mut first = BoardPoint { x: 0, y: 0 };
            for point_str in line.split("->") {
                let values: Vec<i32> = get_vals(point_str);
                let second = BoardPoint {
                    x: values[0],
                    y: values[1],
                };
                if second.x > p_max.x {
                    p_max.x = second.x;
                }
                if second.x < p_min.x {
                    p_min.x = second.x;
                }
                if second.y > p_max.y {
                    p_max.y = second.y;
                }
                if second.y < p_min.y {
                    p_min.y = second.y;
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

        println!("{:?}, {:?}", p_min, p_max);
        day.grid.print_board_with_players_pretty();

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        let count = drop_sand(self);
        println!("{count}");
        self.grid.print_board_with_players_pretty();

        Ok(count.to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(24.to_string()),
            false => Some(614.to_string()),
        }
    }

    fn solve_part2(&mut self) -> Result<String> {
        Ok("to do".to_string())
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => None,
            false => None,
        }
    }
}
