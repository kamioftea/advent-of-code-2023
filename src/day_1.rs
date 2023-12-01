//! This is my solution for [Advent of Code - Day 1: _Trebuchet?_](https://adventofcode.com/2023/day/1)
//!
//!

use regex::Regex;
use std::fs;

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

fn sum_calibration_values(input: &String, extractor: &ValueExtractor) -> u32 {
    input.lines().map(|line| parse_line(line, &extractor)).sum()
}

fn parse_line(line: &str, extractor: &ValueExtractor) -> u32 {
    fn iter(
        line: &str,
        extractor: &ValueExtractor,
        pos: usize,
        tens: Option<u32>,
        units: Option<u32>,
    ) -> u32 {
        match extractor.pattern.find_at(line, pos) {
            Some(m) => {
                let value = (extractor.digit_mapper)(m.as_str());

                iter(
                    line,
                    extractor,
                    m.start() + 1,
                    tens.or(Some(value)),
                    Some(value),
                )
            }
            None => tens.unwrap_or(0) * 10 + units.unwrap_or(0),
        }
    }

    iter(line, extractor, 0, None, None)
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
