use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args[1].to_string();

    let contents = utils::read_lines(file_path);

    utils::solve(day07::part_one, contents.clone(), Some("Total sum of directories smaller than 100_000: "));
    utils::solve(day07::part_two, contents.clone(), Some("Size of smallest directory that needs to be deleted: "));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sample() {
        let input = String::from("resource/test/7.txt");
        let contents = utils::read_lines(input);
        assert_eq!(95437, day07::part_one(contents.clone()));
        assert_eq!(24933642, day07::part_two(contents.clone()));
    }
}
