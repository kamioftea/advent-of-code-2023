//! This is my solution for [Advent of Code - Day 2: _Cube Conundrum_](https://adventofcode.com/2023/day/2)
//!
//!

use std::fs;

/// A record of the cubes shown in a single draw from a bag
#[derive(Eq, PartialEq, Debug)]
struct Draw {
    red: u8,
    green: u8,
    blue: u8,
}

impl Draw {
    fn new(red: u8, green: u8, blue: u8) -> Draw {
        return Draw { red, green, blue };
    }
}

/// A record of draws made with a specific combination of cubes
#[derive(Eq, PartialEq, Debug)]
struct Game {
    id: u32,
    draws: Vec<Draw>,
}

impl Game {
    fn new(id: u32, draws: Vec<Draw>) -> Game {
        return Game { id, draws };
    }
}

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-2-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 2.
pub fn run() {
    let contents = fs::read_to_string("res/day-2-input.txt").expect("Failed to read file");

    println!(
        "The sum of valid game ids is {}",
        sum_valid_games(&contents)
    );
}

fn sum_valid_games(input: &String) -> u32 {
    input
        .lines()
        .map(parse_game)
        .filter(is_valid_game)
        .map(|g| g.id)
        .sum()
}

fn is_valid_game(game: &Game) -> bool {
    game.draws
        .iter()
        .all(|d| d.red <= 12 && d.green <= 13 && d.blue <= 14)
}

fn parse_game(line: &str) -> Game {
    let (id_part, draws_part) = line.split_once(": ").expect("Invalid line");

    Game::new(
        parse_id(id_part),
        draws_part.split("; ").map(parse_draw).collect(),
    )
}

fn parse_id(id_string: &str) -> u32 {
    id_string
        .replace("Game ", "")
        .parse()
        .expect(format!("Invalid game id {}", id_string).as_str())
}

fn parse_draw(draw_str: &str) -> Draw {
    let mut draw = Draw::new(0, 0, 0);
    for (colour, count) in draw_str.split(", ").map(parse_cube) {
        match colour {
            "red" => draw.red = count,
            "green" => draw.green = count,
            "blue" => draw.blue = count,
            _ => unreachable!("Invalid colour {}", colour),
        }
    }

    draw
}

fn parse_cube(cube_str: &str) -> (&str, u8) {
    let (count_str, colour) = cube_str
        .split_once(" ")
        .expect(format!("Invalid cube {}", cube_str).as_str());

    return (
        colour,
        count_str
            .parse()
            .expect(format!("Invalid count {}", count_str).as_str()),
    );
}

#[cfg(test)]
mod tests {
    use crate::day_2::*;

    #[test]
    fn can_parse_game() {
        assert_eq!(
            parse_game("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            Game::new(
                1,
                vec! {
                    Draw::new(4, 0, 3),
                    Draw::new(1, 2, 6),
                    Draw::new(0, 2, 0),
                },
            )
        );
        assert_eq!(
            parse_game("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"),
            Game::new(
                2,
                vec! {
                    Draw::new(0, 2, 1),
                    Draw::new(1, 3, 4),
                    Draw::new(0, 1, 1),
                },
            )
        );
        assert_eq!(
            parse_game("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"),
            Game::new(
                3,
                vec! {
                    Draw::new(20, 8, 6),
                    Draw::new(4, 13, 5),
                    Draw::new(1, 5, 0),
                },
            )
        );
        assert_eq!(
            parse_game("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"),
            Game::new(
                4,
                vec! {
                    Draw::new(3, 1, 6),
                    Draw::new(6, 3, 0),
                    Draw::new(14, 3, 15),
                },
            )
        );
        assert_eq!(
            parse_game("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"),
            Game::new(
                5,
                vec! {
                    Draw::new(6, 3, 1),
                    Draw::new(1, 2, 2),
                }
            )
        );
    }

    #[test]
    fn can_sum_valid_games() {
        let input = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            .to_string();

        assert_eq!(sum_valid_games(&input), 8);
    }

    #[test]
    fn can_find_minimal_contents() {}
}
