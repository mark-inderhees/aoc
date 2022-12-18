use anyhow::Result;

use crate::puzzle::Puzzle;

#[allow(unused_imports)]
use crate::utils::utils::*;

pub struct Day18 {
    box_3d: Vec<Vec<Vec<bool>>>,
    size: usize,
}

impl Puzzle for Day18 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let size = 30;
        let mut day = Day18 {
            box_3d: vec![vec![vec![false; size]; size]; size],
            size: 0,
        };

        let mut max = 0;
        let mut count = 0;

        for line in input.lines() {
            let values: Vec<usize> = get_vals(line);
            let x = values[0];
            let y = values[1];
            let z = values[2];
            max = std::cmp::max(max, x);
            max = std::cmp::max(max, y);
            max = std::cmp::max(max, z);
            assert!(!day.box_3d[x][y][z]);
            day.box_3d[x][y][z] = true;
            count += 1;
        }

        day.size = max + 5;
        log::debug!("Max is {max}, count is {count}");

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        let mut count = 0;
        let mut lava = 0;
        for x in 0..self.size {
            for y in 0..self.size {
                for z in 0..self.size {
                    if self.box_3d[x][y][z] {
                        lava += 1;
                        for dx in [-1i32, 1] {
                            let x2 = (x as i32) + dx;
                            if x2 < 0 {
                                count += 1;
                                continue;
                            }
                            let x3 = x2 as usize;
                            if !self.box_3d[x3][y][z] {
                                count += 1;
                            }
                        }
                        for dy in [-1i32, 1] {
                            let y2 = (y as i32) + dy;
                            if y2 < 0 {
                                count += 1;
                                continue;
                            }
                            let y3 = y2 as usize;
                            if !self.box_3d[x][y3][z] {
                                count += 1;
                            }
                        }
                        for dz in [-1i32, 1] {
                            let z2 = (z as i32) + dz;
                            if z2 < 0 {
                                count += 1;
                                continue;
                            }
                            let z3 = z2 as usize;
                            if !self.box_3d[x][y][z3] {
                                count += 1;
                            }
                        }
                    }
                }
            }
        }

        log::debug!("Found {lava} lava");
        Ok(count.to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(64.to_string()),
            false => Some(3466.to_string()),
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
