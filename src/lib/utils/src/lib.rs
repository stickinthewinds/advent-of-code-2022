use std::fmt::Display;
use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn solve<T, F, R: Display>(f: F, input: T, msg: Option<&str>) where F: Fn(T) -> R {
    let m = msg.unwrap_or_default();
    println!("{}{}", m, f(input));
}

pub fn read_lines(file_path: String) -> Vec<String> {
    let file = File::open(file_path).expect("Should have been able to read the file");
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    lines
}

// TODO: Add tests
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
