//! This is my solution for [Advent of Code - Day 6: _???_](https://adventofcode.com/2023/day/6)
//!
//! Today was solved with maths rather than brute force. The parsing is not much effort, but different for each part.
//! [`part_1_line_parser`] and [`part_2_line_parser`] contain these differences, and the relevant one is passed to
//! [`parse_input`] giving a list of [`Race`]s.
//!
//! [`find_count_of_winning_hold_times`] uses the quadratic formula to calculate the bounds, and therefore length of
//! the winning range of seconds to hold before releasing the boat. [`find_product_of_races`] can be used for both
//! parts, as the single race is unchanged by `iter().product`.

use std::fs;

/// A race duration, with the distance to beat in that time
#[derive(Eq, PartialEq, Debug)]
struct Race {
    duration: i64,
    distance_record: i64,
}

impl Race {
    fn new(time: i64, distance_record: i64) -> Race {
        Race {
            duration: time,
            distance_record,
        }
    }
}

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-6-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 6.
pub fn run() {
    let contents = fs::read_to_string("res/day-6-input.txt").expect("Failed to read file");

    println!(
        "The product of the number of ways to win is: {}",
        find_product_of_races(&parse_input(&contents, part_1_line_parser))
    );

    println!(
        "The number of ways to win the combined race is: {}",
        find_product_of_races(&parse_input(&contents, part_2_line_parser))
    );
}

/// Parse input from a line of durations and a line current record best times into a
/// list of records. How to parse each line is abstracted to a `line_parser` for each part
fn parse_input(input: &String, line_parser: fn(&str) -> Vec<i64>) -> Vec<Race> {
    let mut lines = input.split("\n");
    line_parser(lines.next().unwrap())
        .iter()
        .zip(line_parser(lines.next().unwrap()))
        .map(|(&t, d)| Race::new(t, d))
        .collect()
}

/// Parse lines as multiple numbers separated by whitespace
fn part_1_line_parser(line: &str) -> Vec<i64> {
    line.split(" ")
        .filter_map(|word| word.parse().ok())
        .collect()
}

/// Parse lines as a single number each
fn part_2_line_parser(line: &str) -> Vec<i64> {
    let num = line
        .chars()
        .filter_map(|chr| chr.to_digit(10))
        .fold(0i64, |acc, digit| acc * 10 + digit as i64);

    return vec![num];
}

/// Convert a list of races into the size of the range of hold times, and find the product of these as the puzzle
/// answer.
fn find_product_of_races(races: &Vec<Race>) -> i64 {
    races.iter().map(find_count_of_winning_hold_times).product()
}

/// Calculate the range of seconds the boat's button could be pressed for to exceed the current record for a race.
/// Uses the [quadratic formula](https://en.wikipedia.org/wiki/Quadratic_formula) to calculate the upper and lower
/// bound, and return the size of the range of integers within those bounds.
fn find_count_of_winning_hold_times(race: &Race) -> i64 {
    // `sqrt` and `/` expect to work with floats
    let duration = race.duration as f64;
    let record = race.distance_record as f64;

    let root_a = (duration + (duration.powf(2.0) - 4.0 * record).sqrt()) / 2.0;
    let root_b = (duration - (duration.powf(2.0) - 4.0 * record).sqrt()) / 2.0;

    // Inclusive. Floor here rounds down the the last excluded integer before the range, so then add one to get the
    // inclusive lower bound.
    let lower_bound = root_a.min(root_b).floor() as i64 + 1;
    // Exclusive. Ceil always gives the next integer beyond the range.
    let upper_bound = root_a.max(root_b).ceil() as i64;

    upper_bound - lower_bound
}

#[cfg(test)]
mod tests {
    use crate::day_6::*;

    #[test]
    fn can_parse_input_for_part_1() {
        assert_eq!(
            parse_input(&example_input(), part_1_line_parser),
            vec![Race::new(7, 9), Race::new(15, 40), Race::new(30, 200)]
        );
    }
    #[test]
    fn can_parse_input_for_part_2() {
        assert_eq!(
            parse_input(&example_input(), part_2_line_parser),
            vec![Race::new(71530, 940200)]
        );
    }

    fn example_input() -> String {
        "\
Time:      7  15   30
Distance:  9  40  200
"
        .to_string()
    }

    #[test]
    fn can_find_count_of_winning_hold_times() {
        assert_eq!(find_count_of_winning_hold_times(&Race::new(7, 9)), 4);
        assert_eq!(find_count_of_winning_hold_times(&Race::new(15, 40)), 8);
        assert_eq!(find_count_of_winning_hold_times(&Race::new(30, 200)), 9);
    }

    #[test]
    fn can_find_product_of_winning_hold_times() {
        assert_eq!(
            find_product_of_races(&parse_input(&example_input(), part_1_line_parser)),
            288
        );
    }

    #[test]
    fn can_find_hold_times_for_combined_race() {
        assert_eq!(
            find_product_of_races(&parse_input(&example_input(), part_2_line_parser)),
            71503
        );
    }
}
