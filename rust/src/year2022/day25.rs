// 2022 Day 25
// https://adventofcode.com/2022/day/25
// --- Day 25: Full of Hot Air ---
// Convert a number to and from base 5
// But it is a funky base 5 with negative numbers!
// To balance out the negatives, need to add one to next higher digit, then
// subtract 1 or 2 to get the intended target.

use anyhow::Result;
use std::collections::VecDeque;

use crate::puzzle::Puzzle;

pub struct Day25 {
    snafus: Vec<Snafu>,
}

#[derive(Debug)]
struct Snafu {
    value: Vec<char>,
    decimal: i64,
}

fn determine_snafu_value(char: &char) -> i64 {
    match char {
        '2' => 2,
        '1' => 1,
        '0' => 0,
        '-' => -1,
        '=' => -2,
        _ => panic!("Unexpected snafu char"),
    }
}

fn snafu_to_decimal(snafu: &Vec<char>) -> i64 {
    let mut pow = 0;
    let mut decimal = 0;
    let mut list2 = snafu.clone();
    list2.reverse();
    for char in list2 {
        let value = determine_snafu_value(&char);
        decimal += 5i64.pow(pow) * value;
        pow += 1;
    }
    decimal
}

fn decimal_to_snafu(decimal: i64) -> Vec<char> {
    // Normally would convert decimal to base 5 by
    // while decimal > 0
    //   let target = decimal % 5
    //   answer.push_front(target)
    //   decimal = decimal / 5
    //
    // But now, to make target for 3 and 4, we need to
    // increase the next value +1 and then subtract -1 or -2 here

    let mut snafu = VecDeque::new();
    let mut decimal2 = decimal;

    while decimal2 > 0 {
        // Find target and turn into a char
        let target = decimal2 % 5;
        let mut target_char = target.to_string().chars().collect::<Vec<char>>()[0];

        // To make 3 or 4, we need to use negative numbers.
        // Add 5 to the total so the next digit is 1 higher.
        // Then subtract 1 or 2 to make 4 or 3 respectively.
        if target == 3 {
            decimal2 += 5;
            target_char = '='; // Subtract 2 to make 3
        } else if target == 4 {
            decimal2 += 5;
            target_char = '-'; // Subtract 1 to make 4
        }

        snafu.push_front(target_char);
        decimal2 = decimal2 / 5;
    }

    log::debug!("Converted {decimal} into {snafu:?}");

    snafu.into()
}

fn test_conversion() {
    // Some extra tests for the conversion logic
    let tests = [
        (1, "1"),
        (2, "2"),
        (3, "1="),
        (4, "1-"),
        (5, "10"),
        (6, "11"),
        (7, "12"),
        (8, "2="),
        (9, "2-"),
        (10, "20"),
        (15, "1=0"),
        (20, "1-0"),
        (2022, "1=11-2"),
        (12345, "1-0---0"),
        (314159265, "1121-1110-1=0"),
    ];

    for (decimal, snafu_str) in tests {
        let snafu_expected: Vec<char> = snafu_str.chars().collect();
        assert_eq!(snafu_expected, decimal_to_snafu(decimal));
    }
}

impl Puzzle for Day25 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut day = Day25 { snafus: vec![] };

        // Inputs are a bunch of snafus, save them
        for line in input.lines() {
            let snafu = Snafu {
                value: line.chars().collect(),
                decimal: 0,
            };
            day.snafus.push(snafu);
        }

        // Find the decimal value for the snafu inputs
        for snafu in &mut day.snafus {
            snafu.decimal = snafu_to_decimal(&snafu.value);
        }

        log::debug!("Snafus: {:#?}", day.snafus);
        test_conversion();

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        // Sum all of the decimal values then convert to snafu
        let sum = self.snafus.iter().fold(0, |a, x| a + x.decimal);
        log::debug!("Sum is {sum}");
        let snafu = decimal_to_snafu(sum);
        let snafu_string: String = snafu.iter().collect();
        Ok(snafu_string)
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some("2=-1=0".to_string()),
            false => Some("121=2=1==0=10=2-20=2".to_string()),
        }
    }

    fn solve_part2(&mut self) -> Result<String> {
        // There is no part 2!
        Ok(12.to_string())
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(12.to_string()),
            false => Some(12.to_string()),
        }
    }
}
