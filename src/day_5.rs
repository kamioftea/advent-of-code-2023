//! This is my solution for [Advent of Code - Day 5: _If You Give A Seed A Fertilizer_](https://adventofcode.com/2023/day/5)
//!
//! Given input representing a list of [`Seed`] ids and an [`Almanac`] of mappings between ids in pairs of
//! [`Category`]s. Transform the [`Seed`] ids to [`Location`] ids and find the lowest [`Location`] id to solve the
//! puzzle.
//!
//! The parsing starts with [`parse_input`], which delegates to [`parse_seeds`], [`parse_almanac`], [`parse_header`],
//! and [`parse_range`].
//!
//! The seed ids are interpreted for part one using [`ids_as_single_seeds`], and part two with [`ids_to_ranges`]. These
//! seed ranges are turned into the minimum location by [`find_nearest_location`], using
//! [`progress_id_ranges_to_category`] and [`progress_id_range`].

use crate::day_5::Category::*;
use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs;
use std::str::FromStr;

/// A range of ids to modify when applying the mapping for an AlmanacSection
#[derive(Eq, PartialEq, Debug)]
struct AlmanacRange {
    start: i64,
    length: i64,
    delta: i64,
}

impl AlmanacRange {
    fn new(start: i64, length: i64, delta: i64) -> AlmanacRange {
        AlmanacRange {
            start,
            length,
            delta,
        }
    }
}

/// A mapping from one category of ids to another. Ranges are stored sorted by starting source id
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

/// A collection of category mappings grouped by source category
type Almanac = HashMap<Category, AlmanacSection>;

/// The possible categories of id
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

/// A range of ids in a category that should be planted
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
        find_nearest_location(ids_as_single_seeds(&seeds), &almanac)
    );

    println!(
        "The nearest location id from seed ranges is: {}",
        find_nearest_location(ids_to_ranges(&seeds), &almanac)
    )
}

/// Split and parse the puzzle input into the list of seeds and the almanac specification.
/// The seeds and each section are delimited by blank lines.
fn parse_input(input: &String) -> (Vec<i64>, Almanac) {
    let mut parts = input.split("\n\n");

    (parse_seeds(parts.next().unwrap()), parse_almanac(parts))
}

/// Parse the list of seeds to numeric ids
fn parse_seeds(input: &str) -> Vec<i64> {
    input
        .split(" ")
        .filter_map(|id| id.parse::<i64>().ok())
        .collect()
}

