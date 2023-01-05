// 2015 Day 15
// https://adventofcode.com/2015/day/15
// --- Day 15: Science for Hungry People ---

use anyhow::Result;

use crate::puzzle::Puzzle;

#[allow(unused_imports)]
use crate::utils::utils::*;

#[allow(unused_imports)]
use std::collections::VecDeque;

pub struct Day15 {
    ingredients: Vec<Ingredient>,
}

#[derive(Debug)]
struct Ingredient {
    name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

fn mix(max: i32, depth: u32, values: Vec<i32>) {
    let new_depth = depth - 1;
    for x in 0..=max {
        let mut new_values = values.clone();
        new_values.push(x);
        if new_depth > 0 {
            mix(max - x, new_depth, new_values);
        }
    }
}

fn find_best_score(day: &Day15) -> i32 {
    let ingredient_count = day.ingredients.len();
    struct Work {
        values: Vec<i32>,
    }
    let mut jobs = vec![];

    for x in 0..=100 {
        jobs.push(Work { values: vec![x] });
    }

    while jobs.len() > 0 {
        let job = jobs.pop().unwrap();

        if job.values.len() == ingredient_count {
            // Get the score
            continue;
        }

        // Start new jobs
    }

    for a in 0..=100 {
        for b in 0..=100 - a {
            for c in 0..=100 - a - b {
                let d = 100 - a - b - c;
                if d > 0 {
                    // This is a recipe
                }
            }
        }
    }

    0 // TODO
}

impl Puzzle for Day15 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut day = Day15 {
            ingredients: vec![],
        };

        for line in input.lines() {
            let splits: Vec<&str> = line.split(" ").collect();
            let name = splits[0].to_string();
            let vals: Vec<i32> = find_vals(line);
            day.ingredients.push(Ingredient {
                name,
                capacity: vals[0],
                durability: vals[0],
                flavor: vals[0],
                texture: vals[0],
                calories: vals[0],
            })
        }

        log::debug!("{:#?}", day.ingredients);

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        Ok("to do".to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(62842880.to_string()),
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
