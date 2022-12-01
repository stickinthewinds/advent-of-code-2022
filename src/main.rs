use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args[1].to_string();
    println!("In file {}", file_path);

    let contents = read_lines(file_path);

    // DAY 1
    let sums = get_sums_in_descending_order(contents);

    // Part 1: Maximum amount
    let max = sums[0];
    println!("Highest amount is: {max}");
    // Part 2: Combined amount of the top 3 sums
    let combined = sums[0] + sums[1] + sums[2];
    println!("Combined total for the top three is: {combined}");
}

fn read_lines(file_path: String) -> Vec<String> {
    // let mut lines = <Vec<String>>::new();
    let file = File::open(file_path).expect("Should have been able to read the file");
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    return lines;
}

fn get_sums_in_descending_order(lines: Vec<String>) -> Vec<i32> {
    let mut amounts: Vec<i32> = Vec::new();
    let mut current = 0;

    for line in lines {
        if line.is_empty() {
            amounts.push(current);
            current = 0;
            continue;
        }
        current += line.parse::<i32>().expect("Should only be numbers");
    }

    amounts.sort_by(|a, b| b.cmp(a));
    return amounts;
}
