// 2015 Day 15
// https://adventofcode.com/2015/day/15
// --- Day 15: Science for Hungry People ---
// For different ingredients, find the best score for different ingredient proportions

use anyhow::Result;

use crate::puzzle::Puzzle;
use crate::utils::utils::*;

pub struct Day15 {
    ingredients: Vec<Ingredient>,
}

#[derive(Debug, Clone)]
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
    let mut max = 0;

    let mut ingredients = day.ingredients.clone();
    if ingredients.len() < 4 {
        // Add two dummy ingredients for test case so len is always 4
        let dummy = Ingredient {
            name: "Dummy".to_string(),
            capacity: 0,
            durability: 0,
            flavor: 0,
            texture: 0,
            calories: 0,
        };
        ingredients.push(dummy.clone());
        ingredients.push(dummy.clone());
    }

    // Just do nested loops. It's super fast.
    for a in 0..=100 {
        for b in 0..=100 - a {
            for c in 0..=100 - a - b {
                for d in 0..=100 - a - b - c {
                    // Get the score
                    let capacity = a * ingredients[0].capacity
                        + b * ingredients[1].capacity
                        + c * ingredients[2].capacity
                        + d * ingredients[3].capacity;
                    let durability = a * ingredients[0].durability
                        + b * ingredients[1].durability
                        + c * ingredients[2].durability
                        + d * ingredients[3].durability;
                    let flavor = a * ingredients[0].flavor
                        + b * ingredients[1].flavor
                        + c * ingredients[2].flavor
                        + d * ingredients[3].flavor;
                    let texture = a * ingredients[0].texture
                        + b * ingredients[1].texture
                        + c * ingredients[2].texture
                        + d * ingredients[3].texture;
                    let calories = a * ingredients[0].calories
                        + b * ingredients[1].calories
                        + c * ingredients[2].calories
                        + d * ingredients[3].calories;
                    if capacity <= 0 || durability <= 0 || flavor <= 0 || texture <= 0 {
                        continue;
                    }
                    if must_be_500_cal && calories != 500 {
                        continue;
                    }
                    let score = capacity * durability * flavor * texture;
                    max = std::cmp::max(max, score);
                }
            }
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
