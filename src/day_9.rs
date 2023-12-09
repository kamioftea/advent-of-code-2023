//! This is my solution for [Advent of Code - Day 9: _???_](https://adventofcode.com/2023/day/9)
//!
//!

use itertools::{iterate, Itertools};
use std::fs;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-9-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 9.
pub fn run() {
    let contents = fs::read_to_string("res/day-9-input.txt").expect("Failed to read file");

    let sequences = parse_input(&contents);

    println!(
        "The sum of the forwards extrapolated numbers is: {}",
        analyse_sequences(&sequences, extrapolate_sequence_forwards)
    );

    println!(
        "The sum of the backwards extrapolated numbers is: {}",
        analyse_sequences(&sequences, extrapolate_sequence_backwards)
    );
}

fn parse_input(input: &String) -> Vec<Vec<i64>> {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> Vec<i64> {
    line.split(" ").filter_map(|num| num.parse().ok()).collect()
}

fn extrapolate_sequence_forwards(sequence: &Vec<i64>) -> i64 {
    let sequences: Vec<Vec<i64>> = power_sequences(sequence);

    sequences
        .iter()
        .rev()
        .map(|seq| seq.last().unwrap_or(&0))
        .sum()
}

fn extrapolate_sequence_backwards(sequence: &Vec<i64>) -> i64 {
    let sequences: Vec<Vec<i64>> = power_sequences(sequence);

    sequences
        .iter()
        .rev()
        .map(|seq| seq.first().unwrap_or(&0))
        .fold(0, |acc, val| val - acc)
}

fn power_sequences(sequence: &Vec<i64>) -> Vec<Vec<i64>> {
    iterate(sequence.clone(), build_diff_sequence)
        .take_while(|seq| seq.iter().any(|&v| v != 0))
        .collect()
}

fn build_diff_sequence(sequence: &Vec<i64>) -> Vec<i64> {
    sequence
        .into_iter()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .collect()
}

fn analyse_sequences(
    sequences: &Vec<Vec<i64>>,
    extrapolator: fn(sequence: &Vec<i64>) -> i64,
) -> i64 {
    sequences.iter().map(extrapolator).sum()
}

#[cfg(test)]
mod tests {
    use crate::day_9::*;

    fn example_sequences() -> Vec<Vec<i64>> {
        vec![
            vec![0, 3, 6, 9, 12, 15],
            vec![1, 3, 6, 10, 15, 21],
            vec![10, 13, 16, 21, 30, 45],
        ]
    }

    #[test]
    fn can_parse_input() {
        let input = "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"
            .to_string();

        assert_eq!(parse_input(&input), example_sequences());
    }

    #[test]
    fn can_extrapolate_sequences_forwards() {
        let results: Vec<i64> = example_sequences()
            .iter()
            .map(extrapolate_sequence_forwards)
            .collect();

        assert_eq!(results, vec![18, 28, 68])
    }

    #[test]
    fn can_extrapolate_sequences_backwards() {
        let results: Vec<i64> = example_sequences()
            .iter()
            .map(extrapolate_sequence_backwards)
            .collect();

        assert_eq!(results, vec![-3, 0, 5])
    }
}
