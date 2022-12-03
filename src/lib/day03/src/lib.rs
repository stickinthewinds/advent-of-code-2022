// Total score when X/Y/Z is your choice
pub fn part_one(lines: Vec<String>) -> i32 {
    convert_lines_to_pairs(lines)
    .iter()
    .map(|(lhs, rhs)| {
        lhs.chars().filter(|c| rhs.contains(*c)).collect::<Vec<char>>()[0]
    })
    .map(|c| get_priority(c))
    .sum()
}

// Total score when X/Y/Z is your choice
pub fn part_two(lines: Vec<String>) -> i32 {
    lines
        .chunks(3)
        .map(|set| get_priority(get_common_char_from_list(set)))
        .sum()
}

fn get_priority(c: char) -> i32 {
    if c.is_ascii_lowercase() {
        return c as i32 - 96;
    }
    c as i32 - 64 + 26
}

fn get_common_char_from_list(lists: &[String]) -> char {
    let mut lhs = lists[0].to_string();
    let rhs = lists[1].to_string();
    let mut result = get_common_chars(lhs, rhs);
    for l in &lists[2..] {
        lhs = result.into_iter().collect();
        result = get_common_chars(lhs, l.to_string());
    }
    result[0]
}

fn get_common_chars(lhs: String, rhs: String) -> Vec<char> {
    lhs.chars().filter(|c| rhs.contains(*c)).collect::<Vec<char>>()
}

fn convert_lines_to_pairs(lines: Vec<String>) -> Vec<(String, String)> {
    lines
        .iter()
        .map(|l| {
            let (a, b) = l.split_at(l.len() / 2);
            (a.to_string(), b.to_string())
        })
        .collect()
}
