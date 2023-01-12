// 2015 Day 22
// https://adventofcode.com/2015/day/22
// --- Day 22: Wizard Simulator 20XX ---
// Fight an RPG battle like day 21. But now I'm a wizzard that casts spells and
// has no armor. This uses path finding now.

use anyhow::Result;

use crate::puzzle::Puzzle;
use crate::utils::utils::*;

pub struct Day22 {
    boss: Boss,
    me: Player,
    spells: Vec<Spell>,
}

/// An effect is a spell that lasts multiple turns
#[derive(Debug, Clone)]
struct Effect {
    name: String,
    time_left: u32,
    armor: i32,
    damage: i32,
    mana_recharge: u32,
}

#[derive(Debug, Clone)]
struct Boss {
    hit_points: i32,
    damage: i32,
}

#[derive(Debug, Clone)]
struct Player {
    hit_points: i32,
    mana: u32,
    mana_spent: u32,
    effects: Vec<Effect>,
}

#[derive(Default, Debug, Clone)]
struct Spell {
    name: String,
    effect: bool,
    effect_length: u32,
    mana: u32,
    damage: i32,
    heal: i32,
    armor: i32,
    mana_recharge: u32,
}

/// Find the win that results in using the least amount of mana.
/// For hard mode, I take 1hp damage extra each round.
fn find_cheapest_win(day: &Day22, hard_mode: bool) -> u32 {
    // This is path finding
    // I wear no armor, but I have spell effects that give armor
    // Boss has no armor
    // The boss always does at least 1 damage
    // I must cast a spell every round, if I have no mana then I lose
    // Effects happen at the start of both my and boss turns
    // I cannot cast an effect that is already running
    // In hard mode, I take one damage at the very start of my turn

    #[derive(Debug, Clone)]
    struct Work {
        me: Player,
        boss: Boss,
        my_turn: bool,
    }
    let mut jobs: Vec<Work> = vec![Work {
        me: day.me.clone(),
        boss: day.boss.clone(),
        my_turn: true,
    }];

    let mut min_mana_spent = u32::MAX;

    while jobs.len() > 0 {
        let mut job = jobs.pop().unwrap();

        // For hard mode, player loses 1hp each round
        if job.my_turn && hard_mode {
            job.me.hit_points -= 1;

            // Am I dead?
            if job.me.hit_points <= 0 {
                continue;
            }
        }

        // Apply effects
        for effect in job.me.effects.iter_mut() {
            effect.time_left -= 1;
            job.boss.hit_points -= effect.damage; // Poison damage
            job.me.mana += effect.mana_recharge; // Reacharge gain mana
        }

        // Remove expired effects
        job.me.effects.retain(|x| x.time_left > 0);

        // Is boss dead?
        if job.boss.hit_points <= 0 {
            log::debug!("Boss is dead!");
            min_mana_spent = std::cmp::min(min_mana_spent, job.me.mana_spent);
            continue;
        }

        // Is this a bad path?
        if job.me.mana_spent > min_mana_spent {
            continue;
        }

        // Do a player turn
        if job.my_turn {
            // Cast a new spell
            'spell_loop: for spell in day.spells.iter() {
                // Check if this spell effect is already running
                if spell.effect {
                    for effect in job.me.effects.iter() {
                        if spell.name == effect.name {
                            // We already have this spell running, skip it
                            continue 'spell_loop;
                        }
                    }
                }

                // Check if I have enough mana to cast this spell
                if spell.mana > job.me.mana {
                    continue;
                }

                // Start a new job
                let mut new_job = job.clone();
                new_job.my_turn = false;

                // Deduct cost of spell
                new_job.me.mana -= spell.mana;
                new_job.me.mana_spent += spell.mana;

                if spell.effect {
                    // Add the effect to our list
                    new_job.me.effects.push(Effect {
                        name: spell.name.clone(),
                        time_left: spell.effect_length,
                        armor: spell.armor,
                        damage: spell.damage,
                        mana_recharge: spell.mana_recharge,
                    });
                } else {
                    // Apply the spell immediately
                    new_job.me.hit_points += spell.heal;
                    new_job.boss.hit_points -= spell.damage;

                    // Is the boss dead?
                    if new_job.boss.hit_points <= 0 {
                        log::debug!("Boss is dead!");
                        min_mana_spent = std::cmp::min(min_mana_spent, new_job.me.mana_spent);
                        continue;
                    }
                }

                // Schedule the job
                jobs.push(new_job);
            }
        } else {
            // Boss attacks, my armor only comes from effects
            let my_armor = job.me.effects.iter().fold(0, |a, x| a + x.armor);

            // Attack always does at least 1 damage
            let attack = std::cmp::max(1, job.boss.damage - my_armor);
            job.me.hit_points -= attack;

            // Am I dead?
            if job.me.hit_points <= 0 {
                continue;
            }

            // Reschedule the job
            job.my_turn = true;
            jobs.push(job);
        }
    }

    min_mana_spent
}

impl Puzzle for Day22 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        // Input is simple stats for the boss
        let lines: Vec<&str> = input.trim().lines().collect();
        let hit_points = find_val(lines[0]);
        let damage = find_val(lines[1]);

        #[allow(unused_mut)]
        let mut day = Day22 {
            boss: Boss { hit_points, damage },
            me: Player {
                hit_points: 50,
                mana: 500,
                mana_spent: 0,
                effects: vec![],
            },
            spells: vec![],
        };

        if day.boss.hit_points < 20 {
            // This is test case
            day.me.hit_points = 10;
            day.me.mana = 250;
        }

        // Spells are staticaly defined
        day.spells.push(Spell {
            name: "Magic Missle".to_string(),
            effect: false,
            mana: 53,
            damage: 4,
            ..Default::default()
        });
        day.spells.push(Spell {
            name: "Drain".to_string(),
            effect: false,
            mana: 73,
            damage: 2,
            heal: 2,
            ..Default::default()
        });
        day.spells.push(Spell {
            name: "Shield".to_string(),
            effect: true,
            effect_length: 6,
            mana: 113,
            armor: 7,
            ..Default::default()
        });
        day.spells.push(Spell {
            name: "Poison".to_string(),
            effect: true,
            effect_length: 6,
            mana: 173,
            damage: 3,
            ..Default::default()
        });
        day.spells.push(Spell {
            name: "Recharge".to_string(),
            effect: true,
            effect_length: 5,
            mana: 229,
            mana_recharge: 101,
            ..Default::default()
        });

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        // Find least amount of mana needed to win in normal mode
        let answer = find_cheapest_win(self, false);
        Ok(answer.to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(226.to_string()),
            false => Some(900.to_string()),
        }
    }

    fn solve_part2(&mut self) -> Result<String> {
        // Find least amount of mana needed to win in hard mode
        let answer = find_cheapest_win(self, true);
        Ok(answer.to_string())
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(u32::MAX.to_string()), // There were no test cases, so this is lame
            false => Some(1216.to_string()),
        }
    }
}
