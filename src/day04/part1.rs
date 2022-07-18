use std::fmt::Write as FmtWrite;

use md5;

static PUZZLE_INPUT: &str = "bgvyzdsv";

fn calculate_number_for_string(s: &str) -> u64 {
    for n in 0..std::u64::MAX {
        let mut key_number_string = String::new();
        let mut digest_as_string = String::new();

        write!(&mut key_number_string, "{}{}", s, n.to_string()).expect("Not written");
        let digest = md5::compute(key_number_string.as_bytes());
        write!(&mut digest_as_string, "{:x}", digest).expect("Not written");

        if digest_as_string.starts_with("00000") {
            return n;
        }
    }
    panic!("There exists no u64 that, when appended to {}, results in five leading zeros in MD5 sum", s);
}

fn main() {
    println!("Hash: {}", calculate_number_for_string(PUZZLE_INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        assert_eq!(calculate_number_for_string("abcdef"), 609043u64);
        assert_eq!(calculate_number_for_string("pqrstuv"), 1048970u64);
    }

    #[test]
    fn check_challenge_input() {
        assert_eq!(calculate_number_for_string(PUZZLE_INPUT), 254575u64);
    }
}
