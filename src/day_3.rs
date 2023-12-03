//! This is my solution for [Advent of Code - Day 3: _Gear Ratios_](https://adventofcode.com/2023/day/3)
//!
//! [`parse_grid`] turns the plain text 2D grid of characters, in to a list of [`PartNumber`]s and a [`SymbolLookup`].
//!
//! [`sum_valid_part_numbers`] solves part one, delegating to [`has_adjacent_symbol`] and [`get_adjacent_points`] to
//! determine if each number is valid.
//!
//! [`sum_gear_ratios`] solves part two. The key logic is in [`find_gears`] which uses
//! * [`explode_adjacent_points`] (which uses [`get_adjacent_points`] again) to list all adjacent points
//! * [`is_point_a_gear_symbol`] to filter that list to `*` symbols that might be gears
//! * Then turns those that are valid into the expected list of [`Gear`]s

use std::collections::HashMap;
use std::fs;

/// Represents a part number as the position of the first digit, and the number it represents
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

/// A point in 2D space
type Point = (usize, usize);

/// A map of 2D points to the part symbol at that point
type SymbolLookup = HashMap<Point, char>;

/// A representation of a gear by the two part numbers that make up its "gear ratio"
///
/// This type exists to implement PartialEq regardless of number ordering
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

    println!(
        "The sum of gear ratios is {}",
        sum_gear_ratios(&part_numbers, &symbol_lookup)
    );
}

/// Parse a string representing a 2D grid into a list of part numbers and a lookup table of points with character
/// symbols
fn parse_grid(input: &String) -> (Vec<PartNumber>, SymbolLookup) {
    let mut parts = Vec::new();
    let mut symbols = HashMap::new();
    let mut num: u32 = 0;
    let mut num_origin: Option<Point> = None;

    for (y, line) in input.lines().enumerate() {
        for (x, chr) in line.chars().enumerate() {
            // We only know we've completed a part number when we next see a non-digit character. Check for that here
            // and emit the `PartNumber`.
            //
            // The if clauses need to be separate as Rust doesn't support mixed `if` and `if let` conditions yet.
            if !chr.is_digit(10) {
                if let Some((x, y)) = num_origin {
                    parts.push(PartNumber::new(num, x, y));

                    num = 0;
                    num_origin = None;
                }
            }

            match chr {
                '.' => {}
                // For PartNumbers build the number digit by digit, recording the origin on the first digit seen
                c if c.is_digit(10) => {
                    num_origin = num_origin.or(Some((x, y)));
                    num = num * 10 + chr.to_digit(10).expect("Tested with is_digit");
                }
                _ => {
                    symbols.insert((x, y), chr);
                }
            }
        }

        // Line breaks also split part numbers, so complete the current PartNumber if there is one
        if let Some((x, y)) = num_origin {
            parts.push(PartNumber::new(num, x, y));

            num = 0;
            num_origin = None;
        }
    }

    (parts, symbols)
}

/// Solves part 1 - the sum of part numbers next to a symbol
fn sum_valid_part_numbers(part_numbers: &Vec<PartNumber>, symbol_lookup: &SymbolLookup) -> u32 {
    part_numbers
        .iter()
        .filter(|&part_number| has_adjacent_symbol(part_number, symbol_lookup))
        .map(|part_number| part_number.number)
        .sum()
}

/// Part numbers are valid if adjacent to a symbol
fn has_adjacent_symbol(part_number: &PartNumber, symbol_lookup: &SymbolLookup) -> bool {
    return get_adjacent_points(part_number)
        .iter()
        .any(|point| symbol_lookup.contains_key(point));
}

/// Return the list of points adjacent to the whole part number that have non-negative co-ordinates
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

/// Return a list of valid gears. A gear is any `*` symbol with exactly two adjacent PartNumbers.
fn find_gears(part_numbers: &Vec<PartNumber>, symbol_lookup: &SymbolLookup) -> Vec<Gear> {
    // Since PartNumbers can have variable length it is easier to start with all the points adjacent to part numbers
    // and then filter to part number / `*` point pairs` ...
    let part_nums_adjacent_to_gear_points = part_numbers
        .iter()
        .flat_map(explode_adjacent_points)
        .filter(|(_, point)| is_point_a_gear_symbol(point, symbol_lookup));

    // ... Then invert the relationship by grouping the numbers by the `*` they are adjacent to
    let mut part_numbers_per_gear_point: HashMap<Point, Vec<u32>> = HashMap::new();
    for (part_number, point) in part_nums_adjacent_to_gear_points {
        part_numbers_per_gear_point
            .entry(point)
            .or_insert(Vec::new())
            .push(part_number)
    }

    // Any that have the required two numbers are the `Gear`s to return
    part_numbers_per_gear_point
        .values()
        .filter(|parts| parts.len() == 2)
        .map(|parts| Gear::new(parts[0], parts[1]))
        .collect()
}

/// Turn a PartNumber into a list of pairs of the bare number and each point it is adjacent to
fn explode_adjacent_points(part_number: &PartNumber) -> Vec<(u32, Point)> {
    get_adjacent_points(part_number)
        .into_iter()
        .map(|point| (part_number.number, point))
        .collect::<Vec<(u32, Point)>>()
}

/// Returns true if a 2D co-ordinate maps to a `*` symbol
fn is_point_a_gear_symbol(point: &Point, symbol_lookup: &SymbolLookup) -> bool {
    symbol_lookup
        .get(point)
        .filter(|&symbol| *symbol == '*')
        .is_some()
}

/// Solution to part 2 - finds all the valid gears and sums the multiplications of their "gear ratio" numbers.
fn sum_gear_ratios(part_numbers: &Vec<PartNumber>, symbol_lookup: &SymbolLookup) -> u32 {
    find_gears(part_numbers, symbol_lookup)
        .iter()
        .map(|Gear { part_1, part_2 }| part_1 * part_2)
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day_3::*;
    use crate::helpers::test::assert_contains_in_any_order;

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

        assert_contains_in_any_order(parts, expected_parts);

        assert_contains_in_any_order(symbols, expected_symbol_lookup);
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
            assert_contains_in_any_order(get_adjacent_points(&part_number), expected_points);
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

        assert_contains_in_any_order(
            find_gears(&example_part_numbers(), &example_symbol_lookup()),
            expected_gears,
        )
    }

    #[test]
    fn can_find_gears_with_shared_part_number() {
        let example_grid = "\
1...2
.*.*.
..3.."
            .to_string();

        let (part_numbers, symbol_lookup) = parse_grid(&example_grid);

        let expected_gears = vec![Gear::new(1, 3), Gear::new(2, 3)];

        assert_contains_in_any_order(find_gears(&part_numbers, &symbol_lookup), expected_gears)
    }

    #[test]
    fn can_sum_gear_ratios() {
        assert_eq!(
            sum_gear_ratios(&example_part_numbers(), &example_symbol_lookup()),
            467835
        );
    }
}
