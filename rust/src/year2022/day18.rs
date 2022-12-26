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
fn can_escape(day: &Day18, point: &Point3d) -> bool {
    let mut jobs = vec![];
    jobs.push(point.clone());
    let mut job_count = 0;
    let mut stack_max = 0;

    // Optimize by only looking at a spot we've been once
    let size = day.grid.get_size();
    let mut been_here = Grid3d::new(size, false);

    log::trace!("Can {:?} escape?", point);

    while jobs.len() > 0 {
        job_count += 1;
        stack_max = std::cmp::max(stack_max, jobs.len());
        let job_point = jobs.pop().unwrap();

        if been_here.get_at(&job_point) {
            log::trace!("Been at {:?}", job_point);
            continue;
        }
        been_here.set_at(&job_point, true);

        let spot = day.grid.get_at(&job_point);
        match spot {
            ScanType::Lava => {
                log::trace!("Hit lava at {:?}", job_point);
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

        if job_point.x == 0
            || job_point.y == 0
            || job_point.z == 0
            || job_point.x == size - 1
            || job_point.y == size - 1
            || job_point.z == size - 1
        {
            log::trace!("Can escape, found edge");
            return true;
        }

        struct Point3dSigned {
            x: isize,
            y: isize,
            z: isize,
        }
        let offsets = vec![
            Point3dSigned { x: 1, y: 0, z: 0 },
            Point3dSigned { x: -1, y: 0, z: 0 },
            Point3dSigned { x: 0, y: 1, z: 0 },
            Point3dSigned { x: 0, y: -1, z: 0 },
            Point3dSigned { x: 0, y: 0, z: 1 },
            Point3dSigned { x: 0, y: 0, z: -1 },
        ];

        for offset in offsets {
            let x = ((job_point.x as isize) + offset.x) as usize;
            let y = ((job_point.y as isize) + offset.y) as usize;
            let z = ((job_point.z as isize) + offset.z) as usize;
            jobs.push(Point3d { x, y, z });
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
                        // This spot is lava
                        lava += 1;

                        // See if nearby values are not lava
                        let values = self.grid.get_nearby_values(&point);
                        for value in values.iter() {
                            if *value != ScanType::Lava {
                                count += 1;
                            }
                        }

                        // Parts off grid are also not lava
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
        for x in 0..size {
            for y in 0..size {
                for z in 0..size {
                    let point = Point3d { x, y, z };
                    if self.grid.get_at(&point) == ScanType::Unknown {
                        if can_escape(&self, &point) {
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

        let mut count = 0;
        let mut lava = 0;
        // Find how many edges of lava are exposed to outside air
        // Search all lava cubes and count adjacent outside cubes
        for x in 0..size {
            for y in 0..size {
                for z in 0..size {
                    let point = Point3d { x, y, z };
                    if self.grid.get_at(&point) == ScanType::Lava {
                        // This is lava
                        lava += 1;

                        // Find everything nearby that is "outside" type
                        let values = self.grid.get_nearby_values(&point);
                        for value in values.iter() {
                            if *value == ScanType::Outside {
                                count += 1;
                            }
                        }

                        // Parts off grid are also "outside"
                        count += 6 - values.len();
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
