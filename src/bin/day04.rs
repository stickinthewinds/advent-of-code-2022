use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args[1].to_string();

    let contents = utils::read_lines(file_path);

    utils::solve(day04::part_one, contents.clone(), Some("Number of pairs where one fully contains the other: "));
    utils::solve(day04::part_two, contents.clone(), Some("Number of pairs where the ranges overlap: "));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sample() {
        let input = String::from("resource/test/4.txt");
        let contents = utils::read_lines(input);
        assert_eq!(2, day04::part_one(contents.clone()));
        assert_eq!(4, day04::part_two(contents.clone()));
    }
}
