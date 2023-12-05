//! This is my solution for [Advent of Code - Day 5: _???_](https://adventofcode.com/2023/day/5)
//!
//!

use itertools::Itertools;
use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

#[derive(Eq, PartialEq, Debug)]
struct Range {
    start: i32,
    delta: i32,
    length: i32,
}

impl Range {
    fn new(start: i32, delta: i32, length: i32) -> Range {
        Range {
            start,
            delta,
            length,
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
struct AlmanacMap {
    source: Category,
    destination: Category,
    ranges: Vec<Range>,
}

impl AlmanacMap {
    fn new(source: Category, destination: Category, ranges: Vec<Range>) -> AlmanacMap {
        AlmanacMap {
            source,
            destination,
            ranges,
        }
    }
}

#[derive(Eq, PartialEq, Debug, Hash, Copy, Clone)]
enum Category {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

impl FromStr for Category {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "seed" => Ok(Category::Seed),
            "soil" => Ok(Category::Soil),
            "fertilizer" => Ok(Category::Fertilizer),
            "water" => Ok(Category::Water),
            "light" => Ok(Category::Light),
            "temperature" => Ok(Category::Temperature),
            "humidity" => Ok(Category::Humidity),
            "location" => Ok(Category::Location),
            _ => Err(()),
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
struct Id {
    category: Category,
    value: i32,
}

impl Id {
    fn new(category: Category, value: i32) -> Id {
        Id { category, value }
    }
}

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-5-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 5.
pub fn run() {
    let _contents = fs::read_to_string("res/day-5-input.txt").expect("Failed to read file");
}

fn parse_input(input: &String) -> (Vec<Id>, HashMap<Category, AlmanacMap>) {
    let mut parts = input.split("\n\n");

    (parse_seeds(parts.next().unwrap()), parse_maps(parts))
}

fn parse_seeds(input: &str) -> Vec<Id> {
    input
        .split(" ")
        .filter_map(|id| id.parse::<i32>().ok())
        .map(|value| Id::new(Category::Seed, value))
        .collect()
}

fn parse_maps<'a>(map_specs: impl Iterator<Item = &'a str>) -> HashMap<Category, AlmanacMap> {
    let mut maps = HashMap::new();

    for map_spec in map_specs {
        let mut lines = map_spec.lines();
        let (from, to) = parse_header(lines.next().unwrap());
        maps.insert(
            from.clone(),
            AlmanacMap::new(
                from,
                to,
                lines
                    .map(parse_range)
                    .sorted_by(|a, b| a.start.cmp(&b.start))
                    .collect(),
            ),
        );
    }

    maps
}

fn parse_header(header_spec: &str) -> (Category, Category) {
    let mapping_part = header_spec.replace(" map:", "");
    let (from, to) = mapping_part.split_once("-to-").unwrap();
    return (from.parse().unwrap(), to.parse().unwrap());
}

fn parse_range(range_spec: &str) -> Range {
    let parts: Vec<i32> = range_spec
        .split(" ")
        .filter_map(|part| part.parse().ok())
        .collect();
    Range::new(parts[0], parts[1] - parts[0], parts[2])
}

#[cfg(test)]
mod tests {
    use crate::day_5::*;
    use crate::helpers::test::assert_contains_in_any_order;
    use std::collections::HashMap;

    fn example_seeds() -> Vec<Id> {
        vec![
            Id::new(Category::Seed, 79),
            Id::new(Category::Seed, 14),
            Id::new(Category::Seed, 55),
            Id::new(Category::Seed, 13),
        ]
    }

    fn example_maps() -> HashMap<Category, AlmanacMap> {
        vec![
            (
                Category::Seed,
                AlmanacMap::new(
                    Category::Seed,
                    Category::Soil,
                    vec![Range::new(50, 48, 2), Range::new(52, -2, 48)],
                ),
            ),
            (
                Category::Soil,
                AlmanacMap::new(
                    Category::Soil,
                    Category::Fertilizer,
                    vec![
                        Range::new(0, 15, 37),
                        Range::new(37, 15, 2),
                        Range::new(39, -39, 15),
                    ],
                ),
            ),
            (
                Category::Fertilizer,
                AlmanacMap::new(
                    Category::Fertilizer,
                    Category::Water,
                    vec![
                        Range::new(0, 11, 42),
                        Range::new(42, -42, 7),
                        Range::new(49, 4, 8),
                        Range::new(57, -50, 4),
                    ],
                ),
            ),
            (
                Category::Water,
                AlmanacMap::new(
                    Category::Water,
                    Category::Light,
                    vec![Range::new(18, 7, 70), Range::new(88, -70, 7)],
                ),
            ),
            (
                Category::Light,
                AlmanacMap::new(
                    Category::Light,
                    Category::Temperature,
                    vec![
                        Range::new(45, 32, 23),
                        Range::new(68, -4, 13),
                        Range::new(81, -36, 19),
                    ],
                ),
            ),
            (
                Category::Temperature,
                AlmanacMap::new(
                    Category::Temperature,
                    Category::Humidity,
                    vec![Range::new(0, 69, 1), Range::new(1, -1, 69)],
                ),
            ),
            (
                Category::Humidity,
                AlmanacMap::new(
                    Category::Humidity,
                    Category::Location,
                    vec![Range::new(56, 37, 4), Range::new(60, -4, 37)],
                ),
            ),
        ]
        .into_iter()
        .collect()
    }

    #[test]
    fn can_parse_input() {
        let input = "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"
            .to_string();

        let (actual_seeds, actual_maps) = parse_input(&input);

        assert_eq!(actual_seeds, example_seeds());
        assert_contains_in_any_order(actual_maps, example_maps());
    }
}
