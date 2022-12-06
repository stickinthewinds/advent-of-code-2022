use std::cmp::Ordering::*;

pub fn part_one(lines: Vec<String>) -> i32 {
    convert_lines_to_pairs(lines)
        .iter()
        .filter(|(lhs, rhs)| is_overlapping(lhs, rhs, false))
        .count() as i32
}

pub fn part_two(lines: Vec<String>) -> i32 {
    convert_lines_to_pairs(lines)
        .iter()
        .filter(|(lhs, rhs)| is_overlapping(lhs, rhs, true))
        .count() as i32
}

fn is_overlapping(first: &str, second: &str, allow_partial: bool) -> bool{
    let first_minmax = get_range_minmax(first);
    let second_minmax = get_range_minmax(second);
    let fully = either_fully_contains(first_minmax, second_minmax);
    fully || (allow_partial && either_partially_contains(first_minmax, second_minmax))
}

fn either_fully_contains(first: (i32, i32), second: (i32, i32)) -> bool {
    let cmp_min = first.0.cmp(&second.0);
    let cmp_max = first.1.cmp(&second.1);
    cmp_min == Equal || cmp_max == Equal || cmp_min != cmp_max
}

fn either_partially_contains(first: (i32, i32), second: (i32, i32)) -> bool {
    let cmp_f = first.0.cmp(&second.1);
    let cmp_s = second.0.cmp(&first.1);
    !((cmp_f == Less && cmp_s == Greater) || (cmp_s == Less && cmp_f == Greater))
}

fn get_range_minmax(range: &str) -> (i32, i32) {
    let values: Vec<i32> = range.split('-').map(|n| n.parse::<i32>().unwrap_or_else(|_e| panic!("'{n}' is not a number!"))).collect();
    (values[0], values[1])
}

fn convert_lines_to_pairs(lines: Vec<String>) -> Vec<(String, String)> {
    lines
        .iter()
        .map(|l| {
            let (a, b) = l.split_at(l.find(',').unwrap_or_else(|| panic!("No pair found for the line: {l}")));
            (a.to_string(), b[1..].to_string()) // b Starts at the second character to remove the ','
        })
        .collect()
}
