/**
 * We're making the assumption that crates have a single character and that
 * The max of stacks is 9. If the input were to be extended, this solution
 * would not apply.
 *
 * Steps to solve part 1:
 * ---- Crates Creation ----
 * [x] Split input in crates and procedures
 * [x] Reverse the crates input so it's easier to parse
 * [x] Read the first line and get the Number of crates needed
 * [x] Create a Vector with N Stacks (Vectors)
 * [x] Loop through the crates lines
 * [x] Parse each line to get a vector of length N with the Crates inside
 * [x] Add the parsed Crates to their corresponding Stacks
 *
 * ---- Procedure ----
 * [x] Loop through procedure lines
 * [x] Read "move [QTY] from [SRC] to [DST]" into variables
 * [x] Pop QTY from SRC and Push to DST
 */
use lazy_static::lazy_static;
use regex::Regex;
use std::env;
use std::fs;

const FIRST_CRATE_OFFSET: usize = 1;
const SPACE_BETWEEN_CRATES: usize = 4;

fn read_input_file() -> String {
    let args: Vec<String> = env::args().collect();
    let file_path: &String = &args[1];

    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

fn render_crates(stacks: &Vec<Vec<char>>) {
    let no_crate: char = ' ';
    let stack_height: usize = stacks.iter().map(|stack| stack.len()).max().unwrap();
    for h in (0..stack_height).rev() {
        let mut crates = "".to_owned();
        for s in 0..stacks.len() {
            let found_crate = stacks[s].get(h).unwrap_or(&no_crate);
            crates.push_str(format!(" {} ", found_crate).as_str());
        }
        println!("{crates}");
    }
}

fn parse_crate_lines_to_stacks(lines: &Vec<&str>, total_crates: usize) -> Vec<Vec<char>> {
    // It'd be cool to make this more functional
    let mut stacks: Vec<Vec<char>> = vec![vec![]; total_crates];

    for line_of_crates in lines {
        let bytes_of_crates = line_of_crates.as_bytes();
        for idx in 0..total_crates {
            let current_crate: char =
                bytes_of_crates[FIRST_CRATE_OFFSET + idx * SPACE_BETWEEN_CRATES] as char;
            if current_crate != ' ' {
                stacks[idx].insert(0, current_crate);
            }
        }
    }

    stacks
}

fn parse_procedure(procedure: &str) -> (usize, usize, usize) {
    lazy_static! {
        static ref PROCEDURE_REGEX: Regex =
            Regex::new(r"move (?P<qty>\d+) from (?P<from>\d+) to (?P<to>\d+)").unwrap();
    }
    let caps = PROCEDURE_REGEX.captures(procedure).unwrap();
    (
        caps["qty"].parse::<usize>().unwrap(),
        caps["from"].parse::<usize>().unwrap() - 1,
        caps["to"].parse::<usize>().unwrap() - 1,
    )
}

fn solve(is_crate_mover_9001: bool) -> String {
    let file: String = read_input_file();
    let parts: Vec<&str> = file.split("\n\n").collect();
    let mut crate_lines: Vec<&str> = parts[0].lines().collect();
    let procedures: &str = parts[1];

    let last_crate_nr: &str = crate_lines
        .pop()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap();
    let total_crates: usize = last_crate_nr.parse::<usize>().unwrap();
    let mut stacks: Vec<Vec<char>> = parse_crate_lines_to_stacks(&crate_lines, total_crates);

    for procedure in procedures.lines() {
        let (qty, from, to) = parse_procedure(procedure);
        let new_len: usize = stacks[from].len() - qty;
        let mut crates = stacks[from].drain(new_len..).collect::<Vec<char>>();
        if !is_crate_mover_9001 {
            crates.reverse();
        }
        stacks[to].append(&mut crates);
    }

    stacks.iter().map(|stack| stack.last().unwrap()).collect()
}

fn main() {
    let solution_1 = solve(false);
    let solution_2 = solve(true);

    println!("Part 1: {}", solution_1);
    println!("Part 2: {}", solution_2);
}

#[cfg(test)]
mod tests {
    use super::parse_procedure;

    #[test]
    fn parses_procedures_correctly() {
        assert_eq!(parse_procedure("move 1 from 2 to 3"), (1, 1, 2));
        assert_eq!(parse_procedure("move 12 from 8 to 7"), (12, 7, 6));
        assert_eq!(parse_procedure("move 48 from 8 to 5"), (48, 7, 4));
    }
}
