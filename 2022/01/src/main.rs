use std::env;
use std::fs;

fn read_input_file() -> String {
    let args: Vec<String> = env::args().collect();
    let file_path: &String = &args[1];
    println!("In file {}", file_path);

    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

fn sum_total_calories(calories: &str) -> u32 {
    calories
        .split("\n")
        .fold(0, |acc, c| acc + c.parse::<u32>().unwrap_or(0))
}

fn calculate_max_calories_carried_by_elf(input: String) -> u32 {
    input.split("\n\n").map(sum_total_calories).max().unwrap()
}

fn main() {
    let contents = read_input_file();
    let max_carried_calories = calculate_max_calories_carried_by_elf(contents);

    println!("Max calories carried by Elf: {max_carried_calories}");
}
