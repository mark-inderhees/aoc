// 2022 Day 23
// https://adventofcode.com/2022/day/23
// --- Day 23: Unstable Diffusion ---
// Elves are looking where to plan star fruit trees
// They need to spread out (diffuse)

use anyhow::Result;
use std::collections::HashMap;
use std::collections::VecDeque;

use crate::puzzle::Puzzle;
use crate::utils::board::*;

pub struct Day23 {
    board: Board<char>,
}

// There are a whole bunch of specific rules on how the game works.
// Just need to read the details and follow correctly. No optimizations.
fn play(day: &mut Day23, rounds: u32) -> u32 {
    struct ElfMove {
        location: BoardPoint,
        elf_id: PlayerId,
        direction: Direction,
    }
    // Check if elf has no one next to them, then that elf DOES NOTHING

    // Propose one of these thoughts:
    // Look N, NE, or NW, move N
    // Look S, SE, or SW, move S
    // Look W, NW, or SW, move W
    // Look E, NE, or SE, move E
    let mut proposals = VecDeque::from(vec![
        vec![Direction::Up, Direction::UpRight, Direction::UpLeft],
        vec![Direction::Down, Direction::DownRight, Direction::DownLeft],
        vec![Direction::Left, Direction::UpLeft, Direction::DownLeft],
        vec![Direction::Right, Direction::UpRight, Direction::DownRight],
    ]);
    let mut proposal_choice = VecDeque::from(vec![
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ]);

    // If no conflict, then move. If conflict, then neither moves!
    // Then rotate the order of the proposed preferences
    // Repeate 10x (for part 1)

    let step_offsets = HashMap::from([
        (Direction::Up, BoardPoint { x: 0, y: -1 }),
        (Direction::Down, BoardPoint { x: 0, y: 1 }),
        (Direction::Left, BoardPoint { x: -1, y: 0 }),
        (Direction::Right, BoardPoint { x: 1, y: 0 }),
        (Direction::UpLeft, BoardPoint { x: -1, y: -1 }),
        (Direction::UpRight, BoardPoint { x: 1, y: -1 }),
        (Direction::DownLeft, BoardPoint { x: -1, y: 1 }),
        (Direction::DownRight, BoardPoint { x: 1, y: 1 }),
    ]);

    // Get started with rounds
    for round in 0..rounds {
        log::debug!("Start of round {round}");
        let mut elves = vec![];
        for elf_id in 0..day.board.get_players_len() {
            // Check if elf has no one next to them, then that elf DOES NOTHING
            if day.board.is_any_player_nearby(elf_id) {
                elves.push(elf_id);
            }
        }

        // Propose one of the thoughts, if the thought is good then add the thought to the list
        // If no thoughts are good, then do nothing
        let mut elf_proposals = vec![];
        for elf_id in elves {
            let elf_location = day.board.get_player_location(elf_id);
            for (i, proposal) in proposals.iter().enumerate() {
                let mut good = true;
                for direction in proposal {
                    let offset = step_offsets[direction];
                    let location = BoardPoint {
                        x: elf_location.x + offset.x,
                        y: elf_location.y + offset.y,
                    };
                    if day.board.is_player_here(location) {
                        good = false;
                        break;
                    }
                }

                // This is a good thought
                if good {
                    let good_direction = proposal_choice[i];
                    let good_offset = step_offsets[&good_direction];
                    let good_location = BoardPoint {
                        x: elf_location.x + good_offset.x,
                        y: elf_location.y + good_offset.y,
                    };
                    elf_proposals.push(ElfMove {
                        location: good_location,
                        elf_id,
                        direction: good_direction,
                    });
                    log::debug!("Elf {} proposes moving {:?}", elf_id, good_direction);
                    break;
                }
            }
        }

        // Filter out duplicate moves, use a hash map to find collisions
        let mut elf_move_map: HashMap<BoardPoint, PlayerId> = HashMap::new();
        let mut moves_to_skip = vec![];
        for proposal in &elf_proposals {
            if elf_move_map.contains_key(&proposal.location) {
                moves_to_skip.push(proposal.location);
            }
            elf_move_map.insert(proposal.location, proposal.elf_id);
        }
        let moves_to_do: Vec<&ElfMove> = elf_proposals
            .iter()
            .filter(|v| !moves_to_skip.contains(&v.location))
            .collect();

        // If no conflict, then move. If conflict, then neither moves!
        for work in &moves_to_do {
            day.board.set_player_location(work.elf_id, work.location);
            log::debug!("Elf {} moved {:?}", work.elf_id, work.direction);
        }

        if moves_to_do.len() == 0 {
            log::debug!("All done on round {round}");
            return round + 1;
        }

        // Then rotate the order of the proposed preferences
        let p = proposals.pop_front().unwrap();
        proposals.push_back(p);
        let pc = proposal_choice.pop_front().unwrap();
        proposal_choice.push_back(pc);

        log::debug!("Round {round} done");
    }

    return 0;
}

impl Puzzle for Day23 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut day = Day23 {
            board: Board::new(),
        };

        // Find the size of the initial bozrd
        let mut width = 0;
        for line in input.lines() {
            width = std::cmp::max(width, line.chars().count());
        }

        let test = width < 20;

        // Add some extra rows. This is not actually needed as we do not step players
        // But is useful for printing the debug game board to screen
        let num_extra = match test {
            true => 15,
            false => 50,
        };
        let full_line = vec!['.'; num_extra * 2 + width];
        for _ in 0..num_extra {
            day.board.push_row(full_line.clone());
        }

        // Now build the game board with some extra space
        let extra_chars = vec!['.'; num_extra];
        for line in input.lines() {
            let chars: Vec<char> = line.chars().collect();
            let mut row = extra_chars.clone();
            row.extend(chars);
            row.extend(extra_chars.clone());
            day.board.push_row(row);
        }

        // Extra space at bottom
        for _ in 0..num_extra {
            day.board.push_row(full_line.clone());
        }

        day.board.add_players_from_value('#', '.');
        log::debug!("Players {}", day.board.get_players_len());

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        // Move 10 rounds, then find size of elf grid
        play(self, 10);
        let mut x_min = i32::MAX;
        let mut y_min = i32::MAX;
        let mut x_max = 0;
        let mut y_max = 0;
        for player in 0..self.board.get_players_len() {
            let location = self.board.get_player_location(player);
            x_min = std::cmp::min(x_min, location.x);
            y_min = std::cmp::min(y_min, location.y);
            x_max = std::cmp::max(x_max, location.x);
            y_max = std::cmp::max(y_max, location.y);
        }

        let width = x_max - x_min + 1;
        let height = y_max - y_min + 1;
        let spots = width * height - self.board.get_players_len() as i32;
        Ok(spots.to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(110.to_string()),
            false => Some(4138.to_string()),
        }
    }

    fn solve_part2(&mut self) -> Result<String> {
        // Find how many rounds it takes to stop moving
        let rounds = play(self, u32::MAX);
        Ok(rounds.to_string())
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(20.to_string()),
            false => Some(1010.to_string()),
        }
    }
}
