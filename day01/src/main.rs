use std::cmp;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut current: i32 = 0;
    let mut max: i32 = 0;

    let filename = "./input.txt";
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.

        if !line.is_empty() {
            current += line.parse::<i32>().unwrap();
        } else {
            max = cmp::max(current, max);
            current = 0;
        }
    }
    println!("max {}", max);
}
