use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args[1].to_string();

    let contents = utils::read_lines(file_path);

    utils::solve(day06::part_one, contents.get(0).unwrap().to_string(), Some("The first start-of-packet marker is after: "));
    utils::solve(day06::part_two, contents.get(0).unwrap().to_string(), Some("The first start-of-message marker is after: "));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sample() {
        let input = String::from("resource/test/6.txt");
        let contents = utils::read_lines(input);
        assert_eq!(7, day06::part_one(contents.get(0).unwrap().to_string()));
        assert_eq!(5, day06::part_one(contents.get(1).unwrap().to_string()));
        assert_eq!(6, day06::part_one(contents.get(2).unwrap().to_string()));
        assert_eq!(10, day06::part_one(contents.get(3).unwrap().to_string()));
        assert_eq!(11, day06::part_one(contents.get(4).unwrap().to_string()));
    }
}