/// Each almanac section is a single header line, then one line per id mapping
fn parse_almanac<'a>(section_specs: impl Iterator<Item = &'a str>) -> Almanac {
    let mut almanac = HashMap::new();

    for section_spec in section_specs {
        let mut lines = section_spec.lines();
        let (from, to) = parse_header(lines.next().unwrap());
        almanac.insert(
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

    almanac
}

/// Parse a header in the format `seed-to-soil map:` into source and destination categories
fn parse_header(header_spec: &str) -> (Category, Category) {
    let mapping_part = header_spec.replace(" map:", "");
    let (from, to) = mapping_part.split_once("-to-").unwrap();
    return (from.parse().unwrap(), to.parse().unwrap());
}

/// Parse a range of id mappings, three space-separated numbers in the order `destination_start` `source_start` `length`
fn parse_range(range_spec: &str) -> AlmanacRange {
    let parts: Vec<i64> = range_spec
        .split(" ")
        .filter_map(|part| part.parse().ok())
        .collect();

    AlmanacRange::new(parts[1], parts[2], parts[0] - parts[1])
}

/// For part one each seed is a single id, which can be represented as a range of length 1
fn ids_as_single_seeds(ids: &Vec<i64>) -> Vec<IdRange> {
    ids.into_iter()
        .map(|&start| IdRange::new(Seed, start, 1))
        .collect()
}

/// For part two each pair of numbers represents a range, in the format `start length`
fn ids_to_ranges(ids: &Vec<i64>) -> Vec<IdRange> {
    ids.into_iter()
        .tuples()
        .map(|(&start, &length)| IdRange::new(Seed, start, length))
        .collect()
}

/// Apply all almanac mappings, return the start of the lowest resulting range
fn find_nearest_location(seeds: Vec<IdRange>, almanac: &Almanac) -> i64 {
    progress_id_ranges_to_category(seeds, Location, almanac)
        .iter()
        .map(|range| range.start)
        .min()
        .unwrap()
}

/// Recursively advance a list of category ids until a specific category is reached
fn progress_id_ranges_to_category(
    id_ranges: Vec<IdRange>,
    category: Category,
    almanac: &Almanac,
) -> Vec<IdRange> {
    let current_category = id_ranges.get(0).unwrap().category;

    if current_category == category {
        id_ranges
    } else {
        progress_id_ranges_to_category(
            id_ranges
                .iter()
                .flat_map(|range| progress_id_range(range, almanac))
                .collect(),
            category,
            almanac,
        )
    }
}

/// Take a single range of ids to plant in one category and apply the relevant mapping from the almanac.
/// - Where different mappings apply to different parts of the range, return a separate continuous range for each
/// - Where a mapping is not defined, the id doesn't change, but the category still advances
fn progress_id_range(id_range: &IdRange, almanac: &Almanac) -> Vec<IdRange> {
    let mut new_id_ranges = Vec::new();
    let mut current = id_range.start;
    let id_range_end = id_range.start + id_range.length;

    let section = almanac.get(&id_range.category).unwrap();

    // ranges are already sorted, so walk through them adding mapped ranges where they overlap
    for almanac_range in &section.ranges {
        // if there is a gap before the next id starts add an output range with unchanged ids
        if almanac_range.start > current {
            let id_sub_range_end = almanac_range.start.min(id_range_end);
            new_id_ranges.push(IdRange::new(
                section.destination.clone(),
                current,
                id_sub_range_end - current,
            ));

            current = almanac_range.start;
        }

        // Short circuit if we've reached the end of the input range
        if current >= id_range_end {
            break;
        }

        // if there is an overlap, add the overlap to the output
        if almanac_range.start + almanac_range.length > current {
            let id_range_end = (almanac_range.start + almanac_range.length).min(id_range_end);
            new_id_ranges.push(IdRange::new(
                section.destination.clone(),
                current + almanac_range.delta,
                id_range_end - current,
            ));

            current = almanac_range.start + almanac_range.length;
        }

        // Short circuit if we've reached the end of the input range
        if current >= id_range_end {
            break;
        }
    }

    // If the current range extends beyond the mappings in the almanac, add the remaining ids unmapped to the output
    if current < id_range_end {
        new_id_ranges.push(IdRange::new(
            section.destination,
            current,
            id_range_end - current,
        ))
    }

    new_id_ranges
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
                    vec![AlmanacRange::new(50, 48, 2), AlmanacRange::new(98, 2, -48)],
                ),
            ),
            (
                Soil,
                AlmanacSection::new(
                    Soil,
                    Fertilizer,
                    vec![
                        AlmanacRange::new(0, 15, 39),
                        AlmanacRange::new(15, 37, -15),
                        AlmanacRange::new(52, 2, -15),
                    ],
                ),
            ),
            (
                Fertilizer,
                AlmanacSection::new(
                    Fertilizer,
                    Water,
                    vec![
                        AlmanacRange::new(0, 7, 42),
                        AlmanacRange::new(7, 4, 50),
                        AlmanacRange::new(11, 42, -11),
                        AlmanacRange::new(53, 8, -4),
                    ],
                ),
            ),
            (
                Water,
                AlmanacSection::new(
                    Water,
                    Light,
                    vec![AlmanacRange::new(18, 7, 70), AlmanacRange::new(25, 70, -7)],
                ),
            ),
            (
                Light,
                AlmanacSection::new(
                    Light,
                    Temperature,
                    vec![
                        AlmanacRange::new(45, 19, 36),
                        AlmanacRange::new(64, 13, 4),
                        AlmanacRange::new(77, 23, -32),
                    ],
                ),
            ),
            (
                Temperature,
                AlmanacSection::new(
                    Temperature,
                    Humidity,
                    vec![AlmanacRange::new(0, 69, 1), AlmanacRange::new(69, 1, -69)],
                ),
            ),
            (
                Humidity,
                AlmanacSection::new(
                    Humidity,
                    Location,
                    vec![AlmanacRange::new(56, 37, 4), AlmanacRange::new(93, 4, -37)],
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
            find_nearest_location(ids_as_single_seeds(&example_seeds()), &example_almanac()),
            35
        );

        assert_eq!(
            find_nearest_location(ids_to_ranges(&example_seeds()), &example_almanac()),
            46
        );
    }
}
