mod day04;

static FIVE_ZEROS: &str = "00000";

fn main() {
    println!(
        "Number: {}",
        day04::calculate_number_for_string(day04::PUZZLE_INPUT, FIVE_ZEROS)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        assert_eq!(
            day04::calculate_number_for_string("abcdef", FIVE_ZEROS),
            609043u64
        );
        assert_eq!(
            day04::calculate_number_for_string("pqrstuv", FIVE_ZEROS),
            1048970u64
        );
    }

    #[test]
    fn check_challenge_input() {
        assert_eq!(
            day04::calculate_number_for_string(day04::PUZZLE_INPUT, FIVE_ZEROS),
            254575u64
        );
    }
}
