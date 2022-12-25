use std::collections::HashMap;
use std::path::PathBuf;

/// A file system, contains directories and files. As a pwd that changes with commands.
pub struct FileSystem {
    directories: HashMap<PathBuf, Directory>,
    files: HashMap<PathBuf, File>,
    pwd: PathBuf,
}

/// A simple file, it has a name and size.
pub struct File {
    pub name: String,
    pub size: u32,
}

/// A directory, has a name and can hold other directories or files.
/// The size is the sum of all files under this directory.
pub struct Directory {
    pub name: String,
    pub size: u32,
    directories: Vec<String>,
    files: Vec<File>,
}

impl FileSystem {
    /// Create a new file system with one root folder.
    pub fn new() -> FileSystem {
        FileSystem {
            // Init a single directory at root
            directories: HashMap::from([(
                PathBuf::from("/"),
                Directory {
                    name: "/".to_string(),
                    size: 0,
                    directories: vec![],
                    files: vec![],
                },
            )]),
            files: HashMap::from([]),
            // Set present working directory as root
            pwd: PathBuf::from("/"),
        }
    }

    /// cd to /
    pub fn change_directory_to_root(&mut self) {
        self.pwd = PathBuf::from("/");
    }

    /// cd to <name>. This appends to pwd.
    pub fn change_directory(&mut self, name: &str) {
        self.pwd.push(name);
    }

    /// cd ..
    pub fn change_directory_parent(&mut self) {
        self.pwd.pop();
    }

    /// Add a file to the tree. This increases the directory size of all parents.
    pub fn add_file(&mut self, name: &str, size: u32) {
        // Add file in the list of files for this directory
        let directory = self.directories.get_mut(&self.pwd).unwrap();
        directory.files.push(File {
            name: name.to_string(),
            size,
        });

        // Add a new file into the file system
        self.files.insert(
            self.pwd.join(name),
            File {
                name: name.to_string(),
                size: 0,
            },
        );

        // Increase the size of directories in this tree
        for ancestor in self.pwd.ancestors() {
            self.directories.get_mut(ancestor).unwrap().size += size;
        }
    }

    /// Add a new directory into the file system.
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
                name: name.to_string(),
                size: 0,
                directories: vec![],
                files: vec![],
            },
        );
    }

    /// Iterate over all directories in the system.
    pub fn iter_directories(&self) -> std::collections::hash_map::Values<'_, PathBuf, Directory> {
        self.directories.values()
    }

    /// Get the size of the current directory.
    pub fn get_size(&self, path: &PathBuf) -> u32 {
        self.directories[path].size
    }
}
