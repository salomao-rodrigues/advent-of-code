use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn calc_direction_vector(point1: &[i32; 2], point2: &[i32; 2]) -> [i32; 2] {
    let mut new_points = point1
        .iter()
        .zip::<&[i32; 2]>(point2)
        .map(|(p1, p2)| p2 - p1);
    let x = new_points.next().unwrap();
    let y = new_points.next().unwrap();

    return if x.abs() <= 1 && y.abs() <= 1 {
        [0, 0]
    } else if x.abs() == 0 {
        [0, y / y.abs()]
    } else if y.abs() == 0 {
        [x / x.abs(), 0]
    } else {
        [x / x.abs(), y / y.abs()]
    };
}

fn are_coordinates_adjacent(point1: &[i32; 2], point2: &[i32; 2]) -> bool {
    point1
        .iter()
        .zip(point2)
        .all(|(p1, p2)| (p1 - p2).abs() <= 1)
}

fn move_head(head: &mut [i32; 2], movement: &str) {
    let m_input: Vec<&str> = movement.split(" ").collect();
    let direction: &str = m_input[0];
    let lenght: i32 = m_input[1].parse().unwrap();
    match direction {
        "R" => head[0] += lenght,
        "L" => head[0] -= lenght,
        "U" => head[1] += lenght,
        "D" => head[1] -= lenght,
        _ => panic!(
            "Direction \"{}\" not supported when trying to move head",
            direction
        ),
    }
}

fn join_last_knot_xy(rope: &Vec<[i32; 2]>) -> String {
    rope.last().unwrap().map(|n: i32| n.to_string()).join(",")
}

fn solve_problem(reader: BufReader<File>, rope_length: usize) -> usize {
    let mut visited: HashSet<String> = HashSet::new();
    // create ROPE
    let mut rope: Vec<[i32; 2]> = [[0, 0]].repeat(rope_length);

    // add TAIL starting point into visited SET (so it stores unique points)
    visited.insert(join_last_knot_xy(&rope));
    // for each line in motions input
    for movement in reader.lines() {
        move_head(&mut rope[0], &movement.unwrap());

        // while 2nd knot not adjacent to head
        while !are_coordinates_adjacent(&rope[0], &rope[1]) {
            // loop through all knots and keep moving them close to each other
            for idx in 0..rope.len() - 1 {
                let tail_idx: usize = idx + 1;
                // calculate movement vector
                let movement_vec: [i32; 2] = calc_direction_vector(&rope[tail_idx], &rope[idx]);
                // add vector to tail position
                rope[tail_idx] = [
                    rope[tail_idx][0] + movement_vec[0],
                    rope[tail_idx][1] + movement_vec[1],
                ];
            }

            // add new TAIL position to visited SET
            let position = join_last_knot_xy(&rope);
            visited.insert(position);
        }
    }

    // println!("{:?}", visited);
    visited.len()
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let file_path: &String = &args[1];
    let file = File::open(file_path)?;
    let file2 = File::open(file_path)?;
    let reader = BufReader::new(file);
    let reader2 = BufReader::new(file2);

    let part_1 = solve_problem(reader, 2);
    let part_2 = solve_problem(reader2, 10);

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{are_coordinates_adjacent, calc_direction_vector, move_head};

    #[test]
    fn moves_head() {
        let mut head: [i32; 2] = [0, 0];
        move_head(&mut head, "R 4");
        assert_eq!(head, [4, 0]);
        move_head(&mut head, "L 9");
        assert_eq!(head, [-5, 0]);
        move_head(&mut head, "D 3");
        assert_eq!(head, [-5, -3]);
        move_head(&mut head, "U 5");
        assert_eq!(head, [-5, 2]);
    }

    #[test]
    fn finds_if_coordinates_are_adjacent() {
        assert!(are_coordinates_adjacent(&[0, 0], &[0, 0]));
        assert!(are_coordinates_adjacent(&[1, 1], &[0, 0]));
        assert!(are_coordinates_adjacent(&[-2, 5], &[-2, 5]));
        assert!(are_coordinates_adjacent(&[1, 1], &[-1, 0]) == false);
        assert!(are_coordinates_adjacent(&[-2, 3], &[-2, 5]) == false);
        assert!(are_coordinates_adjacent(&[4, 3], &[3, 1]) == false);
    }

    #[test]
    fn calculates_direction_vector() {
        assert_eq!(calc_direction_vector(&[0, 0], &[0, 0]), [0, 0]);
        assert_eq!(calc_direction_vector(&[0, 0], &[1, 1]), [0, 0]);
        assert_eq!(calc_direction_vector(&[0, 0], &[0, 2]), [0, 1]);
        assert_eq!(calc_direction_vector(&[0, 0], &[3, 0]), [1, 0]);
        assert_eq!(calc_direction_vector(&[0, 0], &[1, 2]), [1, 1]);
        assert_eq!(calc_direction_vector(&[0, 0], &[3, 3]), [1, 1]);
        assert_eq!(calc_direction_vector(&[-4, -4], &[-4, -4]), [0, 0]);
        assert_eq!(calc_direction_vector(&[-5, -4], &[-4, -3]), [0, 0]);
        assert_eq!(calc_direction_vector(&[-3, -5], &[-3, -7]), [0, -1]);
        assert_eq!(calc_direction_vector(&[-5, 1], &[-7, 1]), [-1, 0]);
        assert_eq!(calc_direction_vector(&[1, 3], &[0, 1]), [-1, -1]);
    }
}
