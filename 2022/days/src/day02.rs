use anyhow::Result;

use crate::puzzle::Puzzle;
use crate::rock_paper_scissors::*;

pub struct Day02 {
    matches: Vec<(ItemType, ItemType)>,
    matches2: Vec<(ItemType, ItemType)>,
}

fn get_type_value(item: &ItemType) -> u32 {
    match item {
        ItemType::Rock => 1,
        ItemType::Paper => 2,
        ItemType::Scissors => 3,
    }
}

fn get_result_value(result: ResultType) -> u32 {
    match result {
        ResultType::Loss => 0,
        ResultType::Tie => 3,
        ResultType::Win => 6,
    }
}

fn get_input_result(input: char) -> ResultType {
    match input {
        'X' => ResultType::Loss,
        'Y' => ResultType::Tie,
        'Z' => ResultType::Win,
        _ => panic!("Invalid request to get result type"),
    }
}

fn get_their_type(input: char) -> ItemType {
    match input {
        'A' => ItemType::Rock,
        'B' => ItemType::Paper,
        'C' => ItemType::Scissors,
        _ => panic!("Invalid input for their type"),
    }
}

fn get_my_type(input: char) -> ItemType {
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
        sum += get_type_value(me);
        let result = get_result(me, them);
        sum += get_result_value(result);
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
            let them = line.chars().next().unwrap();
            let me = line.chars().last().unwrap();

            let them_type = get_their_type(them);
            let my_type = get_my_type(me);

            day.matches.push((them_type, my_type));

            let result = get_input_result(me);
            day.matches2
                .push((them_type, get_type_from_result(them_type, result)));
        }

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
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
