// Maximum amount
pub fn part_one(lines: Vec<String>) -> i32 {
    let sums = get_sums_in_descending_order(lines);
    sums[0]
}

// Combined amount of the top 3 sums
pub fn part_two(lines: Vec<String>) -> i32 {
    let sums = get_sums_in_descending_order(lines);
    sums[0] + sums[1] + sums[2]
}

pub fn get_sums_in_descending_order(lines: Vec<String>) -> Vec<i32> {
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
    amounts.push(current);
    amounts.sort_by(|a, b| b.cmp(a));

    amounts
}
