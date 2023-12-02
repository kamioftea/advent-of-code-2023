---
day: 2
tags: [ post ]
header: 'Day 2: Cube Conundrum'
---

The challenge today mostly revolved about parsing structured text. I tend to take a bit longer with these to define
a data model out of structs instead of using numbers and tuples. It has the upfront cost of setting it all up, but
makes the resulting code much more readable.

## Setting up the data model

First I write out what I want the internal representation to be. In this case it looks like a struct for each `Game`,
and within that the id and a list of `Draw`s. I'll add constructors to make writing test instances easier, and
derive the traits needed to use them with `assert_eq!`.

```rust
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
    id: u8,
    draws: Vec<Draw>,
}

impl Game {
    fn new(id: u32, draws: Vec<Draw>) -> Game {
        return Game { id, draws };
    }
}
```

The test cases just need to be written out from the example data. A little tedious, but only needs to be done once.

```rust
#[test]
fn can_parse_game() {
    assert_eq!(
        parse_game("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
        Game::new(
            1,
            vec!(Draw::new(3, 0, 2), Draw::new(1, 2, 6), Draw::new(0, 2, 0))
        )
    );
    // Repeat for games 2-5
}
```

Actually parsing them is a case of splitting each layer into its parts and then delegating each part to a smaller parser
until it doesn't make sense to split it out further. I started with parsing a line into a `Game`, split that into
the `id` and `draws`. The `id` required parsing the number and then that is done. The list of draws were split into
parsing each one, and that into parsing each cube as a count / colour pair.

There is a quirk of the input that the colour ordering is not standard, but I can start with an empty draw and 
update each colour as that one is seen.

```rust
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
```

When I first wrote it I had the count and the colour the wrong way round, but the tests caught that. You may have
noticed I'd transcribed the counts for the first draw in the test above. The test output puts one above the other
enabling me to quickly see what was wrong and correct it.

## Part 1 - How many cubes am I holding up?

To actually solve the puzzle I need to filter out the games where too many cubes of a colour were shown, and reduce
that to a single answer by summing the game ids. Firstly the example can be a test case:

```rust
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
```

Then I can write out the top level logic using built in iterable methods, delegating the actual validation to another
function. Then write that function. I've used hardcoded thresholds here because it'll be easy enough to refactor them
out to a parameter later if part two needs different values.

```rust
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
```

This works and produces the right sum when applied to the puzzle input.

## Part 2 - What's in the bag

The second part is where having already structured the input into a data model pays off. I can reduce the individual 
draws into another draw with the maximum of each colour using a similar technique to that when parsing them, and 
calculating the power is a simple multiplication, and the solution is then composing these using iterables.

For each step, I can implement a test from the example then a function to implement it.

Determining the minimum number of each cube in the bag:
```rust
#[test]
fn can_find_minimal_contents() {
    let games = example_games();

    assert_eq!(minimal_contents(&games[0]), Draw::new(4, 2, 6));
    assert_eq!(minimal_contents(&games[1]), Draw::new(1, 3, 4));
    assert_eq!(minimal_contents(&games[2]), Draw::new(20, 13, 6));
    assert_eq!(minimal_contents(&games[3]), Draw::new(14, 3, 15));
    assert_eq!(minimal_contents(&games[4]), Draw::new(6, 3, 2));
}

fn minimal_contents(game: &Game) -> Draw {
    let mut min_contents = Draw::new(0, 0, 0);
    for draw in &game.draws {
        min_contents.red = min_contents.red.max(draw.red);
        min_contents.green = min_contents.green.max(draw.green);
        min_contents.blue = min_contents.blue.max(draw.blue);
    }

    min_contents
}
```

Calculate the power of each set of cubes:

```rust
#[test]
fn can_find_minimal_contents_power() {
    let games = example_games();

    assert_eq!(draw_power(&minimal_contents(&games[0])), 48);
    assert_eq!(draw_power(&minimal_contents(&games[1])), 12);
    assert_eq!(draw_power(&minimal_contents(&games[2])), 1560);
    assert_eq!(draw_power(&minimal_contents(&games[3])), 630);
    assert_eq!(draw_power(&minimal_contents(&games[4])), 36);
}

fn draw_power(draw: &Draw) -> u32 {
    (draw.red as u32) * (draw.green as u32) * (draw.blue as u32)
}
```

... Then reduce the input into the sum of the powers to give a single number as the puzzle answer:

```rust
#[test]
fn can_sum_minimal_contents_power() {
    let input = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
        .to_string();

    assert_eq!(sum_minimal_contents_powers(&input), 2286);
}

fn sum_minimal_contents_powers(input: &String) -> u32 {
    input
        .lines()
        .map(parse_game)
        .map(|game| draw_power(&minimal_contents(&game)))
        .sum()
}
```

This gives the correct solution. I do one small refactor. I'm repeating the parsing of the input string 
unnecessarily, so I:

* Extract the common logic into a `parse_input` method and test,
* Update `sum_valid_games` and `sum_minimal_contents_powers` to expect a `&Vec<Game>`, and 
* Update the tests and run function to match the new API. This has the bonus of removing the repeated example input 
  string.

## Final thoughts

Today felt much more like idiomatic early advent of code. It was good to get into the rhythm of implement logic, 
delegate functionality to a stub, implement logic for each stub, ... until it was done. I continue to be happy 
with how comfortable I'm feeling with the Rust basics. Bring on the rest of the month 🙂.
