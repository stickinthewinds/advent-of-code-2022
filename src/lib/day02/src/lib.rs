// Total score when X/Y/Z is your choice
pub fn part_one(lines: Vec<String>) -> i32 {
    convert_lines_to_pairs(lines)
        .iter()
        .map(|(o, y)| get_score_for_round(*o, *y))
        .sum()
}

// Total score when X/Y/Z is the round result
pub fn part_two(lines: Vec<String>) -> i32 {
    convert_lines_to_pairs(lines)
        .iter()
        .map(|(o, y)| get_score_for_round(*o, determine_choice(*o, *y)))
        .sum()
}

fn letter_as_number(c: char) -> i32 {
    c.to_ascii_uppercase() as i32 - 64 // 'A' is 65 in ascii
}

/// Scoring system is:
/// 6 points for a win
/// 3 points for a draw
/// 0 points for a loss
/// 3 points for scissors
/// 2 points for paper
/// 1 point for rock
fn get_score_for_round(o: i32, y: i32) -> i32 {
    let result = (y - o).rem_euclid(3);
    if result == 0 { // draw
        return y + 3;
    } else if result == 1 { // win
        return y + 6;
    }
    y
}

// 1 means lose 2 means draw 3 means win
fn determine_choice(o: i32, y: i32) -> i32 {
    if y == 1 {
        return if o > 1 {o - 1} else {3};
    } else if y == 3 {
        return if o > 2 {1} else {o + 1};
    }
    o
}

fn convert_lines_to_pairs(lines: Vec<String>) -> Vec<(i32, i32)> {
    lines
        .iter()
        .map(|l| l.split_whitespace())
        .map(|mut v| {
            let o = letter_as_number(get_choice(v.next()));
            let y = letter_as_number(get_choice(v.next())) - 23;
            (o, y)
        })
        .collect()
}

fn get_choice(c: Option<&str>) -> char {
    c
        .expect("No choice for play?")
        .chars()
        .next()
        .expect("Not a valid choice.")
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_get_score_for_round() {
        assert_eq!(8, get_score_for_round(1, 2));
        assert_eq!(1, get_score_for_round(2, 1));
        assert_eq!(6, get_score_for_round(3, 3));
        assert_eq!(4, get_score_for_round(1, 1));
        assert_eq!(7, get_score_for_round(3, 1));
    }

    #[test]
    fn test_determine_choice() {
        assert_eq!(3, determine_choice(1, 1));
        assert_eq!(1, determine_choice(1, 2));
        assert_eq!(2, determine_choice(1, 3));
        assert_eq!(1, determine_choice(2, 1));
        assert_eq!(2, determine_choice(2, 2));
        assert_eq!(3, determine_choice(2, 3));
        assert_eq!(2, determine_choice(3, 1));
        assert_eq!(3, determine_choice(3, 2));
        assert_eq!(1, determine_choice(3, 3));
    }

    #[test]
    fn test_convert_lines_to_pairs() {
        let lines = vec!["A Y".to_string(), "B X".into(), "C Z".into()];
        let result = convert_lines_to_pairs(lines);
        assert_eq!((1, 2), result[0]);
        assert_eq!((2, 1), result[1]);
        assert_eq!((3, 3), result[2]);
    }

    #[test]
    fn test_get_choice() {
        assert_eq!('A', get_choice(Some("A")));
        assert_eq!('B', get_choice(Some("B")));
        assert_eq!('C', get_choice(Some("C")));
        assert_eq!('Y', get_choice(Some("Y")));
        assert_eq!('X', get_choice(Some("X")));
        assert_eq!('Z', get_choice(Some("Z")));
    }
}
