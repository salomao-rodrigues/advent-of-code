use std::cmp::Ordering;

fn get_max_depth(a: &str) -> u16 {
    let mut depth: u16 = 0;
    let mut cur_depth: u16 = 0;

    for c in a.chars() {
        if c == '[' {
            cur_depth += 1;
            depth = depth.max(cur_depth);
        } else if c == ']' {
            cur_depth -= 1;
        }
    }

    return depth;
}

fn parse_packet_to_numbers(p: &str) -> Vec<i32> {
    let mut numbers: String = p.to_string();
    for c in "[] ".chars() {
        numbers = numbers.replace(c, "");
    }

    if numbers.len() == 0 {
        return vec![];
    }

    numbers
        .split(",")
        .map(|n| n.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

fn packet_sorter(a: &str, b: &str) -> Ordering {
    let a_numbers: Vec<i32> = parse_packet_to_numbers(a);
    let b_numbers: Vec<i32> = parse_packet_to_numbers(b);

    if a_numbers.len() + b_numbers.len() == 0 {
        return get_max_depth(a).cmp(&get_max_depth(b));
    }

    let min_length = a_numbers.len().min(b_numbers.len());

    for i in 0..min_length {
        if a_numbers[i] != b_numbers[i] {
            return a_numbers[i].cmp(&b_numbers[i]);
        }
    }

    a_numbers.len().cmp(&b_numbers.len())
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use crate::{get_max_depth, packet_sorter, parse_packet_to_numbers};

    #[test]
    fn parses_packet_to_numbers() {
        assert_eq!(parse_packet_to_numbers("[1,1,3,1,1]"), vec![1, 1, 3, 1, 1]);
        assert_eq!(parse_packet_to_numbers("[[1],[2,3,4]]"), vec![1, 2, 3, 4]);
        assert_eq!(
            parse_packet_to_numbers("[1,[2,[3,[4,[5,6,7]]]],8,9]"),
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
        );
    }

    #[test]
    fn gets_max_depth() {
        assert_eq!(get_max_depth("[1,1,3,1,1]"), 1);
        assert_eq!(get_max_depth("[[1],[2,3,4]]"), 2);
        assert_eq!(get_max_depth("[[4,4],4,4]"), 2);
        assert_eq!(get_max_depth("[[[]]]"), 3);
        assert_eq!(get_max_depth("[[[]]]"), 3);
        assert_eq!(get_max_depth("[1,[2,[3,[4,[5,6,7]]]],8,9]"), 5);
    }

    #[test]
    fn sorts_pairs() {
        let mut a: &str;
        let mut b: &str;
        a = "[1,1,3,1,1]";
        b = "[1,1,5,1,1]";

        assert_eq!(packet_sorter(a, b), Ordering::Less);

        a = "[[1],[2,3,4]]";
        b = "[[1],4]";

        assert_eq!(packet_sorter(a, b), Ordering::Less);

        a = "[9]";
        b = "[[8,7,6]]";

        assert_eq!(packet_sorter(a, b), Ordering::Greater);

        a = "[[4,4],4,4]";
        b = "[[4,4],4,4,4]";

        assert_eq!(packet_sorter(a, b), Ordering::Less);

        a = "[7,7,7,7]";
        b = "[7,7,7]";

        assert_eq!(packet_sorter(a, b), Ordering::Greater);

        a = "[]";
        b = "[3]";

        assert_eq!(packet_sorter(a, b), Ordering::Less);

        a = "[[[]]]";
        b = "[[]]";

        assert_eq!(packet_sorter(a, b), Ordering::Greater);

        a = "[1,[2,[3,[4,[5,6,7]]]],8,9]";
        b = "[1,[2,[3,[4,[5,6,0]]]],8,9]";

        assert_eq!(packet_sorter(a, b), Ordering::Greater);
    }
}
