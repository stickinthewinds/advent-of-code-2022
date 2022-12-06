use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args[1].to_string();

    let contents = utils::read_lines(file_path);

    utils::solve(day05::part_one, contents.clone(), Some("The message is: "));
    utils::solve(day05::part_two, contents.clone(), Some("The message is: "));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sample() {
        let input = String::from("resource/test/5.txt");
        let contents = utils::read_lines(input);
        assert_eq!("CMZ", day05::part_one(contents.clone()));
        assert_eq!("MCD", day05::part_two(contents.clone()));
    }
}
