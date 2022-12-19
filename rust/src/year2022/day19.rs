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
    ore: Resource,
    clay: Resource,
    obsidian: Resource,
    geode: Resource,
}

#[derive(Debug, Clone, Copy)]
enum Resource {
    Ore(u32),
    Clay(u32),
    Obsidian(u32),
    Geode(u32),
}

#[derive(Debug, Clone, Copy)]
struct Cost {
    ore: Resource,
    clay: Resource,
    obsidian: Resource,
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
}

fn do_work(day: &mut Day19) {
    struct Work {
        robots: Robots,
        resources: Resources,
        what_to_build: Robot,
    }
    let mut jobs: Vec<Work> = vec![];
    let job = Work {
        robots: Robots {
            ore: 1,
            clay: 0,
            obsidian: 0,
            geode: 0,
        },
        resources: Resources {
            ore: Resource::Ore(0),
            clay: Resource::Clay(0),
            obsidian: Resource::Obsidian(0),
            geode: Resource::Geode(0),
        },
        what_to_build: Robot::Ore,
    };
    jobs.push(job);

    while jobs.len() > 0 {
        let job = jobs.pop();
    }
}

fn work_choices(robots: &Robots) -> Vec<Robot> {
    vec![]
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
                    ore: Resource::Ore(ore_robot_cost_ore),
                    clay: Resource::Clay(0),
                    obsidian: Resource::Obsidian(0),
                },
                robot_clay: Cost {
                    ore: Resource::Ore(clay_robot_cost_ore),
                    clay: Resource::Clay(0),
                    obsidian: Resource::Obsidian(0),
                },
                robot_obsidian: Cost {
                    ore: Resource::Ore(obsidian_robot_cost_ore),
                    clay: Resource::Clay(obsidian_robot_cost_clay),
                    obsidian: Resource::Obsidian(0),
                },
                robot_geode: Cost {
                    ore: Resource::Ore(geode_robot_cost_ore),
                    clay: Resource::Clay(0),
                    obsidian: Resource::Obsidian(geode_robot_cost_obsidian),
                },
            };
            day.blueprints.push(blueprint);
        }

        log::debug!("Blueprints {:#?}", day.blueprints);

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        Ok("to do".to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => None,
            false => None,
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
