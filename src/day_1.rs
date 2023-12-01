//! This is my solution for [Advent of Code - Day 1 - _???_](https://adventofcode.com/2023/day/1)
//!
//!

use std::fs;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-1-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 1.
pub fn run() {
    let _contents = fs::read_to_string("res/day-1-input").expect("Failed to read file");
}

#[cfg(test)]
mod tests {

}