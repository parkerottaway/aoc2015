use std::{env, fs};

fn string_has_two_letters_twice(s: &str) -> bool {
    if s.len() < 4 {
        return false;
    }
    let mut str_vec: Vec<&str> = Vec::new();
    for idx in 0..s.len() - 1 {
        str_vec.push(&s[idx..idx + 2]);
    }
    for idx in 0..str_vec.len() - 1 {
        for idy in idx + 2..str_vec.len() {
            if str_vec[idx] == str_vec[idy] {
                return true;
            }
        }
    }
    return false;
}

fn string_has_one_letter_repeat_with_one_in_between(s: &str) -> bool {
    if s.len() < 3 {
        return false;
    }
    for idx in 0..s.len() - 2 {
        if s[idx..idx + 1] == s[idx + 2..idx + 3] {
            return true;
        }
    }
    return false;
}

fn decide_string(s: &str) -> &str {
    return if string_has_two_letters_twice(s) && string_has_one_letter_repeat_with_one_in_between(s)
    {
        "nice"
    } else {
        "naughty"
    };
}

fn count_nice_strings(sv_vec: &Vec<&str>) -> u32 {
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
            "Expected one argument. Got {}.\n\nUsage: target/debug/day05_part2 src/day05/data/input.txt",
            args.len()
        );
    }
    let input_file_path: &str = args.first().unwrap();
    let file_content: String = fs::read_to_string(input_file_path).unwrap();

    let split_file_content: Vec<&str> = file_content.split("\n").collect();
    println!(
        "Total number of nice strings: {}",
        count_nice_strings(&split_file_content)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_each_property() {
        assert_eq!(string_has_two_letters_twice("xyxy"), true);
        assert_eq!(string_has_two_letters_twice("aabcdefgaa"), true);
        assert_eq!(string_has_two_letters_twice("aaa"), false);

        assert_eq!(
            string_has_one_letter_repeat_with_one_in_between("xyx"),
            true
        );
        assert_eq!(
            string_has_one_letter_repeat_with_one_in_between("abcdefeghi"),
            true
        );
        assert_eq!(
            string_has_one_letter_repeat_with_one_in_between("aaa"),
            true
        );
        assert_eq!(
            string_has_one_letter_repeat_with_one_in_between("xyz"),
            false
        );
        assert_eq!(
            string_has_one_letter_repeat_with_one_in_between("abcdefghijk"),
            false
        );
    }

    #[test]
    fn check_examples() {
        assert_eq!(decide_string("qjhvhtzxzqqjkmpb"), "nice");
        assert_eq!(decide_string("xxyxx"), "nice");
        assert_eq!(decide_string("uurcxstgmygtbstg"), "naughty");
        assert_eq!(decide_string("ieodomkazucvgmuy"), "naughty");
    }

    #[test]
    fn check_challenge_input() {
        let file_content: String = fs::read_to_string("src/day05/data/input.txt").unwrap();
        let split_file_content: Vec<&str> = file_content.split("\n").collect();

        assert_eq!(count_nice_strings(&split_file_content), 53u32);
    }
}
