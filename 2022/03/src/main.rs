use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

const UPPERCASE_OFFSET: u32 = 38;
const LOWERCASE_OFFSET: u32 = 96;

fn convert_to_priority(c: char) -> u32 {
    return if c.is_uppercase() {
        c as u32 - UPPERCASE_OFFSET
    } else {
        c as u32 - LOWERCASE_OFFSET
    };
}

fn get_common_item((first_compartment, second_compartment): (&str, &str)) -> char {
    first_compartment
        .chars()
        .find(|item| second_compartment.contains(*item))
        .unwrap()
}

fn find_common_item_in_rucksacks(rucksacks: &[String]) -> char {
    let index = rucksacks[0]
        .find(|item| {
            rucksacks
                .iter()
                .skip(1)
                .all(|rucksack| rucksack.contains(item))
        })
        .unwrap();

    rucksacks[0].chars().nth(index).unwrap()
}

fn sum_all_priorities(reader: BufReader<File>) -> u32 {
    reader
        .lines()
        .map(|l| l.unwrap_or(String::from("")))
        .collect::<Vec<String>>()
        .iter()
        .map(|l| l.split_at(l.len() / 2))
        .map(get_common_item)
        .map(convert_to_priority)
        .sum()
}

fn sum_group_priorities(reader: BufReader<File>) -> u32 {
    reader
        .lines()
        .map(|l| l.unwrap_or(String::from("")))
        .collect::<Vec<String>>()
        .chunks(3)
        .collect::<Vec<&[String]>>()
        .iter()
        .map(|&triplet| find_common_item_in_rucksacks(triplet))
        .map(convert_to_priority)
        .sum()
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let file_path_1: &String = &args[1];
    let file_path_2: &String = &args[2];
    let file_1 = File::open(file_path_1)?;
    let file_2 = File::open(file_path_2)?;
    let reader_1 = BufReader::new(file_1);
    let reader_2 = BufReader::new(file_2);

    // Test convert_to_priority
    assert_eq!(convert_to_priority('a'), 1);
    assert_eq!(convert_to_priority('z'), 26);
    assert_eq!(convert_to_priority('A'), 27);
    assert_eq!(convert_to_priority('Z'), 52);

    // Test get_common_item
    assert_eq!(get_common_item(("vJrwpWtwJgWr", "hcsFMMfFFhFp")), 'p');
    assert_eq!(
        get_common_item(("jqHRNqRjqzjGDLGL", "rsFMfFZSrLrFZsSL")),
        'L'
    );

    // Test find_common_item_in_rucksacks
    assert_eq!(
        find_common_item_in_rucksacks(&[
            String::from("HjHHRtwjnjRblQRttHwQGvGWNNBWvqGzfTvfNN"),
            String::from("FmScCcrsdVZrpBrVcCVFzffvzzmWGLWgqWqgGWzW"),
            String::from("SFVSDDBdsdDSJhnjJltJbPtHRM")
        ]),
        'B'
    );

    let sum_part_1 = sum_all_priorities(reader_1);
    let sum_part_2 = sum_group_priorities(reader_2);

    println!("Part 1 - Total sum: {sum_part_1}");
    println!("Part 2 - Grouped sum: {sum_part_2}");
    Ok(())
}
