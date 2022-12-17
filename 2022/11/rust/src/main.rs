use lazy_static::lazy_static;
use regex::Regex;
use std::env;
use std::fs;

#[derive(Debug)]
struct Monkey {
    items: Vec<u64>,
    operation: String,
    factor: u64,
    throws_to: (usize, usize),
}

fn breed_monkey(monkey: &str) -> Monkey {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^Monkey\s(?P<subject>\d):\s+Starting\sitems:(?P<items>.*)\s+Operation:\snew\s=\s(?P<operation>.*)\s+Test:\D+(?P<factor>\d+)\s*If\strue:\D+(?P<monkey_one>\d+)\s*If\sfalse:\s\D+(?P<monkey_two>\d)").unwrap();
    }

    let caps = RE.captures(monkey).unwrap();
    let items = caps["items"]
        .split(",")
        .map(|item| item.trim().parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let throw_if_true: usize = caps["monkey_one"].parse::<usize>().unwrap();
    let throw_if_false: usize = caps["monkey_two"].parse::<usize>().unwrap();

    return Monkey {
        items,
        operation: caps["operation"].to_string(),
        factor: caps["factor"].parse::<u64>().unwrap(),
        throws_to: (throw_if_true, throw_if_false),
    };
}

fn read_input_file() -> String {
    let args: Vec<String> = env::args().collect();
    let file_path: &String = &args[1];

    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

fn calculate_worry_level(old: u64, formula: &str) -> u64 {
    let resolved: String = formula.replace("old", &old.to_string());
    let parts: Vec<&str> = resolved.split(" ").collect();
    let new: u64 = match parts[1] {
        "+" => parts[0].parse::<u64>().unwrap() + parts[2].parse::<u64>().unwrap(),
        "-" => parts[0].parse::<u64>().unwrap() - parts[2].parse::<u64>().unwrap(),
        "*" => parts[0].parse::<u64>().unwrap() * parts[2].parse::<u64>().unwrap(),
        "/" => parts[0].parse::<u64>().unwrap() / parts[2].parse::<u64>().unwrap(),
        _ => unreachable!(),
    };

    new
}

fn calculate_monkey_business(
    iterations: u32,
    hide_the_pain: fn(worry: u64, factor: u64) -> u64,
) -> u64 {
    let contents: String = read_input_file();
    let mut troop: Vec<Monkey> = contents.split("\n\n").map(breed_monkey).collect();
    let mut count: Vec<u64> = [0].repeat(troop.len());

    let common_factor: u64 = troop.iter().fold(1, |factor, m| factor * m.factor);

    for _ in 0..iterations {
        for idx in 0..troop.len() {
            count[idx] += troop[idx].items.len() as u64;
            for mut worry_level in troop[idx].items.drain(..).collect::<Vec<u64>>() {
                // - calculate worry level
                //ops[idx](&mut worry_level);
                worry_level = calculate_worry_level(worry_level, &troop[idx].operation);
                // worry_level = hide_the_pain(worry_level, common_factor);
                worry_level = hide_the_pain(worry_level, common_factor);
                // - check if item is divisible by factor
                let target_monkey: usize = if worry_level.clone() % troop[idx].factor.clone() == 0 {
                    troop[idx].throws_to.0
                } else {
                    troop[idx].throws_to.1
                };

                // - throw item at monkey depending on divisible true/false
                troop[target_monkey].items.push(worry_level.clone());
            }
        }
    }

    count.sort();
    count.iter().rev().take(2).fold(1, |acc, n| acc * n)
}

fn main() {
    let part1: u64 = calculate_monkey_business(20, |w, _| w / 3);
    let part2: u64 = calculate_monkey_business(10000, |w, f| w % f);

    println!("Part 1 Solution: {}", part1);
    println!("Part 2 Solution: {}", part2);
}

#[cfg(test)]
mod tests {
    use crate::Monkey;

    use super::{breed_monkey, calculate_worry_level};

    #[test]
    fn parses_monkeys() {
        let monkey_string = "Monkey 0:
        Starting items: 79, 98
        Operation: new = old * 19
        Test: divisible by 23
            If true: throw to monkey 2
            If false: throw to monkey 3";

        let expected: Monkey = Monkey {
            items: [79, 98].to_vec(),
            operation: "old * 19".to_string(),
            factor: 23,
            throws_to: (2, 3),
        };

        let actual: Monkey = breed_monkey(monkey_string);

        assert_eq!(actual.items, expected.items);
        assert_eq!(actual.operation, expected.operation);
        assert_eq!(actual.factor, expected.factor);
        assert_eq!(actual.throws_to, expected.throws_to);
    }

    #[test]
    fn calculates_worry_level() {
        assert_eq!(calculate_worry_level(79, "old * 19"), 1501);
        assert_eq!(calculate_worry_level(98, "old * 19"), 1862);
        assert_eq!(calculate_worry_level(15, "old + 6"), 21);
        assert_eq!(calculate_worry_level(6, "old * old"), 36);
        assert_eq!(calculate_worry_level(6, "13 - old"), 7);
        assert_eq!(calculate_worry_level(39, "old / 5"), 7);
    }
}
