use anyhow::Result;

use crate::puzzle::Puzzle;

#[allow(unused_imports)]
use crate::utils::utils::*;

#[derive(Debug)]
pub struct Day19 {
    blueprints: Vec<Blueprint>,
}

#[derive(Debug, Clone, Copy)]
struct Resources {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
}

#[derive(Debug, Clone, Copy)]
struct Cost {
    ore: u32,
    clay: u32,
    obsidian: u32,
}

#[derive(Debug, Clone, Copy)]
struct Robots {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
}

#[derive(Debug, Clone, Copy)]
enum Robot {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug, Clone, Copy)]
struct Blueprint {
    robot_ore: Cost,
    robot_clay: Cost,
    robot_obsidian: Cost,
    robot_geode: Cost,
    max_cost: Cost, // The maximum of each resource for each robot cost
}

// Try all possibilities based on the blueprint, return max geode made
fn do_work(blueprint: &Blueprint, total_time: u32) -> u32 {
    #[derive(Debug, Clone)]
    struct Work {
        robots: Robots,
        resources: Resources,
        what_to_build: Robot,
        time_left: u32,
        time_passed: u32,
    }
    let mut jobs: Vec<Work> = vec![];
    let mut job = Work {
        robots: Robots {
            ore: 1,
            clay: 0,
            obsidian: 0,
            geode: 0,
        },
        resources: Resources {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
        },
        what_to_build: Robot::Ore,
        time_left: total_time,
        time_passed: 1,
    };
    for choice in work_choices(&job.robots, &blueprint) {
        job.what_to_build = choice;
        jobs.push(job.clone());
    }
    let mut max_geodes = 0;

    while jobs.len() > 0 {
        let mut job = jobs.pop().unwrap();

        // Try to build the robot
        let build_done = build_robot(job.what_to_build, &mut job.resources, blueprint);

        // Get our new resources
        mine_resources(&job.robots, &mut job.resources);

        // Tick the clock
        job.time_left -= 1;
        job.time_passed += 1;
        if job.time_left == 0 {
            max_geodes = std::cmp::max(max_geodes, job.resources.geode);
            log::debug!("All done, got {} geodes", job.resources.geode);
            continue;
        }

        // Bail early if this is a terrible path
        let max_possible_geods =
            max_geodes_we_could_get(job.time_left, &job.robots, &job.resources);
        if max_possible_geods < max_geodes {
            log::debug!("Giving up, geodes best possible {max_possible_geods} < {max_geodes}, the current max");
            continue;
        }

        // Find new work to do
        if build_done {
            // Add our new robot to the list
            match job.what_to_build {
                Robot::Ore => job.robots.ore += 1,
                Robot::Clay => job.robots.clay += 1,
                Robot::Obsidian => job.robots.obsidian += 1,
                Robot::Geode => job.robots.geode += 1,
            };

            for choice in work_choices(&job.robots, &blueprint) {
                job.what_to_build = choice;
                jobs.push(job.clone());
            }
        } else {
            // Reschedule this work
            jobs.push(job)
        }
    }

    max_geodes
}

// Return a list of the robots we can build
fn work_choices(robots: &Robots, blueprint: &Blueprint) -> Vec<Robot> {
    let mut choices = vec![];

    // Only build this robot if we are not already maxed out on this robot type
    if robots.ore < blueprint.max_cost.ore {
        choices.push(Robot::Ore);
    }

    if robots.clay < blueprint.max_cost.clay {
        choices.push(Robot::Clay);
    }

    if robots.clay > 0 {
        if robots.obsidian < blueprint.max_cost.obsidian {
            choices.push(Robot::Obsidian);
        }

        // We never max out on geode robots
        if robots.obsidian > 0 {
            choices.push(Robot::Geode);
        }
    }
    choices
}

