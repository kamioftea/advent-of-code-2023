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
struct AlmanacRange {
    start: i64,
    delta: i64,
    length: i64,
}

impl AlmanacRange {
    fn new(start: i64, delta: i64, length: i64) -> AlmanacRange {
        AlmanacRange {
            start,
            delta,
            length,
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
struct AlmanacSection {
    source: Category,
    destination: Category,
    ranges: Vec<AlmanacRange>,
}

impl AlmanacSection {
    fn new(source: Category, destination: Category, ranges: Vec<AlmanacRange>) -> AlmanacSection {
        AlmanacSection {
            source,
            destination,
            ranges,
        }
    }
}

type Almanac = HashMap<Category, AlmanacSection>;

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
struct IdRange {
    category: Category,
    start: i64,
    length: i64,
}

impl IdRange {
    fn new(category: Category, start: i64, length: i64) -> IdRange {
        IdRange {
            category,
            start,
            length,
        }
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
        "The nearest location id from individual seeds is: {}",
        find_nearest_location_from_ranges(ids_as_single_seeds(&seeds), &almanac)
    );

    println!(
        "The nearest location id from seed ranges is: {}",
        find_nearest_location_from_ranges(ids_to_ranges(&seeds), &almanac)
    )
}

fn find_nearest_location_from_ranges(seeds: Vec<IdRange>, almanac: &Almanac) -> i64 {
    progress_id_ranges_to(seeds, Location, almanac)
        .iter()
        .map(|range| range.start)
        .min()
        .unwrap()
}

fn parse_input(input: &String) -> (Vec<i64>, Almanac) {
    let mut parts = input.split("\n\n");

    (parse_seeds(parts.next().unwrap()), parse_maps(parts))
}

fn parse_seeds(input: &str) -> Vec<i64> {
    input
        .split(" ")
        .filter_map(|id| id.parse::<i64>().ok())
        .collect()
}

fn parse_maps<'a>(map_specs: impl Iterator<Item = &'a str>) -> Almanac {
    let mut maps = HashMap::new();

    for map_spec in map_specs {
        let mut lines = map_spec.lines();
        let (from, to) = parse_header(lines.next().unwrap());
        maps.insert(
            from.clone(),
            AlmanacSection::new(
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

fn parse_range(range_spec: &str) -> AlmanacRange {
    let parts: Vec<i64> = range_spec
        .split(" ")
        .filter_map(|part| part.parse().ok())
        .collect();

    AlmanacRange::new(parts[1], parts[0] - parts[1], parts[2])
}

fn progress_id_range(id_range: &IdRange, almanac: &Almanac) -> Vec<IdRange> {
    let mut new_id_ranges = Vec::new();
    let mut current = id_range.start;
    let id_range_end = id_range.start + id_range.length;

    let section = almanac.get(&id_range.category).unwrap();

    for almanac_range in &section.ranges {
        if almanac_range.start > current {
            let id_sub_range_end = almanac_range.start.min(id_range_end);
            new_id_ranges.push(IdRange::new(
                section.destination.clone(),
                current,
                id_sub_range_end - current,
            ));

            current = almanac_range.start;
        }

        if current >= id_range_end {
            break;
        }

        if almanac_range.start + almanac_range.length > current {
            let id_range_end = (almanac_range.start + almanac_range.length).min(id_range_end);
            new_id_ranges.push(IdRange::new(
                section.destination.clone(),
                current + almanac_range.delta,
                id_range_end - current,
            ));

            current = almanac_range.start + almanac_range.length;
        }

        if current >= id_range_end {
            break;
        }
    }

    if current < id_range_end {
        new_id_ranges.push(IdRange::new(
            section.destination,
            current,
            id_range_end - current,
        ))
    }

    new_id_ranges
}

fn ids_as_single_seeds(ids: &Vec<i64>) -> Vec<IdRange> {
    ids.into_iter()
        .map(|&start| IdRange::new(Seed, start, 1))
        .collect()
}

fn ids_to_ranges(ids: &Vec<i64>) -> Vec<IdRange> {
    ids.into_iter()
        .tuples()
        .map(|(&start, &length)| IdRange::new(Seed, start, length))
        .collect()
}

fn progress_id_ranges_to(
    id_ranges: Vec<IdRange>,
    category: Category,
    almanac: &Almanac,
) -> Vec<IdRange> {
    let current_category = id_ranges.get(0).unwrap().category;

    if current_category == category {
        id_ranges
    } else {
        progress_id_ranges_to(
            id_ranges
                .iter()
                .flat_map(|range| progress_id_range(range, almanac))
                .collect(),
            category,
            almanac,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::day_5::*;
    use crate::helpers::test::assert_contains_in_any_order;

    fn example_seeds() -> Vec<i64> {
        vec![79, 14, 55, 13]
    }

    fn example_almanac() -> Almanac {
        vec![
            (
                Seed,
                AlmanacSection::new(
                    Seed,
                    Soil,
                    vec![AlmanacRange::new(50, 2, 48), AlmanacRange::new(98, -48, 2)],
                ),
            ),
            (
                Soil,
                AlmanacSection::new(
                    Soil,
                    Fertilizer,
                    vec![
                        AlmanacRange::new(0, 39, 15),
                        AlmanacRange::new(15, -15, 37),
                        AlmanacRange::new(52, -15, 2),
                    ],
                ),
            ),
            (
                Fertilizer,
                AlmanacSection::new(
                    Fertilizer,
                    Water,
                    vec![
                        AlmanacRange::new(0, 42, 7),
                        AlmanacRange::new(7, 50, 4),
                        AlmanacRange::new(11, -11, 42),
                        AlmanacRange::new(53, -4, 8),
                    ],
                ),
            ),
            (
                Water,
                AlmanacSection::new(
                    Water,
                    Light,
                    vec![AlmanacRange::new(18, 70, 7), AlmanacRange::new(25, -7, 70)],
                ),
            ),
            (
                Light,
                AlmanacSection::new(
                    Light,
                    Temperature,
                    vec![
                        AlmanacRange::new(45, 36, 19),
                        AlmanacRange::new(64, 4, 13),
                        AlmanacRange::new(77, -32, 23),
                    ],
                ),
            ),
            (
                Temperature,
                AlmanacSection::new(
                    Temperature,
                    Humidity,
                    vec![AlmanacRange::new(0, 1, 69), AlmanacRange::new(69, -69, 1)],
                ),
            ),
            (
                Humidity,
                AlmanacSection::new(
                    Humidity,
                    Location,
                    vec![AlmanacRange::new(56, 4, 37), AlmanacRange::new(93, -37, 4)],
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
    fn can_explode_seed_pairs() {
        let expected_ranges: Vec<IdRange> =
            vec![IdRange::new(Seed, 79, 14), IdRange::new(Seed, 79, 14)];

        assert_contains_in_any_order(ids_to_ranges(&example_seeds()), expected_ranges);
    }

    #[test]
    fn can_progress_id_ranges() {
        let almanac = example_almanac();
        assert_eq!(
            progress_id_range(&IdRange::new(Seed, 0, 100), &almanac),
            vec![
                IdRange::new(Soil, 0, 50),
                IdRange::new(Soil, 52, 48),
                IdRange::new(Soil, 50, 2)
            ]
        );
        assert_eq!(
            progress_id_range(&IdRange::new(Seed, 97, 2), &almanac),
            vec![IdRange::new(Soil, 99, 1), IdRange::new(Soil, 50, 1)]
        );
    }

    #[test]
    fn can_find_nearest_location_from_ranges() {
        assert_eq!(
            find_nearest_location_from_ranges(
                ids_as_single_seeds(&example_seeds()),
                &example_almanac()
            ),
            35
        );

        assert_eq!(
            find_nearest_location_from_ranges(ids_to_ranges(&example_seeds()), &example_almanac()),
            46
        );
    }
}
