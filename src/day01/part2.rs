use std::{env, fs, io};

/// The "basement" is reached when the `current_floor` is -1. As soon as
/// `current_floor` is -1, return the index of the character that caused
/// `current_floor` to become zero.
fn find_index_of_first_basement_encounter(sv: &str) -> Result<usize, io::Error> {
    let floor_stop_condition: i32 = -1;
    let mut current_floor: i32 = 0;
    let mut idx_where_first_entered_basement: Option<usize> = None;

    sv.chars().enumerate().for_each(|(idx, c)| match c {
        '(' => current_floor += 1,
        ')' => {
            current_floor -= 1;
            if current_floor == floor_stop_condition && idx_where_first_entered_basement.is_none() {
                idx_where_first_entered_basement = Some(idx);
            }
        }
        _ => panic!("Encountered unsupported char \'{}\'.", c),
    });
    match idx_where_first_entered_basement {
        // Increment the index (the char position is one-indexed).
        Some(x) => return Ok(x + 1),
        None => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Santa never reached the basement...",
            ))
        }
    }
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
    let first_basement_encounter_idx: usize =
        find_index_of_first_basement_encounter(&file_content).unwrap();

    println!(
        "Character index that causes Santa to first enter the basement: {}",
        first_basement_encounter_idx
    );

    return Ok(());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_thrown_when_no_path_to_basement() {
        let result = find_index_of_first_basement_encounter("(((").map_err(|e| e.kind());
        let expected = Err(std::io::ErrorKind::InvalidInput);
        assert_eq!(expected, result);
    }

    #[test]
    fn immediately_enter_basement() {
        assert_eq!(1, find_index_of_first_basement_encounter(")").unwrap());
    }

    #[test]
    fn up_then_plumit_to_basement() {
        assert_eq!(
            7,
            find_index_of_first_basement_encounter("((())))").unwrap()
        );
    }

    #[test]
    fn only_return_first_index_when_basement_reached() {
        assert_eq!(1, find_index_of_first_basement_encounter(")(())").unwrap());
    }

    #[test]
    fn check_challenge_input() {
        let file_content = fs::read_to_string("src/day01/data/part1_input.txt");
        let first_idx_basement_reached: usize =
            find_index_of_first_basement_encounter(&file_content.unwrap()).unwrap();
        assert_eq!(1783, first_idx_basement_reached);
    }
}
