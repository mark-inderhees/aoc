// 2022 Day 07
// https://adventofcode.com/2022/day/7
// --- Day 7: No Space Left On Device ---
// Build a file system! Need to navigate the system to build it up. Then get sizes.

use anyhow::Result;
use std::path::PathBuf;

use crate::utils::file_system::*;
use crate::puzzle::Puzzle;

pub struct Day07 {
    file_system: FileSystem,
}

enum CommandType {
    CdRoot,
    CdBack,
    CdDir(String),
    Ls,
}

fn determine_command_type(line: &str) -> CommandType {
    match line {
        "$ cd /" => CommandType::CdRoot,
        "$ cd .." => CommandType::CdBack,
        "$ ls" => CommandType::Ls,
        _ => CommandType::CdDir(line.split(" ").last().unwrap().to_string()),
    }
}

impl Puzzle for Day07 {
    #[allow(unused_variables)]
    fn from_input(input: &str) -> Result<Self> {
        #[allow(unused_mut)]
        let mut day = Day07 {
            file_system: FileSystem::new(),
        };

        // Input is the output from walking around a file system
        for line in input.lines() {
            if line.starts_with("$") {
                // Command
                match determine_command_type(line) {
                    CommandType::CdRoot => day.file_system.change_directory_to_root(),
                    CommandType::CdDir(name) => day.file_system.change_directory(&name),
                    CommandType::CdBack => day.file_system.change_directory_parent(),
                    CommandType::Ls => (),
                }
            } else if line.starts_with("dir") {
                // Listing a directory, like
                // dir <dirname>
                let name = line.split(" ").last().unwrap();
                day.file_system.add_directory(name);
            } else {
                // Listing a file, like
                // <size> <filename>
                let mut line2 = line.split(" ");
                let size = line2.next().unwrap().parse::<u32>()?;
                let name = line2.last().unwrap();
                day.file_system.add_file(name, size)
            }
        }

        day.file_system.print();

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        let mut sum = 0;
        // Find directories less than 100kb
        // Sum their sizes
        for directory in self.file_system.iter_directories() {
            if directory.size <= 100_000 {
                sum += directory.size
            }
        }
        Ok(sum.to_string())
    }

    fn answer_part1(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(95437.to_string()),
            false => Some(1581595.to_string()),
        }
    }

    fn solve_part2(&mut self) -> Result<String> {
        // Find smallest folder to delete to free needed space
        let available = 70_000_000 - self.file_system.current_directory_size(&PathBuf::from("/"));
        let needed = 30_000_000 - available;
        let mut directories = self
            .file_system
            .iter_directories()
            .collect::<Vec<&Directory>>();

        // Sort smallest to largest, first find is our answer
        directories.sort_by(|a, b| a.size.cmp(&b.size));
        let mut value = 0;
        for directory in directories {
            if directory.size > needed {
                value = directory.size;
                break;
            }
        }
        Ok(value.to_string())
    }

    fn answer_part2(&mut self, test: bool) -> Option<String> {
        match test {
            true => Some(24933642.to_string()),
            false => Some(1544176.to_string()),
        }
    }
}
