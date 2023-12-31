//! This is my solution for [Advent of Code - Day 1: _Trebuchet?_](https://adventofcode.com/2023/day/1)
//!
//! [`parse_line`] does most of the heavy lifting, using the provided regex to find digits in a line, and then turn
//! those into a calibration value. It defers to [`overlapping_matches`] to handle getting regex matches that might
//! overlap. [`sum_calibration_values`] reduces the values into the final puzzle answer.
//!
//! [`ValueExtractor`]s are used to codify the different logic for the two parts, see [`part_1_extractor`] and
//! [`part_2_extractor`].

use itertools::unfold;
use regex::Regex;
use std::fs;

/// Describes how to find digits in a string, and how to turn those into their numeric representation
struct ValueExtractor {
    pattern: Regex,
    digit_mapper: fn(&str) -> u32,
}
/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-1-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 1.
pub fn run() {
    let contents = fs::read_to_string("res/day-1-input.txt").expect("Failed to read file");

    println!(
        "The sum of calibration values is {}",
        sum_calibration_values(&contents, &part_1_extractor())
    );
    println!(
        "The sum of calibration values with digit strings is {}",
        sum_calibration_values(&contents, &part_2_extractor())
    );
}

fn part_1_extractor() -> ValueExtractor {
    ValueExtractor {
        pattern: Regex::new(r"\d").unwrap(),
        digit_mapper: |d| d.parse().unwrap(),
    }
}

fn part_2_extractor() -> ValueExtractor {
    ValueExtractor {
        pattern: Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap(),
        digit_mapper: |d| match d {
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9,
            _ => d.parse().unwrap(),
        },
    }
}

/// Reduce the value extracted in each line to the sum required as the puzzle answer.
fn sum_calibration_values(input: &String, extractor: &ValueExtractor) -> u32 {
    input.lines().map(|line| parse_line(line, &extractor)).sum()
}

/// Return regex matches that might overlap
///
/// ```rust
/// let pattern = Regex::new(r"(eight|three)").unwrap();
/// let res: Vec<&str> = overlapping_matches("eighthree", &pattern);
/// assert_eq!(res, vec!("eight", "three"));
/// ```
fn overlapping_matches<'a>(line: &'a str, pattern: &Regex) -> Vec<&'a str> {
    unfold(0usize, |pos| {
        // Find the next match
        let digit = pattern.find_at(line, *pos);
        // The next iteration should start from the next character after
        // the match to allow for overlaps
        *pos = digit.map(|m| m.start()).unwrap_or(0) + 1;
        // For convenience return only the match's contents
        digit.map(|m| m.as_str())
    })
    .collect()
}

/// Use the logic in the provided extractor to find all matches, then take the first and last and combine them into a
/// two digit number.
fn parse_line(line: &str, extractor: &ValueExtractor) -> u32 {
    let matches: Vec<&str> = overlapping_matches(line, &extractor.pattern);

    let tens = matches
        .first()
        .map(|&s| (extractor.digit_mapper)(s))
        .unwrap_or(0);

    let units = matches
        .last()
        .map(|&s| (extractor.digit_mapper)(s))
        .unwrap_or(0);

    tens * 10 + units
}

#[cfg(test)]
mod tests {
    use crate::day_1::*;

    #[test]
    fn can_parse_lines() {
        let part_1_extractor = part_1_extractor();

        assert_eq!(parse_line("1abc2", &part_1_extractor), 12);
        assert_eq!(parse_line("pqr3stu8vwx", &part_1_extractor), 38);
        assert_eq!(parse_line("a1b2c3d4e5f", &part_1_extractor), 15);
        assert_eq!(parse_line("treb7uchet", &part_1_extractor), 77);

        let part_2_extractor = part_2_extractor();

        assert_eq!(parse_line("two1nine", &part_2_extractor), 29);
        assert_eq!(parse_line("eightwothree", &part_2_extractor), 83);
        assert_eq!(parse_line("abcone2threexyz", &part_2_extractor), 13);
        assert_eq!(parse_line("xtwone3four", &part_2_extractor), 24);
        assert_eq!(parse_line("4nineeightseven2", &part_2_extractor), 42);
        assert_eq!(parse_line("zoneight234", &part_2_extractor), 14);
        assert_eq!(parse_line("7pqrstsixteen", &part_2_extractor), 76);

        assert_eq!(parse_line("five", &part_2_extractor), 55);
        assert_eq!(parse_line("eighthree", &part_2_extractor), 83);
    }

    #[test]
    fn can_sum_calibration_values() {
        let part_1_input = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
            .to_string();

        assert_eq!(
            sum_calibration_values(&part_1_input, &part_1_extractor()),
            142
        );

        let part_2_input = "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
            .to_string();

        assert_eq!(
            sum_calibration_values(&part_2_input, &part_2_extractor()),
            281
        );
    }
}
