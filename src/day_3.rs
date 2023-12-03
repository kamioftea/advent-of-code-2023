//! This is my solution for [Advent of Code - Day 3: _Gear Ratios_](https://adventofcode.com/2023/day/3)
//!
//!

use std::collections::HashMap;
use std::fs;

#[derive(Eq, PartialEq, Debug)]
struct PartNumber {
    number: u32,
    x: u32,
    y: u32,
}

impl PartNumber {
    fn new(number: u32, x: u32, y: u32) -> PartNumber {
        PartNumber { number, x, y }
    }
}

type Point = (u32, u32);

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-3-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 3.
pub fn run() {
    let _contents = fs::read_to_string("res/day-3-input.txt").expect("Failed to read file");
}

fn parse_grid(input: &String) -> (Vec<PartNumber>, HashMap<Point, char>) {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::day_3::*;
    use std::collections::HashMap;

    fn sample_input() -> String {
        return "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            .to_string();
    }

    #[test]
    fn can_parse_grid() {
        let expected_parts = vec![
            PartNumber::new(467, 0, 0),
            PartNumber::new(114, 5, 0),
            PartNumber::new(35, 2, 2),
            PartNumber::new(633, 6, 2),
            PartNumber::new(617, 0, 4),
            PartNumber::new(58, 7, 5),
            PartNumber::new(592, 2, 6),
            PartNumber::new(755, 6, 7),
            PartNumber::new(664, 1, 9),
            PartNumber::new(598, 5, 9),
        ];

        let expected_symbol_lookup: HashMap<Point, char> = vec![
            ((3, 1), '*'),
            ((6, 3), '#'),
            ((3, 4), '*'),
            ((5, 5), '+'),
            ((3, 8), '$'),
            ((5, 8), '$'),
        ]
        .into_iter()
        .collect();

        let (parts, symbols) = parse_grid(&sample_input());

        assert_eq!(
            parts.len(),
            expected_parts.len(),
            "The length of parsed parts({}) does not match the expected length({})",
            parts.len(),
            expected_parts.len()
        );

        for expected_part in expected_parts {
            assert!(
                parts.contains(&expected_part),
                "Expected part {:?} was not contained in the list of parts",
                expected_part
            );
        }

        assert_eq!(
            symbols.len(),
            expected_symbol_lookup.len(),
            "The length of the symbol lookup({}) does not match the expected length({})",
            symbols.len(),
            expected_symbol_lookup.len()
        );

        for (expected_point, expected_symbol) in &expected_symbol_lookup {
            assert_eq!(
                symbols.get(expected_point),
                Some(expected_symbol),
                "Expected symbol {:?} at {:?}, found: {:?}",
                expected_symbol,
                expected_point,
                symbols.get(expected_point)
            );
        }
    }
}
