use std::env;
use std::fs;

fn read_input_file() -> String {
    let args: Vec<String> = env::args().collect();
    let file_path: &String = &args[1];

    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

fn convert_xyz_to_abc(c: char) -> char {
    char::from_u32(c as u32 - 23).unwrap()
}

fn calculate_score(rounds: String) -> u32 {
    rounds.split("\n").fold(0, |acc, r| {
        if r.is_empty() {
            return acc;
        }

        let hands: Vec<char> = r
            .replace(" ", "")
            .chars()
            .to_owned()
            .map(convert_xyz_to_abc)
            .collect();

        println!("ASCII value: {}", hands[1]);
        acc + 1
    })
}

fn main() {
    // would be more efficient to read line by line
    let contents = read_input_file();
    let score: u32 = calculate_score(contents);
    println!("Score: {score}");
}
