use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args[1].to_string();

    let contents = utils::read_lines(file_path);

    utils::solve(day02::part_one, contents.clone(), Some("Total score is: "));
    utils::solve(day02::part_two, contents.clone(), Some("Total score is: "));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sample() {
        let input = String::from("resource/test/2.txt");
        let contents = utils::read_lines(input);
        assert_eq!(15, day02::part_one(contents.clone()));
        assert_eq!(12, day02::part_two(contents));
    }
}
