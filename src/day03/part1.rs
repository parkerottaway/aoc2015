use std::collections::HashMap;
use std::{env, fs};

mod day03;

fn count_houses_visited(directions_string: &str) -> u16 {
    let move_north: day03::Point = day03::Point::new(1, 0);
    let move_south: day03::Point = day03::Point::new(-1, 0);
    let move_east: day03::Point = day03::Point::new(0, 1);
    let move_west: day03::Point = day03::Point::new(0, -1);

    let mut houses_visited: HashMap<day03::Point, ()> = HashMap::new();

    let mut current_house: day03::Point = day03::Point::new(0, 0);

    houses_visited.entry(current_house).or_insert(());

    directions_string
        .chars()
        .enumerate()
        .for_each(|(_, c)| match c {
            '^' => {
                current_house = current_house + move_north;
                houses_visited.entry(current_house).or_insert(());
            }
            'v' => {
                current_house = current_house + move_south;
                houses_visited.entry(current_house).or_insert(());
            }
            '>' => {
                current_house = current_house + move_east;
                houses_visited.entry(current_house).or_insert(());
            }
            '<' => {
                current_house = current_house + move_west;
                houses_visited.entry(current_house).or_insert(());
            }
            _ => panic!("Encountered unsupported char {}!", c),
        });

    return TryInto::<u16>::try_into(houses_visited.keys().len()).unwrap();
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 1 {
        panic!(
            "Expected one argument. Got {}.\n\nUsage: ./day03_part1 src/day03/data/part1_input.txt",
            args.len()
        );
    }
    let input_file_path: &str = args.first().unwrap();
    let file_content: String = fs::read_to_string(input_file_path).unwrap();

    println!(
        "Total number of houses visited: {}.",
        count_houses_visited(&file_content)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_addition_tests() {
        let origin: day03::Point = day03::Point::new(0, 0);
        let point_a: day03::Point = day03::Point::new(1, 0);
        let point_b: day03::Point = day03::Point::new(0, 1);
        let point_c: day03::Point = day03::Point::new(1, 1);
        assert_eq!(origin + origin, origin);
        assert_eq!(point_a + point_b, point_c);
    }

    #[test]
    fn point_subtraction_tests() {
        let origin: day03::Point = day03::Point::new(0, 0);
        let point_c: day03::Point = day03::Point::new(1, 1);
        assert_eq!(origin - origin, origin);
        assert_eq!(point_c - point_c, origin);
        assert_eq!(origin - point_c, day03::Point::new(-1, -1));
    }

    #[test]
    fn only_count_unique_points() {
        assert_eq!(count_houses_visited(""), 1u16);
        assert_eq!(count_houses_visited("^"), 2u16);
        assert_eq!(count_houses_visited(">>"), 3u16);
        assert_eq!(count_houses_visited("vvv"), 4u16);
        assert_eq!(count_houses_visited("<<<<"), 5u16);
        assert_eq!(count_houses_visited("<<<<>>>>"), 5u16);
        assert_eq!(count_houses_visited("<><>"), 2u16);
        assert_eq!(count_houses_visited("^v^v"), 2u16);
        assert_eq!(count_houses_visited("^>v<"), 4u16);
    }

    #[test]
    #[should_panic]
    fn unsupported_input_panics() {
        _ = count_houses_visited("^ ")
    }

    #[test]
    fn check_challenge_input() {
        let file_content: String = fs::read_to_string("src/day03/data/part1_input.txt").unwrap();

        assert_eq!(count_houses_visited(&file_content), 2565u16);
    }
}
