mod day04;

static SIX_ZEROS: &str = "000000";

fn main() {
    println!(
        "Number: {}",
        day04::calculate_number_for_string(day04::PUZZLE_INPUT, SIX_ZEROS)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_challenge_input() {
        assert_eq!(
            day04::calculate_number_for_string(day04::PUZZLE_INPUT, SIX_ZEROS),
            1038736u64
        );
    }
}
