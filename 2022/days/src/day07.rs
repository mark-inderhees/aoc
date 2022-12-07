use anyhow::Result;
use std::path::PathBuf;

use crate::file_system::*;
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

fn get_command_type(line: &str) -> CommandType {
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

        for line in input.lines() {
            if line.starts_with("$") {
                // Command
                let command = get_command_type(line);
                match command {
                    CommandType::CdRoot => day.file_system.change_directory_to_root(),
                    CommandType::CdDir(name) => day.file_system.change_directory(&name),
                    CommandType::CdBack => day.file_system.change_directory_parent(),
                    CommandType::Ls => (),
                }
            } else if line.starts_with("dir") {
                // Listing a directory
                let name = line.split(" ").last().unwrap();
                day.file_system.add_directory(name);
            } else {
                // Listing a file
                let mut line2 = line.split(" ");
                let size = line2.next().unwrap().parse::<u32>()?;
                let name = line2.last().unwrap();
                day.file_system.add_file(name, size)
            }
        }

        let mut keys: Vec<PathBuf> = vec![];
        for key in day.file_system.folders.keys() {
            keys.push(key.to_path_buf());
        }
        keys.sort_by(|a, b| b.components().count().cmp(&a.components().count()));
        for key in &keys {
            let folder = &day.file_system.folders[key];
            let dirs = day.file_system.folders[key].dirs.clone();
            for subdir in dirs {
                let subdir_name = key.join(subdir);
                let size = day.file_system.folders[&subdir_name].size;
                day.file_system.folders.get_mut(key).unwrap().size += size;
            }
            let size = day.file_system.folders[key].size;
        }

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        let mut sum = 0;
        for (_, folder) in self.file_system.folders.iter() {
            if folder.size <= 100000 {
                sum += folder.size
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
        let available = 70000000 - self.file_system.folders[&PathBuf::from("/")].size;
        let needed = 30000000 - available;
        let mut folders = self
            .file_system
            .folders
            .iter()
            .collect::<Vec<(&PathBuf, &Folder)>>();
        folders.sort_by(|a, b| a.1.size.cmp(&b.1.size));
        let mut value = 0;
        for (_, folder) in folders {
            if folder.size > needed {
                value = folder.size;
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
