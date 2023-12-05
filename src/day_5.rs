//! This is my solution for [Advent of Code - Day 5: _???_](https://adventofcode.com/2023/day/5)
//!
//!

use crate::day_5::Category::*;
use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs;
use std::str::FromStr;

#[derive(Eq, PartialEq, Debug)]
struct Range {
    start: i64,
    delta: i64,
    length: i64,
}

impl Range {
    fn new(start: i64, delta: i64, length: i64) -> Range {
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
            "seed" => Ok(Seed),
            "soil" => Ok(Soil),
            "fertilizer" => Ok(Fertilizer),
            "water" => Ok(Water),
            "light" => Ok(Light),
            "temperature" => Ok(Temperature),
            "humidity" => Ok(Humidity),
            "location" => Ok(Location),
            _ => Err(()),
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct Id {
    category: Category,
    value: i64,
}

impl Id {
    fn new(category: Category, value: i64) -> Id {
        Id { category, value }
    }
}

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-5-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 5.
pub fn run() {
    let contents = fs::read_to_string("res/day-5-input.txt").expect("Failed to read file");

    let (seeds, almanac) = parse_input(&contents);

    println!(
        "The nearest location_id is: {}",
        find_nearest_location(&seeds, &almanac)
    )
}

fn find_nearest_location(seeds: &Vec<Id>, almanac: &HashMap<Category, AlmanacMap>) -> i64 {
    seeds
        .iter()
        .map(|seed| progress_id_to(seed.clone(), Location, almanac).value)
        .min()
        .unwrap_or(0)
}

fn parse_input(input: &String) -> (Vec<Id>, HashMap<Category, AlmanacMap>) {
    let mut parts = input.split("\n\n");

    (parse_seeds(parts.next().unwrap()), parse_maps(parts))
}

fn parse_seeds(input: &str) -> Vec<Id> {
    input
        .split(" ")
        .filter_map(|id| id.parse::<i64>().ok())
        .map(|value| Id::new(Seed, value))
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
    let parts: Vec<i64> = range_spec
        .split(" ")
        .filter_map(|part| part.parse().ok())
        .collect();
    if parts.len() < 3 {
        println!("error parsing {}, got {:?}", range_spec, parts)
    }

    Range::new(parts[1], parts[0] - parts[1], parts[2])
}

fn progress_id(id: &Id, almanac: &HashMap<Category, AlmanacMap>) -> Id {
    let mapper = almanac.get(&id.category).unwrap();
    for range in &mapper.ranges {
        if id.value < range.start {
            return Id::new(mapper.destination, id.value);
        }
        if id.value < range.start + range.length {
            return Id::new(mapper.destination, id.value + range.delta);
        }
    }

    Id::new(mapper.destination, id.value)
}

fn progress_id_to(id: Id, category: Category, almanac: &HashMap<Category, AlmanacMap>) -> Id {
    return if id.category == category {
        id
    } else {
        progress_id_to(progress_id(&id, almanac), category, almanac)
    };
}

#[cfg(test)]
mod tests {
    use crate::day_5::*;
    use crate::helpers::test::assert_contains_in_any_order;
    use std::collections::HashMap;

    fn example_seeds() -> Vec<Id> {
        vec![
            Id::new(Seed, 79),
            Id::new(Seed, 14),
            Id::new(Seed, 55),
            Id::new(Seed, 13),
        ]
    }

    fn example_almanac() -> HashMap<Category, AlmanacMap> {
        vec![
            (
                Seed,
                AlmanacMap::new(
                    Seed,
                    Soil,
                    vec![Range::new(50, 2, 48), Range::new(98, -48, 2)],
                ),
            ),
            (
                Soil,
                AlmanacMap::new(
                    Soil,
                    Fertilizer,
                    vec![
                        Range::new(0, 39, 15),
                        Range::new(15, -15, 37),
                        Range::new(52, -15, 2),
                    ],
                ),
            ),
            (
                Fertilizer,
                AlmanacMap::new(
                    Fertilizer,
                    Water,
                    vec![
                        Range::new(0, 42, 7),
                        Range::new(7, 50, 4),
                        Range::new(11, -11, 42),
                        Range::new(53, -4, 8),
                    ],
                ),
            ),
            (
                Water,
                AlmanacMap::new(
                    Water,
                    Light,
                    vec![Range::new(18, 70, 7), Range::new(25, -7, 70)],
                ),
            ),
            (
                Light,
                AlmanacMap::new(
                    Light,
                    Temperature,
                    vec![
                        Range::new(45, 36, 19),
                        Range::new(64, 4, 13),
                        Range::new(77, -32, 23),
                    ],
                ),
            ),
            (
                Temperature,
                AlmanacMap::new(
                    Temperature,
                    Humidity,
                    vec![Range::new(0, 1, 69), Range::new(69, -69, 1)],
                ),
            ),
            (
                Humidity,
                AlmanacMap::new(
                    Humidity,
                    Location,
                    vec![Range::new(56, 4, 37), Range::new(93, -37, 4)],
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
        assert_contains_in_any_order(actual_maps, example_almanac());
    }

    #[test]
    fn can_progress_id() {
        let almanac = example_almanac();

        assert_eq!(progress_id(&Id::new(Seed, 79), &almanac), Id::new(Soil, 81));
        assert_eq!(progress_id(&Id::new(Seed, 14), &almanac), Id::new(Soil, 14));
        assert_eq!(progress_id(&Id::new(Seed, 55), &almanac), Id::new(Soil, 57));
        assert_eq!(progress_id(&Id::new(Seed, 13), &almanac), Id::new(Soil, 13));
    }

    #[test]
    fn can_progress_id_to_location() {
        let almanac = example_almanac();

        assert_eq!(
            progress_id_to(Id::new(Seed, 79), Location, &almanac),
            Id::new(Location, 82)
        );
        assert_eq!(
            progress_id_to(Id::new(Seed, 14), Location, &almanac),
            Id::new(Location, 43)
        );
        assert_eq!(
            progress_id_to(Id::new(Seed, 55), Location, &almanac),
            Id::new(Location, 86)
        );
        assert_eq!(
            progress_id_to(Id::new(Seed, 13), Location, &almanac),
            Id::new(Location, 35)
        );
    }

    #[test]
    fn can_find_nearest_location() {
        assert_eq!(
            find_nearest_location(&example_seeds(), &example_almanac()),
            35
        );
    }
}
