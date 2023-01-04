// 2015 Day 11
// https://adventofcode.com/2015/day/11
// --- Day 11: Corporate Policy ---
// Find a new password while following silly rules

use anyhow::Result;

use crate::puzzle::Puzzle;

pub struct Day11 {
    current_password: String,
}

/// Incrementing like ay -> az -> ba until valid password found
fn increment_password(password: &mut Vec<char>) {
    // Increment with roll over
    let mut i = password.len() - 1;
    loop {
        let mut value = password[i] as u8 + 1;

        // Also skip i, o, or l as those are bad passwords
        if value == 'i' as u8 || value == 'o' as u8 || value == 'l' as u8 {
            value += 1;
        }

        if value <= 'z' as u8 {
            // No roll over, done with increment
            password[i] = value as char;
            break;
        }

        // Roll over detected, keep going so next char is incremented
        password[i] = 'a';
        i -= 1;
    }
}

/// Find next password to use based on current password
/// Keep incrementing like ay -> az -> ba until valid password found
/// Must have one three char straight like "bcd"
/// Cannont contain i, o, or l
/// Must contain two unique pairs, like aa and jj
fn find_next_password(current_password: &str) -> String {
    let mut chars: Vec<char> = current_password.chars().collect();

    loop {
        increment_password(&mut chars);

        // Must have one three char straight like "bcd"
        let mut has_straight = false;
        for i in 0..chars.len() - 2 {
            let char1 = chars[i] as u8;
            let char2 = chars[i + 1] as u8;
            let char3 = chars[i + 2] as u8;
            if char1 == char2 - 1 && char1 == char3 - 2 {
                has_straight = true;
                break;
            }
        }
        if !has_straight {
            continue;
        }

        // Must contain two unique pairs, like aa and jj
        let mut pair_count = 0;
        let mut first_pair = ' ';
        for i in 0..chars.len() - 1 {
            let char = chars[i];

            // If we already found this pair, skip
            if char == first_pair {
                continue;
            }

            // Check if this is a double pair
            let next_char = chars[i + 1];
            if char == next_char {
                pair_count += 1;

                // Check if this is second pair
                if pair_count == 2 {
                    break;
                }
                first_pair = char;
            }
        }
        if pair_count != 2 {
            continue;
        }

        // We got passed all checks, end the loop
        break;
    }

    chars.iter().collect()
}

impl Puzzle for Day11 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut day = Day11 {
            current_password: input.trim().to_string(),
        };

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        // Find next password
        let answer = find_next_password(&self.current_password);
        Ok(answer)
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some("abcdffaa".to_string()),
            false => Some("hxbxxyzz".to_string()),
        }
    }

    fn solve_part2(&mut self) -> Result<String> {
        // Find second next password
        let mut answer = find_next_password(&self.current_password);
        answer = find_next_password(&answer);
        Ok(answer)
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some("abcdffbb".to_string()),
            false => Some("hxcaabcc".to_string()),
        }
    }
}
