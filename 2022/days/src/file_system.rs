use std::{collections::HashMap, path::PathBuf};

#[allow(dead_code)]
struct File {
    size: u32,
    name: String,
}

pub struct Folder {
    pub size: u32,
    pub name: String,
    files: Vec<File>,
    pub dirs: Vec<String>,
}

pub struct FileSystem {
    pub folders: HashMap<PathBuf, Folder>,
    pub pwd: PathBuf,
}

impl FileSystem {
    pub fn new() -> FileSystem {
        FileSystem {
            // Init a single directory at root
            folders: HashMap::from([(
                PathBuf::from("/"),
                Folder {
                    size: 0,
                    name: "/".to_string(),
                    files: vec![],
                    dirs: vec![],
                },
            )]),
            // Set present working directory as root
            pwd: PathBuf::from("/"),
        }
    }

    pub fn add_file(&mut self, name: &str, size: u32) {
        // Add file in the list of files for this directory
        let folder = self.folders.get_mut(&self.pwd).unwrap();
        folder.files.push(File {
            size,
            name: name.to_string(),
        });

        // Increase the size of this folder
        folder.size += size;
    }

    pub fn add_directory(&mut self, name: &str) {
        // Add directory name in the list of directories for this directory
        self.folders
            .get_mut(&self.pwd)
            .unwrap()
            .dirs
            .push(name.to_string());

        // Add a new folder into the file system
        self.folders.insert(
            self.pwd.join(name),
            Folder {
                size: 0,
                name: name.to_string(),
                files: vec![],
                dirs: vec![],
            },
        );
    }
}
