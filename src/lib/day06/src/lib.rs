use itertools::Itertools;

pub fn part_one(line: String) -> i32 {
    let chars: Vec<char> = line.chars().collect();
    let result = find_marker(chars, 4);
    result.unwrap_or_else(|e| panic!("{}: There should be at least one substring of length 4 with no matching characters but there were none in the string '{}'", e, line))
}

pub fn part_two(line: String) -> i32 {
    let chars: Vec<char> = line.chars().collect();
    let result = find_marker(chars, 14);
    result.unwrap_or_else(|e| panic!("{}: There should be at least one substring of length 4 with no matching characters but there were none in the string '{}'", e, line))
}

// Marker is the position of the first string of num_distinct length of unique characters
fn find_marker(chars: Vec<char>, num_distinct: usize) -> Result<i32, String> {
    let mut current = chars[..num_distinct].to_vec();
    if current.iter().all_unique() {
        return Ok(4);
    }

    for (pos, &item) in chars.iter().enumerate().take(chars.len() - 1).skip(4) {
        current.remove(0);
        current.push(item);
        if current.iter().all_unique() {
            return Ok(1 + pos as i32)
        }
    }
    Err("No marker found".to_string())
}
