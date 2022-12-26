// 2022 Day 18
// https://adventofcode.com/2022/day/18
// --- Day 18: Boiling Boulders ---
// Cubes in 3d space
// Need to map paths in 3d!

use anyhow::Result;

use crate::puzzle::Puzzle;
use crate::utils::grid3d::*;
use crate::utils::utils::*;

#[derive(Clone, Copy, PartialEq, Debug)]
enum ScanType {
    Lava,
    Unknown,
    AirPocket,
    Outside,
}

pub struct Day18 {
    grid: Grid3d<ScanType>,
}

/// For a given spot on the 3d board, can we get to the edge of cube or are we blocked?
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

    // Optimize by only looking at a spot we've been once
    let size = day.grid.get_size();
    let mut been_here = vec![vec![vec![false; size]; size]; size];

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

        let spot = day.grid.get_at(&Point3d {
            x: job.x,
            y: job.y,
            z: job.z,
        });
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
            || job.x == size - 1
            || job.y == size - 1
            || job.z == size - 1
        {
            log::trace!("Can escape, found edge");
            return true;
        }

        struct Point3di32 {
            x: i32,
            y: i32,
            z: i32,
        }
        let offsets = vec![
            Point3di32 { x: 1, y: 0, z: 0 },
            Point3di32 { x: -1, y: 0, z: 0 },
            Point3di32 { x: 0, y: 1, z: 0 },
            Point3di32 { x: 0, y: -1, z: 0 },
            Point3di32 { x: 0, y: 0, z: 1 },
            Point3di32 { x: 0, y: 0, z: -1 },
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
            grid: Grid3d::new(size, ScanType::Unknown),
        };

        let mut max = 0;
        let mut count = 0;

        // Add in the lava to the 3d grid
        for line in input.lines() {
            let values: Vec<usize> = get_vals(line);
            let x = values[0];
            let y = values[1];
            let z = values[2];
            max = std::cmp::max(max, x);
            max = std::cmp::max(max, y);
            max = std::cmp::max(max, z);
            let point = Point3d { x, y, z };
            assert_eq!(day.grid.get_at(&point), ScanType::Unknown);
            day.grid.set_at(&point, ScanType::Lava);
            count += 1;
        }

        day.grid.resize(max + 1);
        log::debug!("Max is {max}, count is {count}");

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        let mut count = 0;
        let mut lava = 0;
        // Count the exposed edges of lava
        // Check every lava and count spots around it that are not lava
        let size = self.grid.get_size();
        for x in 0..size {
            for y in 0..size {
                for z in 0..size {
                    let point = Point3d { x, y, z };
                    if self.grid.get_at(&point) == ScanType::Lava {
                        lava += 1;
                        let values = self.grid.get_nearby_values(&point);
                        for value in values.iter() {
                            if *value != ScanType::Lava {
                                count += 1;
                            }
                        }
                        count += 6 - values.len();
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
        // Find air pockets, these are non lava spots that cannot escape the grid
        // Walk the whole grid and check unknown spots
        let size = self.grid.get_size();
        let size_i32 = size as i32;
        for x in 0..size {
            for y in 0..size {
                for z in 0..size {
                    let point = Point3d { x, y, z };
                    if self.grid.get_at(&point) == ScanType::Unknown {
                        if can_escape(&self, x, y, z) {
                            self.grid.set_at(&point, ScanType::Outside);
                        } else {
                            self.grid.set_at(&point, ScanType::AirPocket);
                            air_pockets += 1;
                        }
                    }
                }
            }
        }
        log::debug!("Done populating");

        // Everything else unknown is therefore outside the lava
        for x in 0..size {
            for y in 0..size {
                for z in 0..size {
                    let point = Point3d { x, y, z };
                    if self.grid.get_at(&point) == ScanType::Unknown {
                        self.grid.set_at(&point, ScanType::Outside);
                    }
                }
            }
        }

        let mut count = 0;
        let mut lava = 0;
        // Find how many edges of lava are exposed to outside air
        // Search all lava cubes and count adjacent outside cubes
        for x in 0..size {
            for y in 0..size {
                for z in 0..size {
                    let mut point = Point3d { x, y, z };
                    if self.grid.get_at(&point) == ScanType::Lava {
                        lava += 1;
                        for dx in [-1i32, 1] {
                            let x2 = (x as i32) + dx;
                            if x2 < 0 || x2 >= size_i32 {
                                count += 1;
                                continue;
                            }
                            let x3 = x2 as usize;
                            point = Point3d { x: x3, y, z };
                            if self.grid.get_at(&point) == ScanType::Outside {
                                count += 1;
                            }
                        }
                        for dy in [-1i32, 1] {
                            let y2 = (y as i32) + dy;
                            if y2 < 0 || y2 >= size_i32 {
                                count += 1;
                                continue;
                            }
                            let y3 = y2 as usize;
                            point = Point3d { x, y: y3, z };
                            if self.grid.get_at(&point) == ScanType::Outside {
                                count += 1;
                            }
                        }
                        for dz in [-1i32, 1] {
                            let z2 = (z as i32) + dz;
                            if z2 < 0 || z2 >= size_i32 {
                                count += 1;
                                continue;
                            }
                            let z3 = z2 as usize;
                            point = Point3d { x, y, z: z3 };
                            if self.grid.get_at(&point) == ScanType::Outside {
                                count += 1;
                            }
                        }
                    }
                }
            }
        }

        log::debug!("Found {air_pockets} air pockets and {lava} lava");

        Ok(count.to_string())
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(58.to_string()),
            false => Some(2012.to_string()),
        }
    }
}
