//! This is my solution for [Advent of Code - Day 10: _Pipe Maze_](https://adventofcode.com/2023/day/10)
//!
//!

use std::collections::HashSet;
use std::fs;

const NORTH: u8 = 1;
const EAST: u8 = 2;
const SOUTH: u8 = 4;
const WEST: u8 = 8;

type Coordinate = (isize, isize);

type Cell = u8;

#[derive(Eq, PartialEq, Debug)]
struct PipeMaze {
    grid: Vec<Vec<Cell>>,
    start: Coordinate,
}

impl PipeMaze {
    fn get(&self, coordinate: Coordinate) -> Option<&Cell> {
        if coordinate.0 < 0 || coordinate.1 < 0 {
            None
        } else {
            self.grid
                .get(coordinate.1 as usize)
                .and_then(|row| row.get(coordinate.0 as usize))
        }
    }

    fn set(&mut self, coordinate: Coordinate, shape: Cell) {
        if coordinate.0 >= 0 && coordinate.1 >= 0 {
            let row = self.grid.get_mut(coordinate.1 as usize).unwrap();
            let cell = row.get_mut(coordinate.0 as usize).unwrap();
            *cell = shape
        }
    }
}

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-10-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 10.
pub fn run() {
    let contents = fs::read_to_string("res/day-10-input.txt").expect("Failed to read file");

    let maze = parse_maze(&contents);

    println!(
        "the path to the furthest point is: {}",
        count_steps_to_furthest_point(&maze)
    );

    println!(
        "the number of enclosed spaces is: {}",
        count_enclosed_spaces(&maze)
    )
}

fn parse_maze(input: &String) -> PipeMaze {
    let mut grid: Vec<Vec<Cell>> = Vec::new();
    let mut eventual_start: Option<Coordinate> = None;
    for (y, line) in input.lines().enumerate() {
        let mut row: Vec<Cell> = Vec::new();
        for (x, chr) in line.chars().enumerate() {
            row.push(parse_cell(&chr));
            if chr == 'S' {
                eventual_start = Some((x as isize, y as isize));
            }
        }

        grid.push(row)
    }

    let mut maze = PipeMaze {
        grid,
        start: eventual_start.unwrap(),
    };

    let start_shape = find_start_shape(&maze);

    maze.set(eventual_start.unwrap(), start_shape);

    maze
}

///
/// | is a vertical pipe connecting north and south.
/// - is a horizontal pipe connecting east and west.
/// L is a 90-degree bend connecting north and east.
/// J is a 90-degree bend connecting north and west.
/// 7 is a 90-degree bend connecting south and west.
/// F is a 90-degree bend connecting south and east.
/// . is ground; there is no pipe in this tile.
/// S is the starting position of the animal; there is a pipe on this tile, but your sketch
/// doesn't show what shape the pipe has.
fn parse_cell(chr: &char) -> Cell {
    match chr {
        '|' => NORTH | SOUTH,
        '-' => EAST | WEST,
        'L' => NORTH | EAST,
        'J' => NORTH | WEST,
        '7' => SOUTH | WEST,
        'F' => SOUTH | EAST,
        _ => 0,
    }
}

fn inverse(cell: Cell) -> u8 {
    match cell {
        NORTH => SOUTH,
        EAST => WEST,
        SOUTH => NORTH,
        WEST => EAST,
        0 => 0,
        _ => {
            inverse(cell & NORTH)
                | inverse(cell & EAST)
                | inverse(cell & SOUTH)
                | inverse(cell & WEST)
        }
    }
}

fn count_steps_to_furthest_point(maze: &PipeMaze) -> usize {
    find_pipe_path(maze).len() / 2
}

fn find_pipe_path(maze: &PipeMaze) -> HashSet<Coordinate> {
    let start_shape = find_start_shape(maze);
    let mut direction = find_start_direction(start_shape);

    let mut position = maze.start;
    let mut path = HashSet::new();

    loop {
        position = match direction {
            NORTH => (position.0, position.1 - 1),
            EAST => (position.0 + 1, position.1),
            SOUTH => (position.0, position.1 + 1),
            WEST => (position.0 - 1, position.1),
            _ => unreachable!(),
        };

        path.insert(position);

        if position == maze.start {
            break;
        }

        direction = maze.get(position).unwrap() & !inverse(direction)
    }

    path
}

fn find_start_direction(start_shape: u8) -> u8 {
    *vec![
        start_shape & NORTH,
        start_shape & EAST,
        start_shape & SOUTH,
        start_shape & WEST,
    ]
    .iter()
    .find(|&&d| d != 0)
    .unwrap()
}

fn find_start_shape(maze: &PipeMaze) -> u8 {
    let to_search = vec![
        ((-1, 0), EAST),
        ((1, 0), WEST),
        ((0, -1), SOUTH),
        ((0, 1), NORTH),
    ];

    let mut shape = 0;

    for ((dx, dy), required_direction) in to_search {
        let x = maze.start.0 + dx;
        let y = maze.start.1 + dy;
        if let Some(cell) = maze.get((x, y)) {
            shape |= cell & required_direction;
        }
    }

    inverse(shape)
}

