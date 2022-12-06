use anyhow::Result;
use std::collections::HashMap;

use crate::puzzle::Puzzle;

#[derive(Debug)]
pub struct Day02 {
    matches: Vec<(String, String)>,
    matches2: Vec<(String, String)>,
}

fn get_type_value(item: &String) -> u32 {
    match item.as_ref() {
        "rock" => 1,
        "paper" => 2,
        "scissors" => 3,
        _ => 0,
    }
}

fn get_result_value(result: &String) -> u32 {
    match result.as_ref() {
        "loss" => 0,
        "tie" => 3,
        "win" => 6,
        _ => 0,
    }
}

fn get_result(them: &String, me: &String) -> String {
    match (me.as_ref(), them.as_ref()) {
        ("rock", "rock") => "tie".to_string(),
        ("rock", "paper") => "loss".to_string(),
        ("rock", "scissors") => "win".to_string(),
        ("paper", "rock") => "win".to_string(),
        ("paper", "paper") => "tie".to_string(),
        ("paper", "scissors") => "loss".to_string(),
        ("scissors", "rock") => "loss".to_string(),
        ("scissors", "paper") => "win".to_string(),
        ("scissors", "scissors") => "tie".to_string(),
        _ => "ERROR!!!!".to_string(),
    }
}

fn get_my_type(them: &String, result: &String) -> String {
    match (them.as_ref(), result.as_ref()) {
        ("rock", "loss") => "scissors".to_string(),
        ("rock", "tie") => "rock".to_string(),
        ("rock", "win") => "paper".to_string(),
        ("paper", "loss") => "rock".to_string(),
        ("paper", "tie") => "paper".to_string(),
        ("paper", "win") => "scissors".to_string(),
        ("scissors", "loss") => "paper".to_string(),
        ("scissors", "tie") => "scissors".to_string(),
        ("scissors", "win") => "rock".to_string(),
        _ => "ERRROROROROR".to_string(),
    }
}

fn get_input_result(input: char) -> String {
    match input {
        'X' => "loss".to_string(),
        'Y' => "tie".to_string(),
        'Z' => "win".to_string(),
        _ => "EROEOREROE".to_string(),
    }
}

impl Puzzle for Day02 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut day = Day02 {
            matches: vec![],
            matches2: vec![],
        };

        let them_to_type = HashMap::from([
            ('A', "rock".to_string()),
            ('B', "paper".to_string()),
            ('C', "scissors".to_string()),
        ]);
        let me_to_type = HashMap::from([
            ('X', "rock".to_string()),
            ('Y', "paper".to_string()),
            ('Z', "scissors".to_string()),
        ]);

        for line in input.lines() {
            let them = line.chars().next().unwrap();
            let me = line.chars().last().unwrap();

            let them_type = them_to_type[&them].clone();

            day.matches
                .push((them_type.clone(), me_to_type[&me].clone()));

            let result = get_input_result(me);
            day.matches2
                .push((them_type.clone(), get_my_type(&them_type, &result)));
        }

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        let mut sum = 0;
        for (them, me) in self.matches.iter() {
            sum += get_type_value(&me);
            let result = get_result(&them, &me);
            sum += get_result_value(&result)
        }
        Ok(sum.to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(15.to_string()),
            false => Some(11666.to_string()),
        }
    }

    fn solve_part2(&mut self) -> Result<String> {
        let mut sum = 0;
        for (them, me) in self.matches2.iter() {
            sum += get_type_value(&me);
            let result = get_result(&them, &me);
            sum += get_result_value(&result)
        }
        Ok(sum.to_string())
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(12.to_string()),
            false => Some(12767.to_string()),
        }
    }
}
