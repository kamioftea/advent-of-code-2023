//! This is my solution for [Advent of Code - Day 3: _Gear Ratios_](https://adventofcode.com/2023/day/3)
//!
//!

use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

#[derive(Eq, PartialEq, Debug)]
struct PartNumber {
    number: u32,
    x: usize,
    y: usize,
}

impl PartNumber {
    fn new(number: u32, x: usize, y: usize) -> PartNumber {
        PartNumber { number, x, y }
    }
}

type Point = (usize, usize);

type SymbolLookup = HashMap<Point, char>;

#[derive(Eq, Debug)]
struct Gear {
    part_1: u32,
    part_2: u32,
}

impl Gear {
    fn new(part_1: u32, part_2: u32) -> Gear {
        Gear { part_1, part_2 }
    }
}

impl PartialEq for Gear {
    fn eq(&self, other: &Self) -> bool {
        (self.part_1 == other.part_1 && self.part_2 == other.part_2)
            || (self.part_1 == other.part_2 && self.part_2 == other.part_1)
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-3-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 3.
pub fn run() {
    let contents = fs::read_to_string("res/day-3-input.txt").expect("Failed to read file");

    let (part_numbers, symbol_lookup) = parse_grid(&contents);

    println!(
        "The sum of valid part numbers is {}",
        sum_valid_part_numbers(&part_numbers, &symbol_lookup)
    );
}

fn parse_grid(input: &String) -> (Vec<PartNumber>, SymbolLookup) {
    let mut parts = Vec::new();
    let mut symbols = HashMap::new();
    let mut num: u32 = 0;
    let mut num_origin: Option<Point> = None;

    for (y, line) in input.lines().enumerate() {
        for (x, chr) in line.chars().enumerate() {
            if !chr.is_digit(10) {
                if let Some((x, y)) = num_origin {
                    parts.push(PartNumber::new(num, x, y))
                }
                num = 0;
                num_origin = None;
            }

            match chr {
                '.' => {}
                c if c.is_digit(10) => {
                    num_origin = num_origin.or(Some((x, y)));
                    num = num * 10 + chr.to_digit(10).expect("Tested with is_digit");
                }
                _ => {
                    symbols.insert((x, y), chr);
                }
            }
        }
    }

    if let Some((x, y)) = num_origin {
        parts.push(PartNumber::new(num, x, y))
    }

    (parts, symbols)
}

fn sum_valid_part_numbers(part_numbers: &Vec<PartNumber>, symbol_lookup: &SymbolLookup) -> u32 {
    part_numbers
        .iter()
        .filter(|&part_number| has_adjacent_symbol(part_number, symbol_lookup))
        .map(|part_number| part_number.number)
        .sum()
}

fn has_adjacent_symbol(part_number: &PartNumber, symbol_lookup: &SymbolLookup) -> bool {
    return get_adjacent_points(part_number)
        .iter()
        .any(|point| symbol_lookup.contains_key(point));
}

fn get_adjacent_points(part_number: &PartNumber) -> Vec<Point> {
    let mut points = Vec::new();
    let length = part_number.number.ilog10() as usize + 1;
    let start = part_number.x.checked_sub(1).unwrap_or(0);
    let end = part_number.x + length;

    for x in start..=end {
        if part_number.y > 0 {
            points.push((x, part_number.y - 1))
        }

        if x < part_number.x || x >= end {
            points.push((x, part_number.y))
        }

        points.push((x, part_number.y + 1))
    }

    points
}

fn find_gears(part_numbers: &Vec<PartNumber>, symbol_lookup: &SymbolLookup) -> Vec<Gear> {
    part_numbers
        .iter()
        .flat_map(|part_number| {
            get_adjacent_points(part_number)
                .into_iter()
                .map(|point| (part_number.number, point))
                .collect::<Vec<(u32, Point)>>()
        })
        .filter(|(_, point)| {
            symbol_lookup
                .get(point)
                .filter(|&symbol| *symbol == '*')
                .is_some()
        })
        .group_by(|(_, point)| point.clone())
        .into_iter()
        .map(|(_, group)| group.map(|(part, _)| part).collect::<Vec<u32>>())
        .filter(|parts| parts.len() == 2)
        .map(|parts| Gear::new(parts[0], parts[1]))
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::day_3::*;

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

    fn example_part_numbers() -> Vec<PartNumber> {
        vec![
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
        ]
    }

    fn example_symbol_lookup() -> SymbolLookup {
        vec![
            ((3, 1), '*'),
            ((6, 3), '#'),
            ((3, 4), '*'),
            ((5, 5), '+'),
            ((3, 8), '$'),
            ((5, 8), '*'),
        ]
        .into_iter()
        .collect()
    }

    #[test]
    fn can_parse_grid() {
        let expected_parts = example_part_numbers();

        let expected_symbol_lookup = example_symbol_lookup();

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
                "Expected part {:?} is not in the list of parts",
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

    #[test]
    fn can_find_adjacent_points() {
        #[rustfmt::skip] // Positional coordinates
        let examples = vec![
            (PartNumber::new(99, 0, 0), vec![
                                (2, 0),
                (0, 1), (1, 1), (2, 1)
            ]),
            (PartNumber::new(1, 1, 1), vec![
                (0, 0), (1, 0), (2, 0),
                (0, 1)        , (2, 1),
                (0, 2), (1, 2), (2, 2),
            ]),
        ];

        for (part_number, expected_points) in examples {
            let actual_points = get_adjacent_points(&part_number);
            assert_eq!(
                actual_points.len(),
                expected_points.len(),
                "Points lists were not the same length.\nExpected: {:?}\nActual  : {:?}",
                expected_points,
                actual_points
            );
            for expected_point in expected_points {
                assert!(
                    actual_points.contains(&expected_point),
                    "{:?} is not in the list of points",
                    expected_point
                )
            }
        }
    }

    #[test]
    fn can_determine_if_part_is_adjacent_to_a_symbol() {
        let symbol_lookup = example_symbol_lookup();

        let examples = vec![
            (PartNumber::new(467, 0, 0), true),
            (PartNumber::new(114, 5, 0), false),
            (PartNumber::new(35, 2, 2), true),
            (PartNumber::new(633, 6, 2), true),
            (PartNumber::new(617, 0, 4), true),
            (PartNumber::new(58, 7, 5), false),
            (PartNumber::new(592, 2, 6), true),
            (PartNumber::new(755, 6, 7), true),
            (PartNumber::new(664, 1, 9), true),
            (PartNumber::new(598, 5, 9), true),
        ];

        for (part_number, expected) in examples {
            assert_eq!(
                has_adjacent_symbol(&part_number, &symbol_lookup),
                expected,
                "{:?} should{} have an adjacent symbol",
                part_number,
                if expected { "" } else { " not" }
            )
        }
    }

    #[test]
    fn can_sum_valid_part_numbers() {
        assert_eq!(
            sum_valid_part_numbers(&example_part_numbers(), &example_symbol_lookup(),),
            4361
        )
    }

    #[test]
    fn can_find_gears() {
        let expected_gears = vec![Gear::new(467, 35), Gear::new(755, 598)];

        assert_eq!(
            find_gears(&example_part_numbers(), &example_symbol_lookup()),
            expected_gears
        )
    }
}