fn count_enclosed_spaces(maze: &PipeMaze) -> usize {
    let path = find_pipe_path(maze);
    let (max_x, max_y) = path.iter().fold((0, 0), |(acc_x, acc_y), &(val_x, val_y)| {
        (acc_x.max(val_x), acc_y.max(val_y))
    });

    let mut to_visit: Vec<Coordinate> = Vec::new();
    let mut visited: HashSet<Coordinate> = HashSet::new();
    to_visit.push((0, 0));

    while let Some((x, y)) = to_visit.pop() {
        if visited.contains(&(x, y)) {
            continue;
        }

        visited.insert((x, y));

        let to_search = vec![
            ((-1, 0), true, NORTH),
            ((1, 0), false, NORTH),
            ((0, -1), true, WEST),
            ((0, 1), false, WEST),
        ];

        for ((dx, dy), check_new, direction) in to_search {
            let new_x = x + dx;
            let new_y = y + dy;

            if new_x < 0 || new_y < 0 || new_x > max_x + 1 || new_y > max_y + 1 {
                continue;
            }

            if visited.contains(&(new_x, new_y)) {
                continue;
            }

            let (check_x, check_y) = if check_new { (new_x, new_y) } else { (x, y) };

            if !path.contains(&(check_x, check_y)) {
                to_visit.push((new_x, new_y))
            } else if let Some(&cell) = maze.get((check_x, check_y)) {
                if (cell & direction) == 0 {
                    to_visit.push((new_x, new_y))
                }
            }
        }
    }

    let mut internal_count = 0;

    for maze_y in 0..=max_y {
        for maze_x in 0..=max_x {
            if !path.contains(&(maze_x, maze_y)) && !visited.contains(&(maze_x, maze_y)) {
                internal_count += 1;
            }
        }
    }

    internal_count
}

#[cfg(test)]
mod tests {
    use crate::day_10::*;

    fn example_maze_with_square_loop() -> PipeMaze {
        let grid = vec![
            vec![0, 0, 0, 0, 0],
            vec![0, 6, 10, 12, 0],
            vec![0, 5, 0, 5, 0],
            vec![0, 3, 10, 9, 0],
            vec![0, 0, 0, 0, 0],
        ];

        PipeMaze {
            grid,
            start: (2, 3),
        }
    }

    //noinspection SpellCheckingInspection
    /// 7-F7-
    /// .FJ|7
    /// SJLL7
    /// |F--J
    /// LJ.LJ
    fn example_maze_with_complex_loop_and_noise() -> PipeMaze {
        let grid = vec![
            vec![12, 10, 6, 12, 10],
            vec![0, 6, 9, 5, 12],
            vec![6, 9, 3, 3, 12],
            vec![5, 6, 10, 10, 9],
            vec![3, 9, 0, 3, 9],
        ];

        PipeMaze {
            grid,
            start: (0, 2),
        }
    }

    #[test]
    fn can_parse() {
        let input = "\
.....
.F-7.
.|.|.
.LSJ.
....."
            .to_string();

        assert_eq!(parse_maze(&input), example_maze_with_square_loop());
    }

    //noinspection SpellCheckingInspection
    #[test]
    fn can_parse_with_noise() {
        let input = "\
7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ"
            .to_string();

        assert_eq!(
            parse_maze(&input),
            example_maze_with_complex_loop_and_noise()
        );
    }

    #[test]
    fn can_find_start_shape() {
        assert_eq!(find_start_shape(&example_maze_with_square_loop()), 10);
        assert_eq!(
            find_start_shape(&example_maze_with_complex_loop_and_noise()),
            6
        );
    }

    #[test]
    fn can_find_furthest_point() {
        assert_eq!(
            count_steps_to_furthest_point(&example_maze_with_square_loop()),
            4
        );
        assert_eq!(
            count_steps_to_furthest_point(&example_maze_with_complex_loop_and_noise()),
            8
        );
    }

    //noinspection SpellCheckingInspection
    #[test]
    fn can_count_enclosed_spaces() {
        assert_eq!(count_enclosed_spaces(&example_maze_with_square_loop()), 1);
        assert_eq!(
            count_enclosed_spaces(&example_maze_with_complex_loop_and_noise()),
            1
        );

        assert_eq!(
            count_enclosed_spaces(&parse_maze(
                &"\
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."
                    .to_string()
            )),
            4
        );

        assert_eq!(
            count_enclosed_spaces(&parse_maze(
                &"\
..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
.........."
                    .to_string()
            )),
            4
        );

        assert_eq!(
            count_enclosed_spaces(&parse_maze(
                &"\
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ..."
                    .to_string()
            )),
            8
        );

        assert_eq!(
            count_enclosed_spaces(&parse_maze(
                &"\
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"
                    .to_string()
            )),
            10
        );
    }
}
