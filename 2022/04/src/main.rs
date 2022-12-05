use std::env;
use std::fs;

fn read_input_file() -> String {
    let args: Vec<String> = env::args().collect();
    let file_path: &String = &args[1];

    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

fn run_tests() {
    assert_eq!(parse_ranges("0-4,6-8"), ([0, 4], [6, 8]));
    assert_eq!(parse_ranges("2-8,3-7"), ([2, 8], [3, 7]));
    assert_eq!(parse_ranges("85-87,42-86"), ([85, 87], [42, 86]));
    assert_eq!(one_range_contains_the_other(&([85, 87], [42, 86])), false);
    assert_eq!(one_range_contains_the_other(&([6, 6], [4, 6])), true);
    assert_eq!(one_range_contains_the_other(&([2, 8], [3, 7])), true);
    assert_eq!(one_range_contains_the_other(&([5, 7], [7, 9])), false);
    assert_eq!(ranges_overlap(&([0, 4], [6, 8])), false);
    assert_eq!(ranges_overlap(&([85, 87], [42, 86])), true);
    assert_eq!(ranges_overlap(&([6, 6], [4, 6])), true);
    assert_eq!(ranges_overlap(&([2, 7], [8, 8])), false);
    assert_eq!(ranges_overlap(&([2, 3], [3, 4])), true);
}

fn parse_ranges(line: &str) -> ([u32; 2], [u32; 2]) {
    let ranges: Vec<[u32; 2]> = line
        .split(",")
        .map(|range| {
            range
                .split("-")
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .map(|v| [v[0], v[1]])
        .collect();

    (ranges[0], ranges[1])
}

fn one_range_contains_the_other(([start1, end1], [start2, end2]): &([u32; 2], [u32; 2])) -> bool {
    let first_range = start1..=end1;
    let second_range = start2..=end2;
    (first_range.contains(&start2) && first_range.contains(&end2))
        || (second_range.contains(&start1) && second_range.contains(&end1))
}

fn ranges_overlap(([start1, end1], [start2, end2]): &([u32; 2], [u32; 2])) -> bool {
    let first_range = start1..=end1;
    let second_range = start2..=end2;
    first_range.contains(&start2)
        || first_range.contains(&end2)
        || second_range.contains(&start1)
        || second_range.contains(&end1)
}

fn main() {
    run_tests();
    let file = read_input_file();
    let parsed_ranges_iterator = file.lines().map(parse_ranges);

    let count_contained = parsed_ranges_iterator
        .clone()
        .filter(one_range_contains_the_other)
        .count();

    let count_overlapped = parsed_ranges_iterator
        .clone()
        .filter(ranges_overlap)
        .count();

    println!("Part 1 - Number of ranges containing the other: {count_contained}");
    println!("Part 1 - Number of ranges that overlap: {count_overlapped}");
}
