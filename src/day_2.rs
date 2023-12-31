//! This is my solution for [Advent of Code - Day 2: _Cube Conundrum_](https://adventofcode.com/2023/day/2)
//!
//! [`Game`]s and [`Draw`]s have structs to contain them. Most of the work is in parsing, which is quite heavily
//! subdivided:
//! * [`parse_input`]
//!     * [`parse_game`]
//!         * [`parse_id`]
//!         * [`parse_draw`]
//!             * [`parse_cube`]
//!
//! Once the input is in the internal representation it is mostly composing everything together:
//! * Part 1: [`sum_valid_game_ids`] uses [`is_valid_game`] for each line
//! * Part 2: [`sum_minimal_contents_powers`] splits the logic for each line between [`minimal_contents`] and
//!   [`draw_power`]

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
    let games = parse_input(&contents);

    println!(
        "The sum of valid game ids is {}",
        sum_valid_game_ids(&games)
    );

    println!(
        "The sum of minimal content powers is {}",
        sum_minimal_contents_powers(&games)
    );
}

/// Parse the puzzle input treating each line as a game specification
fn parse_input(input: &String) -> Vec<Game> {
    input.lines().map(parse_game).collect()
}

/// Parse a line of the puzzle input as a [`Game`]
fn parse_game(line: &str) -> Game {
    let (id_part, draws_part) = line.split_once(": ").expect("Invalid line");

    Game::new(
        parse_id(id_part),
        draws_part.split("; ").map(parse_draw).collect(),
    )
}

/// Parse `Game {{ id }}` as a numeric id
fn parse_id(id_string: &str) -> u32 {
    id_string
        .replace("Game ", "")
        .parse()
        .expect(format!("Invalid game id {}", id_string).as_str())
}

/// Parse a comma separated list of drawn cubes as a [`Draw`]
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

/// Parse e.g. `17 green` as a numeric count and the colour string
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

/// This is the solution to part 1 - delegates validity to [`is_valid_game`]
fn sum_valid_game_ids(games: &Vec<Game>) -> u32 {
    games
        .into_iter()
        .filter(|&g| is_valid_game(g))
        .map(|g| g.id)
        .sum()
}

/// Do any draws have more than the expected number of cubes
fn is_valid_game(game: &Game) -> bool {
    game.draws
        .iter()
        .all(|d| d.red <= 12 && d.green <= 13 && d.blue <= 14)
}

/// This is the solution to part 2 - delegates finding the minimal bag contents to [`minimal_contents`] and turning
/// each bag into it's power with [`draw_power`]
fn sum_minimal_contents_powers(games: &Vec<Game>) -> u32 {
    games
        .into_iter()
        .map(|game| draw_power(&minimal_contents(&game)))
        .sum()
}

/// Find the most cubes seen of each colour across the draws, giving the minimum number of each that must be in the bag
fn minimal_contents(game: &Game) -> Draw {
    let mut min_contents = Draw::new(0, 0, 0);
    for draw in &game.draws {
        min_contents.red = min_contents.red.max(draw.red);
        min_contents.green = min_contents.green.max(draw.green);
        min_contents.blue = min_contents.blue.max(draw.blue);
    }

    min_contents
}

/// Given a draw, its "power" is the cube counts multiplied together
fn draw_power(draw: &Draw) -> u32 {
    (draw.red as u32) * (draw.green as u32) * (draw.blue as u32)
}

#[cfg(test)]
mod tests {
    use crate::day_2::*;

    fn example_games() -> Vec<Game> {
        vec![
            Game::new(
                1,
                vec![Draw::new(4, 0, 3), Draw::new(1, 2, 6), Draw::new(0, 2, 0)],
            ),
            Game::new(
                2,
                vec![Draw::new(0, 2, 1), Draw::new(1, 3, 4), Draw::new(0, 1, 1)],
            ),
            Game::new(
                3,
                vec![Draw::new(20, 8, 6), Draw::new(4, 13, 5), Draw::new(1, 5, 0)],
            ),
            Game::new(
                4,
                vec![Draw::new(3, 1, 6), Draw::new(6, 3, 0), Draw::new(14, 3, 15)],
            ),
            Game::new(5, vec![Draw::new(6, 3, 1), Draw::new(1, 2, 2)]),
        ]
    }

    #[test]
    fn can_parse_game() {
        let games = example_games();

        assert_eq!(
            parse_game("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            games[0]
        );
        assert_eq!(
            parse_game("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"),
            games[1]
        );
        assert_eq!(
            parse_game("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"),
            games[2]
        );
        assert_eq!(
            parse_game("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"),
            games[3]
        );
        assert_eq!(
            parse_game("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"),
            games[4]
        );
    }

    #[test]
    fn can_parse_input() {
        let input = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            .to_string();

        assert_eq!(parse_input(&input), example_games());
    }

    #[test]
    fn can_sum_valid_games() {
        assert_eq!(sum_valid_game_ids(&example_games()), 8);
    }

    #[test]
    fn can_find_minimal_contents() {
        let games = example_games();

        assert_eq!(minimal_contents(&games[0]), Draw::new(4, 2, 6));
        assert_eq!(minimal_contents(&games[1]), Draw::new(1, 3, 4));
        assert_eq!(minimal_contents(&games[2]), Draw::new(20, 13, 6));
        assert_eq!(minimal_contents(&games[3]), Draw::new(14, 3, 15));
        assert_eq!(minimal_contents(&games[4]), Draw::new(6, 3, 2));
    }

    #[test]
    fn can_find_minimal_contents_power() {
        let games = example_games();

        assert_eq!(draw_power(&minimal_contents(&games[0])), 48);
        assert_eq!(draw_power(&minimal_contents(&games[1])), 12);
        assert_eq!(draw_power(&minimal_contents(&games[2])), 1560);
        assert_eq!(draw_power(&minimal_contents(&games[3])), 630);
        assert_eq!(draw_power(&minimal_contents(&games[4])), 36);
    }

    #[test]
    fn can_sum_minimal_contents_power() {
        assert_eq!(sum_minimal_contents_powers(&example_games()), 2286);
    }
}
