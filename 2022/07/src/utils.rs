// use std::fmt;

use std::fmt;

pub struct File {
    pub name: String,
    pub size: u64,
}

pub struct Directory {
    pub name: String,
    pub files: Vec<File>,
    pub directories: Vec<Directory>,
}

impl fmt::Debug for Directory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Print the name of the directory
        write!(f, "{}", self.name)?;

        // Print the files in the directory
        for file in &self.files {
            write!(f, "\n\t{}: {} bytes", file.name, file.size)?;
        }

        // Print the subdirectories in the directory
        for dir in &self.directories {
            write!(f, "\n\t{:?}", dir)?;
        }

        Ok(())
    }
}

// impl fmt::Debug for FileTypes {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match *self {
//             FileTypes::One(dir) => write!(f, "Directory"),
//             FileTypes::Two(file) => write!(f, "File"),
//         }
//     }
// }

// impl fmt::Debug for Dir {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.debug_struct("Dir")
//             .field("name", &self)
//             // .field("children", &self.children)
//             .finish()
//     }
// }
