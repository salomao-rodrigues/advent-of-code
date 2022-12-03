use std::env;
use std::fs;

fn read_input_file() -> String {
    let args: Vec<String> = env::args().collect();
    let file_path: &String = &args[1];

    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

fn main() {
    let contents = read_input_file();

    println!("Contents: {contents}");
}
