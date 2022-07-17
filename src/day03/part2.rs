use std::collections::HashMap;
use std::{env, fs};

mod day03;

fn count_houses_visited(directions_string: &str) -> u16 {
    let move_north: day03::Point = day03::Point::new(1, 0);
    let move_south: day03::Point = day03::Point::new(-1, 0);
    let move_east: day03::Point = day03::Point::new(0, 1);
    let move_west: day03::Point = day03::Point::new(0, -1);

    let mut houses_visited: HashMap<day03::Point, ()> = HashMap::new();

    let mut human_santa_current_house: day03::Point = day03::Point::new(0, 0);
    let mut robot_santa_current_house: day03::Point = day03::Point::new(0, 0);

    houses_visited
        .entry(human_santa_current_house)
        .or_insert(());

    directions_string
        .chars()
        .enumerate()
        .for_each(|(idx, c)| match c {
            '^' => {
                if idx % 2 == 0 {
                    human_santa_current_house = human_santa_current_house + move_north;
                    houses_visited
                        .entry(human_santa_current_house)
                        .or_insert(());
                } else {
                    robot_santa_current_house = robot_santa_current_house + move_north;
                    houses_visited
                        .entry(robot_santa_current_house)
                        .or_insert(());
                }
            }
            'v' => {
                if idx % 2 == 0 {
                    human_santa_current_house = human_santa_current_house + move_south;
                    houses_visited
                        .entry(human_santa_current_house)
                        .or_insert(());
                } else {
                    robot_santa_current_house = robot_santa_current_house + move_south;
                    houses_visited
                        .entry(robot_santa_current_house)
                        .or_insert(());
                }
            }
            '>' => {
                if idx % 2 == 0 {
                    human_santa_current_house = human_santa_current_house + move_east;
                    houses_visited
                        .entry(human_santa_current_house)
                        .or_insert(());
                } else {
                    robot_santa_current_house = robot_santa_current_house + move_east;
                    houses_visited
                        .entry(robot_santa_current_house)
                        .or_insert(());
                }
            }
            '<' => {
                if idx % 2 == 0 {
                    human_santa_current_house = human_santa_current_house + move_west;
                    houses_visited
                        .entry(human_santa_current_house)
                        .or_insert(());
                } else {
                    robot_santa_current_house = robot_santa_current_house + move_west;
                    houses_visited
                        .entry(robot_santa_current_house)
                        .or_insert(());
                }
            }
            _ => panic!("Encountered unsupported char {}!", c),
        });

    return TryInto::<u16>::try_into(houses_visited.keys().len()).unwrap();
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 1 {
        panic!("Expected one argument. Got {}.\n\nUsage: ", args.len());
    }
    let input_file_path: &str = args.first().unwrap();
    let file_content: String = fs::read_to_string(input_file_path).unwrap();

    println!(
        "Total number of houses visited by both santas: {}.",
        count_houses_visited(&file_content)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        assert_eq!(count_houses_visited("^v"), 3u16);
        assert_eq!(count_houses_visited("^>v<"), 3u16);
        assert_eq!(count_houses_visited("^v^v^v^v^v"), 11u16);
    }

    #[test]
    fn check_challenge_input() {
        let file_content: String = fs::read_to_string("src/day03/data/part1_input.txt").unwrap();

        assert_eq!(count_houses_visited(&file_content), 2639u16);
    }
}
