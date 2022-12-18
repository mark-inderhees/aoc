use std::iter::Scan;

use anyhow::Result;

use crate::puzzle::Puzzle;

#[allow(unused_imports)]
use crate::utils::utils::*;

#[derive(Clone, Copy, PartialEq, Debug)]
enum ScanType {
    Lava,
    Unknown,
    AirPocket,
    Outside,
}

pub struct Day18 {
    box_3d: Vec<Vec<Vec<ScanType>>>,
    size: usize,
}

fn can_escape(day: &Day18, x: usize, y: usize, z: usize) -> bool {
    struct PathWork {
        x: usize,
        y: usize,
        z: usize,
    }
    let mut jobs = vec![];
    jobs.push(PathWork { x, y, z });
    let mut job_count = 0;
    let mut stack_max = 0;

    let mut been_here = vec![vec![vec![false; day.size]; day.size]; day.size];

    log::trace!("Can {x},{y},{z} escape?");

    while jobs.len() > 0 {
        job_count += 1;
        stack_max = std::cmp::max(stack_max, jobs.len());
        let job = jobs.pop().unwrap();

        if been_here[job.x][job.y][job.z] {
            log::trace!("Been at {},{},{}", job.x, job.y, job.z);
            continue;
        }
        been_here[job.x][job.y][job.z] = true;

        let spot = day.box_3d[job.x][job.y][job.z];
        match spot {
            ScanType::Lava => {
                log::trace!("Hit lava at {},{},{}", job.x, job.y, job.z);
                continue;
            }
            ScanType::AirPocket => {
                log::trace!("Cannot escape, found air pocket");
                return false;
            }
            ScanType::Outside => {
                log::trace!("Can escape, found outside");
                return true;
            }
            ScanType::Unknown => (),
        }

        if job.x == 0
            || job.y == 0
            || job.z == 0
            || job.x == day.size - 1
            || job.y == day.size - 1
            || job.z == day.size - 1
        {
            log::trace!("Can escape, found edge");
            return true;
        }

        struct Point3d {
            x: i32,
            y: i32,
            z: i32,
        }
        let offsets = vec![
            Point3d { x: 1, y: 0, z: 0 },
            Point3d { x: -1, y: 0, z: 0 },
            Point3d { x: 0, y: 1, z: 0 },
            Point3d { x: 0, y: -1, z: 0 },
            Point3d { x: 0, y: 0, z: 1 },
            Point3d { x: 0, y: 0, z: -1 },
        ];

        for offset in offsets {
            let x2 = (job.x as i32) + offset.x;
            let y2 = (job.y as i32) + offset.y;
            let z2 = (job.z as i32) + offset.z;
            let x3 = x2 as usize;
            let y3 = y2 as usize;
            let z3 = z2 as usize;
            jobs.push(PathWork {
                x: x3,
                y: y3,
                z: z3,
            });
        }
    }

    log::trace!("Cannot escape, tried everywhere. {job_count} jobs, {stack_max} max stack");
    false
}

impl Puzzle for Day18 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let size = 30;
        let mut day = Day18 {
            box_3d: vec![vec![vec![ScanType::Unknown; size]; size]; size],
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
            assert_eq!(day.box_3d[x][y][z], ScanType::Unknown);
            day.box_3d[x][y][z] = ScanType::Lava;
            count += 1;
        }

        day.size = max + 1;
        log::debug!("Max is {max}, count is {count}");

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        let mut count = 0;
        let mut lava = 0;
        for x in 0..self.size {
            for y in 0..self.size {
                for z in 0..self.size {
                    if self.box_3d[x][y][z] == ScanType::Lava {
                        lava += 1;
                        for dx in [-1i32, 1] {
                            let x2 = (x as i32) + dx;
                            if x2 < 0 {
                                count += 1;
                                continue;
                            }
                            let x3 = x2 as usize;
                            if self.box_3d[x3][y][z] != ScanType::Lava {
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
                            if self.box_3d[x][y3][z] != ScanType::Lava {
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
                            if self.box_3d[x][y][z3] != ScanType::Lava {
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
        let mut air_pockets = 0;
        log::debug!("Populating types");
        for x in 0..self.size {
            for y in 0..self.size {
                for z in 0..self.size {
                    if self.box_3d[x][y][z] == ScanType::Unknown {
                        if can_escape(&self, x, y, z) {
                            self.box_3d[x][y][z] = ScanType::Outside;
                        } else {
                            self.box_3d[x][y][z] = ScanType::AirPocket;
                            air_pockets += 1;
                        }
                    }
                }
            }
        }
        log::debug!("Done populating");

        for x in 0..self.size + 1 {
            for y in 0..self.size + 1 {
                for z in 0..self.size + 1 {
                    if self.box_3d[x][y][z] == ScanType::Unknown {
                        self.box_3d[x][y][z] = ScanType::Outside;
                    }
                }
            }
        }

        let mut count = 0;
        let mut lava = 0;
        for x in 0..self.size {
            for y in 0..self.size {
                for z in 0..self.size {
                    if self.box_3d[x][y][z] == ScanType::Lava {
                        lava += 1;
                        for dx in [-1i32, 1] {
                            let x2 = (x as i32) + dx;
                            if x2 < 0 {
                                count += 1;
                                continue;
                            }
                            let x3 = x2 as usize;
                            if self.box_3d[x3][y][z] == ScanType::Outside {
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
                            if self.box_3d[x][y3][z] == ScanType::Outside {
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
                            if self.box_3d[x][y][z3] == ScanType::Outside {
                                count += 1;
                            }
                        }
                    }
                }
            }
        }

        log::debug!("Found {air_pockets} air pockets");

        Ok(count.to_string())
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(58.to_string()),
            false => Some(2012.to_string()),
        }
    }
}
