mod utils;
use std::{
    env,
    fs::{self},
    vec,
};
use utils::{Directory, File};

fn read_input_file() -> String {
    let args: Vec<String> = env::args().collect();
    let file_path: &String = &args[1];

    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

fn main() {
    let file: String = read_input_file();
    let mut root: Directory = Directory {
        directories: Vec::new(),
        files: Vec::new(),
        name: String::from("/"),
    };
    let mut current_dir: &mut Directory = &mut root;

    for line in file.lines().skip(1) {
        let command_args: Vec<&str> = line.split_whitespace().collect();
        if command_args[0] == "$" {
            if command_args[1] == "cd" {
                let mut new_dir: Directory = Directory {
                    files: vec![],
                    directories: vec![],
                    name: String::from(command_args[2]),
                };

                current_dir = &new_dir; // current_dir.directories.last_mut().unwrap(); // &mut new_dir;
                current_dir.directories.push(new_dir);
            }
        }
    }
}

fn find_parent_node<'a>(root: &'a Directory, node: &'a Directory) -> &'a Directory {
    root
}

#[cfg(test)]
mod tests {
    use super::find_parent_node;
    use super::Directory;

    #[test]
    fn finds_parent_node() {
        /*
         * Given the following structure:
         * / (root)
         *  - a
         *     - b
         *         - c
         *  - d
         */
        let mut root: Directory = Directory {
            directories: Vec::new(),
            files: Vec::new(),
            name: String::from("/"),
        };

        let mut dir_a: Directory = Directory {
            directories: Vec::new(),
            files: Vec::new(),
            name: String::from("a"),
        };

        let mut dir_b: Directory = Directory {
            directories: Vec::new(),
            files: Vec::new(),
            name: String::from("a"),
        };

        let mut dir_c: Directory = Directory {
            directories: Vec::new(),
            files: Vec::new(),
            name: String::from("a"),
        };

        let mut dir_d: Directory = Directory {
            directories: Vec::new(),
            files: Vec::new(),
            name: String::from("a"),
        };

        let c_ref = &dir_c;

        dir_b.directories.push(dir_c);
        dir_a.directories.push(dir_b);
        root.directories.push(dir_a);
        root.directories.push(dir_d);

        find_parent_node(&root, c_ref);
    }
}
