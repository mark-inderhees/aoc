// 2015 Day 21
// https://adventofcode.com/2015/day/21
// --- Day 21: RPG Simulator 20XX ---
// Play a basic RPG game with attack and defensive points. Who dies?
// What is best equipment combo for cost?

use anyhow::Result;
use itertools::Itertools;

use crate::puzzle::Puzzle;
use crate::utils::utils::*;

pub struct Day21 {
    me: Player,
    boss: Player,
    weapons: Vec<Item>,
    armor: Vec<Item>,
    rings: Vec<Item>,
}

/// An item in the shop
#[derive(Debug, Clone)]
struct Item {
    #[allow(dead_code)]
    name: String,
    cost: u32,
    damage: u32,
    armor: u32,
}

/// Stats of all the items the player has equipped
#[derive(Debug, Clone)]
struct Stats {
    damage: u32,
    armor: u32,
    cost: u32,
}

#[derive(Debug, Clone)]
struct Player {
    hit_points: u32,
    stats: Stats,
}

/// Given a player and a boss, who won this duel?
fn did_i_win(me: &Player, boss: &Player) -> bool {
    // A player losses at least 1 HP per round
    let my_loss_per_round = std::cmp::max(
        1,
        boss.stats.damage.checked_sub(me.stats.armor).unwrap_or(0),
    );
    let boss_loss_per_round = std::cmp::max(
        1,
        me.stats.damage.checked_sub(boss.stats.armor).unwrap_or(0),
    );

    // Do ceiling division as need to go to the next round on fractional round
    let my_death_round = (me.hit_points + my_loss_per_round - 1) / my_loss_per_round;
    let boss_death_round = (boss.hit_points + boss_loss_per_round - 1) / boss_loss_per_round;

    // Player attacks first, so player wins on equal round
    let result = my_death_round >= boss_death_round;

    log::debug!(
        "Result {result}. Round {my_death_round} vs {boss_death_round}. Me {me:?} vs boss {boss:?}"
    );

    result
}

/// Given a bunch of items, what is the sumed stats.
fn calc_stats(weapon: &Item, armor: &Vec<&Item>, rings: &Vec<&Item>) -> Stats {
    let mut stats = Stats {
        damage: weapon.damage,
        armor: weapon.armor,
        cost: weapon.cost,
    };

    for item in armor.iter() {
        stats.damage += item.damage;
        stats.armor += item.armor;
        stats.cost += item.cost;
    }

    for item in rings.iter() {
        stats.damage += item.damage;
        stats.armor += item.armor;
        stats.cost += item.cost;
    }

    stats
}

/// Find both the cheapest way to win and the most expensive way to lose.
fn find_cheapest_win_and_expensive_loss(day: &Day21) -> (u32, u32) {
    // Search all combinations of shop config for which win costs the least
    // And which loss cost the most
    // Must buy one weapon
    // Can by at most one armor, but it's optional
    // Can buy 0 to 2 rings, but no duplicate items
    let mut me = day.me.clone();
    let mut lowest_cost = u32::MAX;
    let mut highest_cost_lost = u32::MIN;

    // Try each weapon
    for weapon in day.weapons.iter() {
        // Try each armor (or zero armor)
        for armor in day.armor.iter() {
            let armor = vec![armor];

            // Try ring combos (0 to 2)
            for ring_len in 1..=2 {
                for rings in day.rings.iter().combinations(ring_len) {
                    // Try this weapon, armor and rings
                    me.stats = calc_stats(weapon, &armor, &rings);
                    if did_i_win(&me, &day.boss) {
                        lowest_cost = std::cmp::min(lowest_cost, me.stats.cost);
                    } else {
                        highest_cost_lost = std::cmp::max(highest_cost_lost, me.stats.cost);
                    }

                    // Try these weapon and rings with no armor
                    me.stats = calc_stats(weapon, &vec![], &rings);
                    if did_i_win(&me, &day.boss) {
                        lowest_cost = std::cmp::min(lowest_cost, me.stats.cost);
                    } else {
                        highest_cost_lost = std::cmp::max(highest_cost_lost, me.stats.cost);
                    }
                }
            }
            // Try this weapon and armor with no rings
            me.stats = calc_stats(weapon, &armor, &vec![]);
            if did_i_win(&me, &day.boss) {
                lowest_cost = std::cmp::min(lowest_cost, me.stats.cost);
            } else {
                highest_cost_lost = std::cmp::max(highest_cost_lost, me.stats.cost);
            }
        }

        // Try this weapon with no armor and no ring
        me.stats = calc_stats(weapon, &vec![], &vec![]);
        if did_i_win(&me, &day.boss) {
            lowest_cost = std::cmp::min(lowest_cost, me.stats.cost);
        } else {
            highest_cost_lost = std::cmp::max(highest_cost_lost, me.stats.cost);
        }
    }

    (lowest_cost, highest_cost_lost)
}

