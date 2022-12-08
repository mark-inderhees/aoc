use std::{collections::HashMap, path::PathBuf};

pub struct File {
    pub size: u32,
    pub name: String,
}

pub struct Directory {
    pub size: u32,
    pub name: String,
    files: Vec<File>,
    directories: Vec<String>,
}

pub struct FileSystem {
    pub directories: HashMap<PathBuf, Directory>,
    pwd: PathBuf,
}

impl FileSystem {
    pub fn new() -> FileSystem {
        FileSystem {
            // Init a single directory at root
            directories: HashMap::from([(
                PathBuf::from("/"),
                Directory {
                    size: 0,
                    name: "/".to_string(),
                    files: vec![],
                    directories: vec![],
                },
            )]),
            // Set present working directory as root
            pwd: PathBuf::from("/"),
        }
    }

    pub fn change_directory_to_root(&mut self) {
        self.pwd = PathBuf::from("/");
    }

    pub fn change_directory(&mut self, name: &str) {
        self.pwd.push(name);
    }

    pub fn change_directory_parent(&mut self) {
        self.pwd.pop();
    }

    pub fn add_file(&mut self, name: &str, size: u32) {
        // Add file in the list of files for this directory
        let directory = self.directories.get_mut(&self.pwd).unwrap();
        directory.files.push(File {
            size,
            name: name.to_string(),
        });

        // Increase the size of directories in this tree
        for ancestor in self.pwd.ancestors() {
            self.directories.get_mut(ancestor).unwrap().size += size;
        }
    }

    pub fn add_directory(&mut self, name: &str) {
        // Add directory name in the list of directories for this directory
        self.directories
            .get_mut(&self.pwd)
            .unwrap()
            .directories
            .push(name.to_string());

        // Add a new directory into the file system
        self.directories.insert(
            self.pwd.join(name),
            Directory {
                size: 0,
                name: name.to_string(),
                files: vec![],
                directories: vec![],
            },
        );
    }

    pub fn iter_directories(&self) -> std::collections::hash_map::Values<'_, PathBuf, Directory> {
        self.directories.values()
    }

    pub fn get_size(&self, path: &PathBuf) -> u32 {
        self.directories[path].size
    }
}
