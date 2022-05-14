use std::{env, fs};

fn surface_area_of_box(length: u32, width: u32, height: u32) -> u32 {
    return 2 * (length * width + width * height + height * length);
}

/// Total surface area plus the area of the smallest side.
fn calculate_single_present_surface_area_needed(s: &str) -> u32 {
    let sorted_dim: Vec<u32> = day02::build_sorted_dimension_vec(s);
    // Area of smallest side is the product of the two smallest dimensions.
    return surface_area_of_box(sorted_dim[0], sorted_dim[1], sorted_dim[2])
        + sorted_dim[0] * sorted_dim[1];
}

fn calculate_total_wrapping_paper_needed(sv_vec: &Vec<&str>) -> u32 {
    let mut total_paper_needed: u32 = 0u32;
    sv_vec
        .iter()
        .for_each(|e| total_paper_needed += calculate_single_present_surface_area_needed(e));
    return total_paper_needed;
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 1 {
        panic!(
            "Expected one argument. Got {}.\n\nUsage: ./day02_part1 src/day02/data/part1_input.txt",
            args.len()
        );
    }
    let input_file_path: &str = args.first().unwrap();
    let file_content: String = fs::read_to_string(input_file_path).unwrap();

    let split_file_content: Vec<&str> = file_content.split("\n").collect();
    println!(
        "Total wrapping paper needed: {} square feet",
        calculate_total_wrapping_paper_needed(&split_file_content)
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sa_example_calc() {
        assert_eq!(surface_area_of_box(2, 3, 4), 52);
        assert_eq!(surface_area_of_box(1, 1, 10), 42);
    }

    #[test]
    fn dimension_vecs_sorted() {
        assert_eq!(day02::build_sorted_dimension_vec("2x2x2"), [2, 2, 2]);
        assert_eq!(day02::build_sorted_dimension_vec("3x2x1"), [1, 2, 3]);
        assert_eq!(day02::build_sorted_dimension_vec("7x8x9"), [7, 8, 9]);
    }

    #[test]
    #[should_panic]
    fn panic_on_negative_dim() {
        _ = day02::build_sorted_dimension_vec("1x-2x3");
    }

    #[test]
    #[should_panic]
    fn panic_on_zero_dim() {
        _ = day02::build_sorted_dimension_vec("99x99x0");
    }

    #[test]
    fn needed_wrapping_paper_example_calcs() {
        assert_eq!(calculate_single_present_surface_area_needed("2x3x4"), 58);
        assert_eq!(calculate_single_present_surface_area_needed("1x1x10"), 43);
    }

    #[test]
    fn calculate_required_paper_from_dims_list() {
        let v: Vec<&str> = vec!["2x3x4", "1x1x10"];
        assert_eq!(calculate_total_wrapping_paper_needed(&v), 58 + 43)
    }

    #[test]
    fn check_challenge_input() {
        let file_content: String = fs::read_to_string("src/day02/data/part1_input.txt").unwrap();

        let split_file_content: Vec<&str> = file_content.split("\n").collect();
        assert_eq!(
            calculate_total_wrapping_paper_needed(&split_file_content),
            1598415
        );
    }
}
