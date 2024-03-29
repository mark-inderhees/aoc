// 2015 Day 13
// https://adventofcode.com/2015/day/13
// --- Day 13: Knights of the Dinner Table ---
// Walk all permuatations for scores around a dinner table

use anyhow::Result;
use std::collections::HashMap;

use crate::puzzle::Puzzle;
use crate::utils::utils::*;

pub struct Day13 {
    /// Map of people scores when sitting next to other people
    people: HashMap<String, HashMap<String, i32>>,
}

/// Try all permuations of seating and return the best overall score
fn find_best_seating(day: &Day13) -> i32 {
    struct Work<'a> {
        person: &'a String,
        seated: Vec<&'a String>,
        score: i32,
    }

    // Start with Alice
    let first_person = "Alice".to_string();
    let mut jobs = vec![Work {
        person: &first_person,
        seated: vec![&first_person],
        score: 0,
    }];
    let mut high_score = i32::MIN;

    while jobs.len() > 0 {
        let job = jobs.pop().unwrap();

        let mut started_work = false;
        for person in day.people.keys() {
            // Tryout seating this person next to anyone who is not yet seated
            if job.seated.contains(&person) {
                continue;
            }

            // Seat these two together, modify score
            let score = job.score + day.people[job.person][person] + day.people[person][job.person];
            let mut seated = job.seated.clone();
            seated.push(person);
            jobs.push(Work {
                person,
                seated,
                score,
            });
            started_work = true;
        }

        if !started_work {
            // All done, need to get score for last person seated next to first person
            let score = job.score
                + day.people[job.person][&first_person]
                + day.people[&first_person][job.person];

            // Save score if a new best score
            high_score = std::cmp::max(high_score, score);
        }
    }

    high_score
}

impl Puzzle for Day13 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut day = Day13 {
            people: HashMap::new(),
        };

        for line in input.lines() {
            // Input looks like
            // Alice would lose 57 happiness units by sitting next to Bob.
            let clean = line.trim().replace(".", "");
            let splits: Vec<&str> = clean.split(" ").collect();
            let person = splits[0].to_string();
            let sign = if splits[2] == "gain" { 1 } else { -1 };
            let value = find_val::<i32>(line) * sign;
            let guest = splits.last().unwrap().to_string();
            log::debug!("Add {person} thinks {value} of {guest}");
            let entry = day.people.entry(person).or_default();
            entry.insert(guest, value);
        }

        log::debug!("{:#?}", day.people);

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        // Find best seating arrangement
        let answer = find_best_seating(self);
        Ok(answer.to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(330.to_string()),
            false => Some(618.to_string()),
        }
    }

    fn solve_part2(&mut self) -> Result<String> {
        // Add me to the party, with score 0, and find the best arrangement
        let me = "MrMark".to_string();
        let persons: Vec<String> = self.people.keys().map(|p| p.clone()).collect();
        let my_entry = self.people.entry(me.clone()).or_default();
        for person in persons {
            my_entry.insert(person, 0);
        }
        for (_, entry) in self.people.iter_mut() {
            entry.insert(me.clone(), 0);
        }

        let answer = find_best_seating(self);
        Ok(answer.to_string())
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(286.to_string()),
            false => Some(601.to_string()),
        }
    }
}
