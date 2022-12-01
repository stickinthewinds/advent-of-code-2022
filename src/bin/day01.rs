use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args[1].to_string();

    let contents = utils::read_lines(file_path);

    utils::solve(day01::part_one, contents.clone(), Some("Highest amount is: "));
    utils::solve(day01::part_two, contents, Some("Combined total for the top three is: "));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sample() {
        let input = String::from("resource/test/1.txt");
        let contents = utils::read_lines(input);
        assert_eq!(24000, day01::part_one(contents.clone()));
        assert_eq!(45000, day01::part_two(contents));

    }
}
