use std::env;
use std::fs;

fn read_input_file() -> String {
    let args: Vec<String> = env::args().collect();
    let file_path: &String = &args[1];

    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

fn map_xyz_to_abc(round: &str) -> Vec<char> {
    let hand: Vec<char> = round.replace(" ", "").chars().collect();
    let opponent = hand[0];
    let mut hero = hand[1];
    if "XYZ".contains(hero) {
        hero = char::from_u32(hero as u32 - 23).unwrap();
    }

    vec![opponent, hero]
}

/**
 * We use direction to choose the hero's play.
 * To get the direction, we subtract 88 (X) plus 1, so that
 * X = -1
 * Y = 0
 * Z = 1
 *
 * If the hero's play goes out of bounds after applying the direction, we multiply
 * the direction by "-3" and add it to the hero's play
 *
 * Example 1: "C X"
 *   The opponent played C and the hero wants to lose.
 *   The losing direction is -1, so C - 1 = B.
 * Example 2: "C Z"
 *   Hero wants to beat C, therefore the direction is "1"
 *   C + 1 becomes D, but that's out of bounds, so we need to multiply by -3.
 *   D + (1 * -3) = D - 3
 *   D - 3 = A, and A beats C (rock beats scissors)
 */
fn play_towards_outcome(round: &str) -> Vec<char> {
    let hand: Vec<char> = round.replace(" ", "").chars().collect();
    let opponent: char = hand[0];
    let desired_outcome: char = hand[1];
    let mut direction: i8 = desired_outcome as i8 - 88 - 1;
    let proposed_play: i8 = (opponent as i8) + direction;

    if !(65..68).contains(&proposed_play) {
        direction *= -2 // play falls out of range, switch position
    }

    vec![opponent, (opponent as i8 + direction) as u8 as char]
}

fn calculate_round_score(opponent_c: char, hero_c: char) -> u32 {
    let hero: u32 = hero_c as u32;
    let mut score: u32 = hero - 64;
    let opponent: u32 = opponent_c as u32;

    if hero - 1 == opponent || hero + 2 == opponent {
        score += 6;
    } else if hero == opponent {
        score += 3;
    }

    score
}

fn calculate_score(rounds: &String, strategy: fn(&str) -> Vec<char>) -> u32 {
    rounds.split("\n").fold(0, |acc, round| {
        if round.is_empty() {
            return acc;
        }

        let hand: Vec<char> = strategy(round);

        let score: u32 = calculate_round_score(hand[0], hand[1]);

        acc + score
    })
}

fn test_hero_play_decision() {
    assert_eq!(play_towards_outcome("A X"), vec!['A', 'C']);
    assert_eq!(play_towards_outcome("B X"), vec!['B', 'A']);
    assert_eq!(play_towards_outcome("C X"), vec!['C', 'B']);
    assert_eq!(play_towards_outcome("A Y"), vec!['A', 'A']);
    assert_eq!(play_towards_outcome("B Y"), vec!['B', 'B']);
    assert_eq!(play_towards_outcome("C Y"), vec!['C', 'C']);
    assert_eq!(play_towards_outcome("A Z"), vec!['A', 'B']);
    assert_eq!(play_towards_outcome("B Z"), vec!['B', 'C']);
    assert_eq!(play_towards_outcome("C Z"), vec!['C', 'A']);
}

fn main() {
    // would be more efficient to read line by line
    let contents = read_input_file();

    test_hero_play_decision();

    let part1_score: u32 = calculate_score(&contents, map_xyz_to_abc);
    let part2_score: u32 = calculate_score(&contents, play_towards_outcome);

    println!("Part 1 Score: {part1_score}");
    println!("Part 2 Score: {part2_score}");
}
