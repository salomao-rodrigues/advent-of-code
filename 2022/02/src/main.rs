use std::env;
use std::fs;

fn read_input_file() -> String {
    let args: Vec<String> = env::args().collect();
    let file_path: &String = &args[1];

    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

fn convert_xyz_to_abc(c: char) -> Option<char> {
    let mut converted_char = c;
    if c == ' ' {
        return None;
    } else if "XYZ".contains(c) {
        converted_char = char::from_u32(c as u32 - 23).unwrap()
    }

    Some(converted_char)
}

fn calculate_round_score(round: &str) -> u32 {
    let hand: Vec<char> = round.chars().filter_map(convert_xyz_to_abc).collect();
    let hero: char = hand[1];
    let mut score: u32 = hero as u32 - 64;
    let opponent: char = hand[0];
    if hero == opponent {
        score += 3;
    } else if hero == 'A' && opponent == 'C'
        || hero == 'B' && opponent == 'A'
        || hero == 'C' && opponent == 'B'
    {
        score += 6;
    }

    score
}

fn calculate_score(rounds: String) -> u32 {
    rounds.split("\n").fold(0, |acc, round| {
        if round.is_empty() {
            return acc;
        }

        let score: u32 = calculate_round_score(round);

        acc + score
    })
}

fn main() {
    // would be more efficient to read line by line
    let contents = read_input_file();
    let score: u32 = calculate_score(contents);
    println!("Score: {score}");
}
