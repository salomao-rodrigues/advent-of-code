use std::env;
use std::fs;

fn read_input_file() -> String {
    let args: Vec<String> = env::args().collect();
    let file_path: &String = &args[1];

    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

fn sum_total_calories(calories: &str) -> u32 {
    calories
        .split("\n")
        .fold(0, |acc, c| acc + c.parse::<u32>().unwrap_or(0))
}

fn calculate_max_calories_carried_by_elf(input: &str) -> u32 {
    input.split("\n\n").map(sum_total_calories).max().unwrap()
}

fn calculate_total_calories_by_top_three_elves(input: &str) -> u32 {
    let mut total_calories_per_elf: Vec<u32> =
        input.split("\n\n").map(sum_total_calories).collect();
    total_calories_per_elf.sort();
    let top_three = total_calories_per_elf.iter().rev().take(3);

    top_three.sum()
}

fn main() {
    let contents = read_input_file();
    let max_carried_calories = calculate_max_calories_carried_by_elf(&contents);
    let top_three_total_calories = calculate_total_calories_by_top_three_elves(&contents);

    println!("-------------\nMax calories carried by Elf: {max_carried_calories}");
    println!("Total calories carried by top 3 Elves: {top_three_total_calories}");
}