impl Puzzle for Day21 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut day = Day21 {
            me: Player {
                hit_points: 100,
                stats: Stats {
                    damage: 0,
                    armor: 0,
                    cost: 0,
                },
            },
            boss: Player {
                hit_points: 100,
                stats: Stats {
                    damage: 0,
                    armor: 0,
                    cost: 0,
                },
            },
            weapons: vec![],
            armor: vec![],
            rings: vec![],
        };

        // Read in boss stats from input
        let lines: Vec<&str> = input.lines().collect();
        day.boss.hit_points = find_val(lines[0]);
        day.boss.stats.damage = find_val(lines[1]);
        day.boss.stats.armor = find_val(lines[2]);

        if day.boss.hit_points < 100 {
            // This is test case
            day.me.hit_points = 8;
        }

        // The shop is static info
        let mut shop = "
        Weapons:    Cost  Damage  Armor
        Dagger        8     4       0
        Shortsword   10     5       0
        Warhammer    25     6       0
        Longsword    40     7       0
        Greataxe     74     8       0

        Armor:      Cost  Damage  Armor
        Leather      13     0       1
        Chainmail    31     0       2
        Splintmail   53     0       3
        Bandedmail   75     0       4
        Platemail   102     0       5

        Rings:      Cost  Damage  Armor
        Damage_1    25     1       0
        Damage_2    50     2       0
        Damage_3   100     3       0
        Defense_1   20     0       1
        Defense_2   40     0       2
        Defense_3   80     0       3";
        shop = shop.trim();

        // Parse the above shop info
        let item_groups: Vec<&str> = shop.split("\n\n").collect();
        let mut item_lists: Vec<Vec<Item>> = vec![vec![]; 3];
        let mut i = 0;
        for item_group in item_groups.iter() {
            let mut j = 0;
            for line in item_group.lines() {
                j += 1;
                if j == 1 {
                    // Skip the first line as it is the header
                    continue;
                }
                let name = line.split(" ").collect::<Vec<&str>>()[0].to_string();
                let vals: Vec<u32> = find_vals(line);
                let cost = vals[0];
                let damage = vals[1];
                let armor = vals[2];
                item_lists[i].push(Item {
                    name,
                    cost,
                    damage,
                    armor,
                })
            }
            i += 1;
        }

        day.weapons = item_lists[0].clone();
        day.armor = item_lists[1].clone();
        day.rings = item_lists[2].clone();

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        // Find cheapest win
        let answer = find_cheapest_win_and_expensive_loss(self).0;
        Ok(answer.to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(65.to_string()),
            false => Some(121.to_string()),
        }
    }

    fn solve_part2(&mut self) -> Result<String> {
        // Find most expensive loss
        let answer = find_cheapest_win_and_expensive_loss(self).1;
        Ok(answer.to_string())
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(188.to_string()),
            false => Some(201.to_string()),
        }
    }
}
