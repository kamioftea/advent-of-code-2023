//! This is my solution for [Advent of Code - Day 6: _???_](https://adventofcode.com/2023/day/6)
//!
//!

use std::fs;

#[derive(Eq, PartialEq, Debug)]
struct Race {
    time: i64,
    distance_record: i64,
}

impl Race {
    fn new(time: i64, distance_record: i64) -> Race {
        Race {
            time,
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
        solve_part_1(&contents)
    );

    println!(
        "The number of ways to win the combined race is: {}",
        solve_part_2(&contents)
    );
}

fn solve_part_2(contents: &String) -> i64 {
    find_count_of_winning_hold_times(parse_input(&contents, part_2_line_parser).get(0).unwrap())
}

fn solve_part_1(contents: &String) -> i64 {
    parse_input(&contents, part_1_line_parser)
        .iter()
        .map(find_count_of_winning_hold_times)
        .fold(1, |acc, num| acc * num)
}

fn parse_input(input: &String, line_parser: fn(&str) -> Vec<i64>) -> Vec<Race> {
    let mut lines = input.split("\n");
    line_parser(lines.next().unwrap())
        .iter()
        .zip(line_parser(lines.next().unwrap()))
        .map(|(&t, d)| Race::new(t, d))
        .collect()
}

fn part_1_line_parser(line: &str) -> Vec<i64> {
    line.split(" ")
        .filter_map(|word| word.parse().ok())
        .collect()
}

fn part_2_line_parser(line: &str) -> Vec<i64> {
    let num = line
        .chars()
        .filter_map(|chr| chr.to_digit(10))
        .fold(0i64, |acc, digit| acc * 10 + digit as i64);

    return vec![num];
}

fn find_count_of_winning_hold_times(race: &Race) -> i64 {
    let root_a =
        (race.time as f64 + f64::sqrt((race.time.pow(2) - 4 * race.distance_record) as f64)) / 2f64;
    let root_b =
        (race.time as f64 - f64::sqrt((race.time.pow(2) - 4 * race.distance_record) as f64)) / 2f64;

    let lower_bound = root_a.min(root_b);
    let lower_bound_inclusive =
        (lower_bound.ceil() - (lower_bound.ceil() - lower_bound.floor()).ceil()) as i64 + 1;

    let upper_bound = root_a.max(root_b);
    let upper_bound_exclusive =
        (upper_bound.floor() + (upper_bound.ceil() - upper_bound.floor()).ceil()) as i64;

    upper_bound_exclusive - lower_bound_inclusive
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
    fn can_solve_part_1() {
        assert_eq!(solve_part_1(&example_input()), 288);
    }

    #[test]
    fn can_solve_part_2() {
        assert_eq!(solve_part_2(&example_input()), 71503);
    }
}
