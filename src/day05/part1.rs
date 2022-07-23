use std::{env, fs};

fn string_contains_at_least_three_vowels(s: &str) -> bool {
    let mut vowels_encountered: u32 = 0u32;
    for c in s.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                vowels_encountered += 1;
            }
            _ => (),
        }
    }
    return vowels_encountered >= 3;
}

fn string_has_one_letter_twice_in_a_row(s: &str) -> bool {
    let mut chars_iter = s.chars();
    let mut current_char = chars_iter.next().unwrap();
    for c in chars_iter {
        if current_char == c {
            return true;
        } else {
            current_char = c;
        }
    }
    return false;
}

fn string_is_nice(s: &str) -> bool {
    return string_contains_at_least_three_vowels(s) && string_has_one_letter_twice_in_a_row(s);
}

fn string_is_not_nice(s: &str) -> bool {
    return s.contains("ab") || s.contains("cd") || s.contains("pq") || s.contains("xy");
}

fn decide_string(s: &str) -> &str {
    return if string_is_nice(s) && !string_is_not_nice(s) {
        "nice"
    } else {
        "naughty"
    };
}

fn count_number_of_nice_strings(sv_vec: &Vec<&str>) -> u32 {
    let mut nice_strings_count: u32 = 0u32;
    sv_vec.iter().for_each(|e| {
        if decide_string(e) == "nice" {
            nice_strings_count += 1
        }
    });
    return nice_strings_count;
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 1 {
        panic!(
            "Expected one argument. Got {}.\n\nUsage: target/debug/day05_part1 src/day05/data/input.txt",
            args.len()
        );
    }
    let input_file_path: &str = args.first().unwrap();
    let file_content: String = fs::read_to_string(input_file_path).unwrap();

    let split_file_content: Vec<&str> = file_content.split("\n").collect();
    println!(
        "Total number of nice strings: {}",
        count_number_of_nice_strings(&split_file_content)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_examples() {
        assert_eq!(decide_string("ugknbfddgicrmopn"), "nice");
        assert_eq!(decide_string("aaa"), "nice");
        assert_eq!(decide_string("jchzalrnumimnmhp"), "naughty");
        assert_eq!(decide_string("haegwjzuvuyypxyu"), "naughty");
        assert_eq!(decide_string("dvszwmarrgswjxmb"), "naughty");
    }

    #[test]
    fn check_challenge_input() {
        let file_content: String = fs::read_to_string("src/day05/data/input.txt").unwrap();
        let split_file_content: Vec<&str> = file_content.split("\n").collect();

        assert_eq!(count_number_of_nice_strings(&split_file_content), 258u32);
    }
}
