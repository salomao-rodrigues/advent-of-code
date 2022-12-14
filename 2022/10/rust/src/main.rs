use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

const CRT_WIDTH: i32 = 40;
const CRT_HEIGHT: i32 = 6;

fn is_strength_cycle(cycle: i32) -> bool {
    cycle == 20 || (cycle - 20) % 40 == 0
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let file_path: &String = &args[1];
    let file: File = File::open(file_path)?;
    let reader: BufReader<File> = BufReader::new(file);
    let mut lines = reader.lines().peekable();
    let mut cycle: i32 = 0;
    let mut x: i32 = 1;
    let mut signal_strenght: i32 = 0;

    let mut increment_cycle = |x: i32| {
        let crt_pos = cycle % CRT_WIDTH;
        cycle += 1;
        let render_sprite = (x - 1..=x + 1).contains(&crt_pos);
        if is_strength_cycle(cycle) {
            signal_strenght += cycle as i32 * x;
        }

        print!("{}", if render_sprite { "#" } else { "." });
        if cycle % CRT_WIDTH == 0 {
            println!();
        }
    };

    while lines.peek().is_some() {
        let line = lines.next().unwrap().unwrap();
        if line == "noop".to_string() {
            increment_cycle(x);
            continue;
        }

        let (_, value) = line.split_once(" ").unwrap();
        increment_cycle(x);
        increment_cycle(x);
        x += value.parse::<i32>().unwrap();
    }

    println!("{}", signal_strenght);
    Ok(())
}
