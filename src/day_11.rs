//! This is my solution for [Advent of Code - Day 11: _Cosmic Expansion_](https://adventofcode.com/2023/day/11)
//!
//!

use itertools::Itertools;
use std::collections::HashSet;
use std::fs;

type Coordinate = (usize, usize);

#[derive(Eq, PartialEq, Debug)]
struct GalaxyImage {
    galaxies: Vec<Coordinate>,
    clear_x: HashSet<usize>,
    clear_y: HashSet<usize>,
}

impl GalaxyImage {
    fn distance(a: usize, b: usize, clear: &HashSet<usize>) -> usize {
        if a > b {
            GalaxyImage::distance(b, a, clear)
        } else {
            b - a + clear.intersection(&(a..b).into_iter().collect()).count()
        }
    }

    fn min_distance(&self, &(x_a, y_a): &Coordinate, &(x_b, y_b): &Coordinate) -> usize {
        GalaxyImage::distance(x_a, x_b, &self.clear_x)
            + GalaxyImage::distance(y_a, y_b, &self.clear_y)
    }
}

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-11-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 11.
pub fn run() {
    let contents = fs::read_to_string("res/day-11-input.txt").expect("Failed to read file");

    let image = parse_input(&contents);

    println!(
        "The sum of distances after expansion is: {}",
        sum_of_all_pair_distances(&image)
    )
}

fn parse_input(input: &String) -> GalaxyImage {
    let mut galaxies: Vec<Coordinate> = Vec::new();
    let mut set_x: HashSet<usize> = HashSet::new();
    let mut set_y: HashSet<usize> = HashSet::new();
    let mut max_x: usize = 0;
    let mut max_y: usize = 0;

    for (y, line) in input.lines().enumerate() {
        for (x, chr) in line.chars().enumerate() {
            if chr == '#' {
                galaxies.push((x, y));
                set_x.insert(x);
                set_y.insert(y);
                max_x = max_x.max(x);
                max_y = max_y.max(y);
            }
        }
    }

    let clear_x = (0..=max_x)
        .into_iter()
        .filter(|x| !set_x.contains(x))
        .collect();
    let clear_y = (0..=max_y)
        .into_iter()
        .filter(|y| !set_y.contains(y))
        .collect();

    GalaxyImage {
        galaxies,
        clear_x,
        clear_y,
    }
}

fn sum_of_all_pair_distances(image: &GalaxyImage) -> usize {
    image
        .galaxies
        .iter()
        .tuple_combinations()
        .map(|(a, b)| image.min_distance(a, b))
        .sum()
}

#[cfg(test)]
mod tests {

    use crate::day_11::*;

    #[test]
    fn can_parse_input() {
        let input = "\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."
            .to_string();

        assert_eq!(parse_input(&input), example_galaxy_image());
    }

    fn example_galaxy_image() -> GalaxyImage {
        GalaxyImage {
            galaxies: vec![
                (3, 0),
                (7, 1),
                (0, 2),
                (6, 4),
                (1, 5),
                (9, 6),
                (7, 8),
                (0, 9),
                (4, 9),
            ],
            clear_x: vec![2, 5, 8].into_iter().collect(),
            clear_y: vec![3, 7].into_iter().collect(),
        }
    }

    #[test]
    fn can_calculate_min_distance() {
        assert_eq!(example_galaxy_image().min_distance(&(1, 5), &(4, 9)), 9);
        assert_eq!(example_galaxy_image().min_distance(&(3, 0), &(7, 8)), 15);
        assert_eq!(example_galaxy_image().min_distance(&(0, 2), &(9, 6)), 17);
        assert_eq!(example_galaxy_image().min_distance(&(0, 9), &(4, 9)), 5);
        assert_eq!(example_galaxy_image().min_distance(&(4, 9), &(0, 9)), 5);
    }

    #[test]
    fn can_calculate_v() {
        assert_eq!(sum_of_all_pair_distances(&example_galaxy_image()), 374);
    }
}
