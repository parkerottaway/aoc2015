use std::{env, fs, io};

fn parse_directions_string(sv: &str) -> i32 {
    let mut current_floor: i32 = 0;

    sv.chars().enumerate().for_each(|(_, c)| match c {
        '(' => current_floor += 1,
        ')' => current_floor -= 1,
        _ => panic!("Encountered unsupported char \'{}\'.", c),
    });

    return current_floor;
}

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 1 {
        panic!(
            "Expected one argument. Got {}.\n\nUsage: ./day01 src/day01/data/part1_input.txt",
            args.len()
        );
    }
    let input_file_path: &str = args.first().unwrap();
    let file_content = fs::read_to_string(input_file_path)?;
    let resulting_floor: i32 = parse_directions_string(&file_content);

    println!("Resulting floor: {}", resulting_floor);

    return Ok(());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empy_string_returns_zero() {
        assert_eq!(0, parse_directions_string(""));
    }

    #[test]
    #[should_panic]
    fn non_parenthesis_causes_panic() {
        _ = parse_directions_string("( ")
    }

    #[test]
    fn open_parenthesis_increments_floor_counter() {
        assert_eq!(1, parse_directions_string("("));
    }

    #[test]
    fn closed_parenthesis_decrements_floor_counter() {
        assert_eq!(-1, parse_directions_string(")"));
    }

    #[test]
    fn different_patterns_same_result() {
        assert_eq!(
            parse_directions_string("(())"),
            parse_directions_string("()()")
        );
        assert_eq!(
            parse_directions_string("((("),
            parse_directions_string("(()(()(")
        );
        assert_eq!(
            parse_directions_string("())"),
            parse_directions_string("))(")
        );
        assert_eq!(
            parse_directions_string(")))"),
            parse_directions_string(")())())")
        );
    }

    #[test]
    fn check_challenge_input() {
        let file_content = fs::read_to_string("src/day01/data/part1_input.txt");
        let resulting_floor: i32 = parse_directions_string(&file_content.unwrap());
        assert_eq!(232, resulting_floor);
    }
}
