// 2022 Day 2
// https://adventofcode.com/2022/day/2
// --- Day 2: Rock Paper Scissors ---
// Play lots of rock paper scissors games.
// Need to get reults. And also determine what I should play to get a specific result.

use anyhow::Result;

use crate::puzzle::Puzzle;
use crate::utils::rock_paper_scissors::*;

pub struct Day02 {
    matches: Vec<(ItemType, ItemType)>,
    matches2: Vec<(ItemType, ItemType)>,
}

fn determine_type_value(item: &ItemType) -> u32 {
    match item {
        ItemType::Rock => 1,
        ItemType::Paper => 2,
        ItemType::Scissors => 3,
    }
}

fn determine_result_value(result: &ResultType) -> u32 {
    match result {
        ResultType::Loss => 0,
        ResultType::Tie => 3,
        ResultType::Win => 6,
    }
}

fn determine_input_result(input: &char) -> ResultType {
    match input {
        'X' => ResultType::Loss,
        'Y' => ResultType::Tie,
        'Z' => ResultType::Win,
        _ => panic!("Invalid request to get result type"),
    }
}

fn determine_input_their_type(input: &char) -> ItemType {
    match input {
        'A' => ItemType::Rock,
        'B' => ItemType::Paper,
        'C' => ItemType::Scissors,
        _ => panic!("Invalid input for their type"),
    }
}

fn determine_input_my_type(input: &char) -> ItemType {
    match input {
        'X' => ItemType::Rock,
        'Y' => ItemType::Paper,
        'Z' => ItemType::Scissors,
        _ => panic!("Invalid input for my type"),
    }
}

fn sum_matches(matches: &Vec<(ItemType, ItemType)>) -> u32 {
    let mut sum = 0;
    for (them, me) in matches.iter() {
        sum += determine_type_value(me);
        let result = determine_result(me, them);
        sum += determine_result_value(&result);
    }
    sum
}

impl Puzzle for Day02 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut day = Day02 {
            matches: vec![],
            matches2: vec![],
        };

        for line in input.lines() {
            let value1 = line.chars().next().unwrap();
            let value2 = line.chars().last().unwrap();

            // For part 1, get result from two players
            let them_type = determine_input_their_type(&value1);
            let my_type = determine_input_my_type(&value2);
            day.matches.push((them_type, my_type));

            // For part 2, get my type from one player and desired result
            let them_type2 = determine_input_their_type(&value1);
            let result = determine_input_result(&value2);
            let my_type2 = determine_type_from_result(&them_type2, &result);
            day.matches2.push((them_type2, my_type2));
        }

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        // Sum scores
        let sum = sum_matches(&self.matches);
        Ok(sum.to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(15.to_string()),
            false => Some(11666.to_string()),
        }
    }

    fn solve_part2(&mut self) -> Result<String> {
        // Sum scores
        let sum = sum_matches(&self.matches2);
        Ok(sum.to_string())
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(12.to_string()),
            false => Some(12767.to_string()),
        }
    }
}
