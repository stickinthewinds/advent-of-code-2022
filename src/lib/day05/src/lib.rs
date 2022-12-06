use std::collections::BTreeMap;

use regex::Regex;

pub fn part_one(lines: Vec<String>) -> String {
    let (stack_lines, movement_lines) = read_to_stacks_and_movements(lines);
    let stacks = build_stacks(stack_lines);
    let stacks = perform_movements(movement_lines, stacks, false);

    get_results(stacks)
}

pub fn part_two(lines: Vec<String>) -> String {
    let (stack_lines, movement_lines) = read_to_stacks_and_movements(lines);
    let stacks = build_stacks(stack_lines);
    let stacks = perform_movements(movement_lines, stacks, true);

    get_results(stacks)
}

fn read_to_stacks_and_movements(lines: Vec<String>) -> (Vec<String>, Vec<String>) {
    let re = Regex::new(r"^$").unwrap();
    let (stack_lines, mut movement_lines) =
        lines.split_at(lines.iter().position(|l| re.is_match(l)).unwrap());
    let mut stack_lines = stack_lines.to_vec();
    stack_lines.reverse();
    movement_lines = &movement_lines[1..];
    (stack_lines, movement_lines.to_vec())
}

fn build_stacks(mut stack_lines: Vec<String>) -> BTreeMap<i32, Vec<String>> {
    let num_stacks =
        char::to_digit(stack_lines.remove(0).chars().last().unwrap(), 10).unwrap() as i32;
    let mut stacks = BTreeMap::new();
    for i in 1..num_stacks + 1 {
        stacks.insert(i, Vec::new());
    }
    stack_lines.iter().for_each(|l| {
        let mut mutable_l = l.clone();
        let re = Regex::new(r"(?P<letter>[A-Z])").unwrap();
        for caps in re.captures_iter(l) {
            let letter = caps["letter"].to_string();
            let position = mutable_l.find(letter.as_str()).unwrap();
            mutable_l.replace_range(position..position+1, "_");
            let stack_num = position as i32 / 4 + 1;
            let mut stack_vec = stacks.remove(&stack_num).unwrap().clone();
            stack_vec.push(letter.clone());
            stacks.insert(stack_num, stack_vec);
        }
    });
    stacks
}

fn parse_movement(movement: String) -> Result<Vec<i32>, String> {
    let s = movement.split_whitespace();
    let mut parts = Vec::new();
    for part in s {
        let parsed = part.parse::<i32>();
        if parsed.is_ok() {
            parts.push(parsed.ok().unwrap());
        }
    }
    if parts.len() != 3 {
        return Err(format!("Not enough arguments in the line: '{movement}'"));
    }
    Ok(parts)
}

fn perform_movements(movement_lines: Vec<String>, mut stacks: BTreeMap<i32, Vec<String>>, allow_multiple: bool) -> BTreeMap<i32, Vec<String>> {
    for movement in movement_lines.iter() {
        let parts = parse_movement(movement.to_string())
            .expect("Expected numbers for moves, from and to in the line.");
        let from = stacks.remove(&parts[1]).unwrap().to_vec();
        let to = stacks.remove(&parts[2]).unwrap().to_vec();
        let amount = parts[0];
        let (from, to) = perform_movement(from.clone(), to.clone(), amount, allow_multiple);
        stacks.insert(parts[1], from);
        stacks.insert(parts[2], to);
    }
    stacks
}

fn perform_movement(
    mut from: Vec<String>,
    mut to: Vec<String>,
    amount: i32,
    allow_multiple: bool
) -> (Vec<String>, Vec<String>) {
    if allow_multiple {
        let insert_point = to.len();
        for _i in 0..amount.min(from.len() as i32) {
            to.insert(insert_point, from.pop().unwrap());
        }
    } else {
        for _i in 0..amount.min(from.len() as i32) {
            to.push(from.pop().unwrap());
        }
    }
    (from, to)
}

fn get_results(stacks: BTreeMap<i32, Vec<String>>) -> String {
    let result = stacks
        .iter()
        .filter(|(_k, v)| !v.is_empty())
        .map(|(_k, v)| v.clone().pop().unwrap())
        .collect::<Vec<String>>()
        .join("");
    result
}

#[cfg(test)]
mod tests {
    use crate::{parse_movement, perform_movement};

    #[test]
    fn test_parse() {
        let movement = "move 1 from 2 to 1".to_string();
        let bad_movement = "move 1 from 2".to_string();
        let result = parse_movement(movement);
        let bad_result = parse_movement(bad_movement);
        assert!(result.is_ok());
        assert!(bad_result.is_err());

        let result = result.ok().unwrap();
        assert_eq!(3, result.len());
        assert_eq!(vec![1, 2, 1], result);
    }

    #[test]
    fn test_move() {
        let from = vec!["D".to_string(), "C".to_string(), "M".to_string()];
        let to = vec!["P".to_string()];
        let amount = 2;
        let (from, to) = perform_movement(from, to, amount);
        assert_eq!(vec!["D".to_string()], from);
        assert_eq!(vec!["P", "M", "C"], to);
    }
}
