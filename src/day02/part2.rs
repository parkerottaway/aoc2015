use std::{env, fs};

fn calc_smallest_face_perimeter(dimensions_vec: &Vec<u32>) -> u32 {
    return 2 * dimensions_vec[0] + 2 * dimensions_vec[1];
}

/// Length of ribbon needed is equal to the value of the volume
/// of the present.
fn calc_bow_ribbon_length(dimensions_vec: &Vec<u32>) -> u32 {
    return dimensions_vec.iter().fold(1, |acc, e| acc * e);
}

fn calc_ribbon_needed_for_one_present(dimensions_vec: &Vec<u32>) -> u32 {
    return calc_smallest_face_perimeter(dimensions_vec) + calc_bow_ribbon_length(dimensions_vec);
}

fn calc_total_ribbon_length_needed(all_dim_strings: &Vec<&str>) -> u32 {
    return all_dim_strings.iter().fold(0, |acc, s| {
        acc + calc_ribbon_needed_for_one_present(&day02::build_sorted_dimension_vec(s))
    });
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 1 {
        panic!(
            "Expected one argument. Got {}.\n\nUsage: ./day02_part2 src/day02/data/part1_input.txt",
            args.len()
        );
    }
    let input_file_path: &str = args.first().unwrap();
    let file_content: String = fs::read_to_string(input_file_path).unwrap();

    let split_file_content: Vec<&str> = file_content.split("\n").collect();
    println!(
        "Total length of ribbon needed: {} feet",
        calc_total_ribbon_length_needed(&split_file_content)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smallest_perimenter_examples() {
        assert_eq!(calc_smallest_face_perimeter(&vec![2, 3, 4]), 10);
        assert_eq!(calc_smallest_face_perimeter(&vec![1, 1, 10]), 4);
    }

    #[test]
    fn ribbon_length_examples() {
        assert_eq!(calc_bow_ribbon_length(&vec![2, 3, 4]), 24);
        assert_eq!(calc_bow_ribbon_length(&vec![1, 1, 10]), 10);
    }

    #[test]
    fn total_ribbon_examples() {
        assert_eq!(calc_ribbon_needed_for_one_present(&vec![2, 3, 4]), 34);
        assert_eq!(calc_ribbon_needed_for_one_present(&vec![1, 1, 10]), 14);
    }

    #[test]
    fn check_challenge_input() {
        let file_content: String = fs::read_to_string("src/day02/data/part1_input.txt").unwrap();

        let split_file_content: Vec<&str> = file_content.split("\n").collect();
        assert_eq!(
            calc_total_ribbon_length_needed(&split_file_content),
            3812909
        );
    }
}
