//! This is my solution for [Advent of Code - Day 1 - _???_](https://adventofcode.com/2023/day/1)
//!
//!

use std::fs;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-1-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 1.
pub fn run() {
    let contents = fs::read_to_string("res/day-1-input.txt").expect("Failed to read file");

    println!(
        "The sum of calibration values is {}",
        sum_calibration_values(&contents)
    );
    println!(
        "The sum of calibration values with digit strings is {}",
        sum_calibration_values_with_substitution(&contents)
    );
}

fn sum_calibration_values(input: &String) -> u32 {
    input.lines().map(parse_line).sum()
}

fn sum_calibration_values_with_substitution(input: &String) -> u32 {
    input
        .lines()
        .map(|line| parse_line(substitute_digit_strings(line).as_str()))
        .sum()
}

fn parse_line(line: &str) -> u32 {
    let digits: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();

    digits.first().unwrap_or(&0) * 10 + digits.last().unwrap_or(&0)
}

fn substitute_digit_strings(line: &str) -> String {
    line.replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "4")
        .replace("five", "5e")
        .replace("six", "6")
        .replace("seven", "7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e")
}

#[cfg(test)]
mod tests {
    use crate::day_1::*;

    #[test]
    fn can_parse_lines() {
        assert_eq!(parse_line("1abc2"), 12);
        assert_eq!(parse_line("pqr3stu8vwx"), 38);
        assert_eq!(parse_line("a1b2c3d4e5f"), 15);
        assert_eq!(parse_line("treb7uchet"), 77);
    }

    #[test]
    fn can_sum_calibration_values() {
        let input = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
            .to_string();

        assert_eq!(sum_calibration_values(&input), 142)
    }

    #[test]
    fn can_substitute_digit_words() {
        assert_eq!(substitute_digit_strings("two1nine"), "t2o1n9e");
        assert_eq!(substitute_digit_strings("eightwothree"), "e8t2ot3e");
        assert_eq!(substitute_digit_strings("abcone2threexyz"), "abco1e2t3exyz");
        assert_eq!(substitute_digit_strings("xtwone3four"), "xt2o1e34");
        assert_eq!(substitute_digit_strings("4nineeightseven2"), "4n9ee8t7n2");
        assert_eq!(substitute_digit_strings("zoneight234"), "zo1e8t234");
        assert_eq!(substitute_digit_strings("7pqrstsixteen"), "7pqrst6teen");
        assert_eq!(substitute_digit_strings("five"), "5e");
    }

    #[test]
    fn can_sum_calibration_values_with_string_digits() {
        let input = "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
            .to_string();

        assert_eq!(sum_calibration_values_with_substitution(&input), 281)
    }
}
