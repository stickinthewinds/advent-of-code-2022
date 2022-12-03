use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args[1].to_string();

    let contents = utils::read_lines(file_path);

    utils::solve(day03::part_one, contents.clone(), Some("Sum of priorities is: "));
    utils::solve(day03::part_two, contents.clone(), Some("Sum of priorities is: "));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sample() {
        let input = String::from("resource/test/3.txt");
        let contents = utils::read_lines(input);
        assert_eq!(157, day03::part_one(contents.clone()));
        assert_eq!(70, day03::part_two(contents.clone()));
    }
}
