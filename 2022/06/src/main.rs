use std::collections::HashSet;
use std::str::Chars;
use std::{env, fs};

const PART_1_LENGTH: usize = 4;
const PART_2_LENGTH: usize = 14;

fn read_input_file() -> String {
    let args: Vec<String> = env::args().collect();
    let file_path: &String = &args[1];

    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

fn find_marker_position(buffer: &str, slice_length: usize) -> usize {
    (slice_length..buffer.len())
        .find(|&i| {
            let char_set: HashSet<char> =
                HashSet::from_iter::<Chars>(buffer[i - slice_length..i].chars());
            char_set.len() == slice_length
        })
        .unwrap()
}

fn main() {
    let buffer: String = read_input_file();
    println!("Part 1: {}", find_marker_position(&buffer, PART_1_LENGTH));
    println!("Part 2: {}", find_marker_position(&buffer, PART_2_LENGTH));
}

#[cfg(test)]
mod tests {
    use crate::{PART_1_LENGTH, PART_2_LENGTH};

    use super::find_marker_position;

    #[test]
    fn finds_the_marker_position_part_1() {
        assert_eq!(
            find_marker_position("bvwbjplbgvbhsrlpgdmjqwftvncz", PART_1_LENGTH),
            5
        );
        assert_eq!(
            find_marker_position("nppdvjthqldpwncqszvftbrmjlhg", PART_1_LENGTH),
            6
        );
        assert_eq!(
            find_marker_position("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", PART_1_LENGTH),
            10
        );
        assert_eq!(
            find_marker_position("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", PART_1_LENGTH),
            11
        );
    }
    #[test]
    fn finds_the_marker_position_part_2() {
        assert_eq!(
            find_marker_position("mjqjpqmgbljsphdztnvjfqwrcgsmlb", PART_2_LENGTH),
            19
        );
        assert_eq!(
            find_marker_position("bvwbjplbgvbhsrlpgdmjqwftvncz", PART_2_LENGTH),
            23
        );
        assert_eq!(
            find_marker_position("nppdvjthqldpwncqszvftbrmjlhg", PART_2_LENGTH),
            23
        );
        assert_eq!(
            find_marker_position("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", PART_2_LENGTH),
            29
        );
        assert_eq!(
            find_marker_position("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", PART_2_LENGTH),
            26
        );
    }
}
