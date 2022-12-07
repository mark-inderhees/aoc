use anyhow::Result;
use std::{collections::HashMap, path::PathBuf};

use crate::puzzle::Puzzle;

pub struct Day07 {
    folders: HashMap<PathBuf, Folder>,
}

#[allow(dead_code)]
struct File {
    size: u32,
    name: String,
}

#[allow(dead_code)]
struct Folder {
    size: u32,
    name: String,
    files: Vec<File>,
    dirs: Vec<String>,
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
            folders: HashMap::from([(
                PathBuf::from("/"),
                Folder {
                    size: 0,
                    name: "/".to_string(),
                    files: vec![],
                    dirs: vec![],
                },
            )]),
        };

        let mut current = PathBuf::from("/");

        for line in input.lines() {
            if line.starts_with("$") {
                let command = get_command_type(line);
                match command {
                    CommandType::CdRoot => current = PathBuf::from("/"),
                    CommandType::CdDir(name) => current.push(name),
                    CommandType::CdBack => {
                        current.pop();
                    }
                    CommandType::Ls => (),
                }
            } else if line.starts_with("dir") {
                let name = line.split(" ").last().unwrap();
                day.folders.insert(
                    current.join(name),
                    Folder {
                        size: 0,
                        name: name.to_string(),
                        files: vec![],
                        dirs: vec![],
                    },
                );
                day.folders
                    .get_mut(&current)
                    .unwrap()
                    .dirs
                    .push(name.to_string());
            } else {
                // File
                let mut line2 = line.split(" ");
                let size = line2.next().expect(" ").parse::<u32>()?;
                let name = line2.last().expect(" ").to_string();
                let folder = day.folders.get_mut(&current).unwrap();
                folder.files.push(File { size, name });
                folder.size += size;
            }
        }

        let mut keys: Vec<PathBuf> = vec![];
        for key in day.folders.keys() {
            keys.push(key.to_path_buf());
        }
        keys.sort_by(|a, b| a.components().count().cmp(&b.components().count()));
        keys.reverse();
        for key in &keys {
            let folder = &day.folders[key];
            let dirs = day.folders[key].dirs.clone();
            for subdir in dirs {
                let subdir_name = key.join(subdir);
                let size = day.folders[&subdir_name].size;
                day.folders.get_mut(key).unwrap().size += size;
            }
            let size = day.folders[key].size;
        }

        Ok(day)
    }

    fn solve_part1(&mut self) -> Result<String> {
        let mut sum = 0;
        for (_, folder) in self.folders.iter() {
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
        let available = 70000000 - self.folders[&PathBuf::from("/")].size;
        let needed = 30000000 - available;
        let mut folders = self.folders.iter().collect::<Vec<(&PathBuf, &Folder)>>();
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
