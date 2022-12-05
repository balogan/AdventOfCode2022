use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut current: i32 = 0;

    let mut v: Vec<i32> = Vec::new();

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
            v.push(current);
            current = 0;
        }
        println!("read line {}", line);
    }

    // NOTE: Need to add blank line after end of input
    v.sort();
    v.reverse();
    let first: &i32 = &v[0];
    let second: &i32 = &v[1];
    let third: &i32 = &v[2];
    println!("Max calories {}", first+second+third);
    println!("{:?}", v);
}