// Try to build a robot if we have the needed resources
// If possible, the robot is built, resources are deducted, and this returns true
// If not possible, then this returns false
fn build_robot(robot: Robot, resources: &mut Resources, blueprint: &Blueprint) -> bool {
    let cost = match robot {
        Robot::Ore => blueprint.robot_ore,
        Robot::Clay => blueprint.robot_clay,
        Robot::Obsidian => blueprint.robot_obsidian,
        Robot::Geode => blueprint.robot_geode,
    };

    if resources.ore >= cost.ore
        && resources.clay >= cost.clay
        && resources.obsidian >= cost.obsidian
    {
        log::debug!("Build robot {:?}, cost {:?}", robot, cost);
        resources.ore -= cost.ore;
        resources.clay -= cost.clay;
        resources.obsidian -= cost.obsidian;

        return true;
    }

    false
}

// Based on the type of robots we have, increase resources count
fn mine_resources(robots: &Robots, resources: &mut Resources) {
    resources.ore += robots.ore;
    resources.clay += robots.clay;
    resources.obsidian += robots.obsidian;
    resources.geode += robots.geode;
}

// Given unlimited resources, how many geode could we make?
fn max_geodes_we_could_get(time_left: u32, robots: &Robots, resources: &Resources) -> u32 {
    let mut count = resources.geode;
    let mut geode_robots = robots.geode;
    for _ in 0..=time_left {
        count += geode_robots;
        geode_robots += 1;
    }
    count
}

impl Puzzle for Day19 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut day = Day19 { blueprints: vec![] };

        for line in input.lines() {
            let data: Vec<&str> = line.split(":").collect();
            let robots_data = data[1];
            let robots: Vec<&str> = robots_data.split(".").collect();

            let ore_robot_cost_ore: u32 = get_val(robots[0]);

            let clay_robot_cost_ore: u32 = get_val(robots[1]);

            let obsidian_robot_costs: Vec<u32> = get_vals(robots[2]);
            let obsidian_robot_cost_ore = obsidian_robot_costs[0];
            let obsidian_robot_cost_clay = obsidian_robot_costs[1];

            let geode_robot_costs: Vec<u32> = get_vals(robots[3]);
            let geode_robot_cost_ore = geode_robot_costs[0];
            let geode_robot_cost_obsidian = geode_robot_costs[1];

            let blueprint = Blueprint {
                robot_ore: Cost {
                    ore: ore_robot_cost_ore,
                    clay: 0,
                    obsidian: 0,
                },
                robot_clay: Cost {
                    ore: clay_robot_cost_ore,
                    clay: 0,
                    obsidian: 0,
                },
                robot_obsidian: Cost {
                    ore: obsidian_robot_cost_ore,
                    clay: obsidian_robot_cost_clay,
                    obsidian: 0,
                },
                robot_geode: Cost {
                    ore: geode_robot_cost_ore,
                    clay: 0,
                    obsidian: geode_robot_cost_obsidian,
                },
                max_cost: Cost {
                    ore: *vec![
                        ore_robot_cost_ore,
                        clay_robot_cost_ore,
                        obsidian_robot_cost_ore,
                        geode_robot_cost_ore,
                    ]
                    .iter()
                    .max()
                    .unwrap(),
                    clay: obsidian_robot_cost_clay,
                    obsidian: geode_robot_cost_obsidian,
                },
            };
            day.blueprints.push(blueprint);
        }

        log::debug!("Blueprints {:#?}", day.blueprints);

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        let mut score = 0;
        for (i, blueprint) in self.blueprints.iter().enumerate() {
            let geode = do_work(&blueprint, 24);
            log::info!("[{i}] Max geodes found {geode}");
            score = score + (i + 1) as u32 * geode;
        }
        Ok(score.to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(33.to_string()),
            false => Some(1962.to_string()),
        }
    }

    fn solve_part2(&mut self) -> Result<String> {
        let mut score = 1;
        let len = std::cmp::min(3, self.blueprints.len());
        for i in 0..len {
            let blueprint = self.blueprints[i];
            let geode = do_work(&blueprint, 32);
            log::info!("[{i}] Max geodes found {geode}");
            score = score * geode;
        }
        Ok(score.to_string())
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(3472.to_string()),
            false => Some(88160.to_string()),
        }
    }
}
