// 2015 Day 15
// https://adventofcode.com/2015/day/15
// --- Day 15: Science for Hungry People ---
// For different ingredients, find the best score for different ingredient proportions

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
    #[allow(dead_code)]
    name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

/// Mix ingredients and find best score
fn find_best_score(day: &Day15, must_be_500_cal: bool) -> i32 {
    let ingredient_count = day.ingredients.len();
    struct Work {
        values: Vec<i32>,
    }
    let mut jobs = vec![];

    for x in 0..=100 {
        jobs.push(Work { values: vec![x] });
    }

    let mut max = 0;

    while jobs.len() > 0 {
        let job = jobs.pop().unwrap();

        if job.values.len() == ingredient_count {
            // Get the score
            let mut capacity = 0;
            let mut durability = 0;
            let mut flavor = 0;
            let mut texture = 0;
            let mut calories = 0;
            for (i, ingredient) in day.ingredients.iter().enumerate() {
                let scaller = job.values[i];
                capacity += ingredient.capacity * scaller;
                durability += ingredient.durability * scaller;
                flavor += ingredient.flavor * scaller;
                texture += ingredient.texture * scaller;
                calories += ingredient.calories * scaller;
            }
            if capacity <= 0 || durability <= 0 || flavor <= 0 || texture <= 0 {
                continue;
            }
            if must_be_500_cal && calories != 500 {
                continue;
            }
            let score = capacity * durability * flavor * texture;
            if score > max {
                log::debug!("New best score {:?} -> {score} [{capacity}, {durability}, {flavor}, {texture}]", job.values);
            }
            max = std::cmp::max(max, score);
            continue;
        }

        // Start new jobs
        let sum: i32 = job.values.iter().sum();
        for x in 0..=100 - sum {
            let mut values = job.values.clone();
            values.push(x);
            jobs.push(Work { values });
        }
    }

    max
}

impl Puzzle for Day15 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut day = Day15 {
            ingredients: vec![],
        };

        for line in input.lines() {
            // Input looks like
            // Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
            let splits: Vec<&str> = line.split(" ").collect();
            let name = splits[0].to_string();
            let vals: Vec<i32> = find_vals(line);
            day.ingredients.push(Ingredient {
                name,
                capacity: vals[0],
                durability: vals[1],
                flavor: vals[2],
                texture: vals[3],
                calories: vals[4],
            })
        }

        log::debug!("{:#?}", day.ingredients);

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        // Find best combination of ingredients
        let answer = find_best_score(self, false);
        Ok(answer.to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(62842880.to_string()),
            false => Some(13882464.to_string()),
        }
    }

    fn solve_part2(&mut self) -> Result<String> {
        // Find best combination of ingredients where calories is exactly 500
        let answer = find_best_score(self, true);
        Ok(answer.to_string())
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(57600000.to_string()),
            false => Some(11171160.to_string()),
        }
    }
}
