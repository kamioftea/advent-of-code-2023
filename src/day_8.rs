//! This is my solution for [Advent of Code - Day 8: _Haunted Wasteland_](https://adventofcode.com/2023/day/8)
//!
//! Follows a map across a network of nodes. [`parse_input`] delegates to [`parse_instructions`],
//! [`parse_network`], and [`parse_node`] to represent list of [`Instruction`]s and [`Network`]
//! or [`Node`]s.
//!
//! [`count_steps`] counts the steps from a specific starting node to one that satisfies a given
//! predicate. [`count_parallel_steps`] determines how long the ghosts need to cycle until they
//! all reach a destination, assuming they are all on a regular loop through the network.

use crate::day_8::Instruction::{Left, Right};
use num::Integer;
use std::collections::HashMap;
use std::fs;

/// An instruction determining which branch to follow when moving to the next node
#[derive(Eq, PartialEq, Debug)]
enum Instruction {
    Left,
    Right,
}

impl TryFrom<char> for Instruction {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Left),
            'R' => Ok(Right),
            _ => Err(()),
        }
    }
}

/// A node in the network defined by the labels of the nodes you can reach next
type Node<'a> = (&'a str, &'a str);

/// A networks of [`Node`]s indexed by their label.
type Network<'a> = HashMap<&'a str, Node<'a>>;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-8-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 8.
pub fn run() {
    let contents = fs::read_to_string("res/day-8-input.txt").expect("Failed to read file");

    let (instructions, network) = parse_input(&contents);

    println!(
        "The number of steps is: {}",
        count_steps("AAA", part_1_terminal, &instructions, &network)
    );

    println!(
        "The number of ghost steps is: {}",
        count_parallel_steps(&instructions, &network)
    );
}

/// Parse the input as an [`Instruction`] line followed by a [`Network`] of [`Node`]s
fn parse_input(input: &String) -> (Vec<Instruction>, Network) {
    let (instructions_spec, network_spec) = input.split_once("\n\n").unwrap();

    (
        parse_instructions(instructions_spec),
        parse_network(network_spec),
    )
}

/// Parse a line of `L` and `R` as [`Instruction`]s.
fn parse_instructions(line: &str) -> Vec<Instruction> {
    line.chars().filter_map(|c| c.try_into().ok()).collect()
}

/// Parse each line of the spec as a labelled [`Node`] in a [`Network`].
fn parse_network(network_spec: &str) -> Network {
    network_spec.lines().map(parse_node).collect()
}

/// Parse a line in the format e.g. `AAA = (BBB, CCC)` as a node labelled `AAA`, linked to `BBB`
/// and `CCC` on the left and right respectively.
fn parse_node(node_spec: &str) -> (&str, Node) {
    let (label, connections) = node_spec.split_once(" = ").unwrap();
    let (left, right) = connections.split_once(", ").unwrap();

    (
        label,
        (left.trim_start_matches("("), right.trim_end_matches(")")),
    )
}

/// The destination for part one is the specific node labelled `ZZZ`
fn part_1_terminal(position: &str) -> bool {
    position == "ZZZ"
}

/// Any node ending in `Z` counts as a destination for part 2
fn part_2_terminal(position: &str) -> bool {
    position.ends_with("Z")
}

/// Follow the the list of instructions in a cycle until a destination node is reached
fn count_steps(
    start: &str,
    terminal_predicate: fn(&str) -> bool,
    instructions: &Vec<Instruction>,
    network: &Network,
) -> usize {
    let mut steps = 0;
    let mut position = start;
    let instruction_length = instructions.len();

    while !terminal_predicate(position) {
        let direction = instructions.get(steps % instruction_length).unwrap();
        let &(left, right) = network.get(position).unwrap();

        position = if *direction == Left { left } else { right };
        steps += 1;
    }

    steps
}

/// Given a ghost starts at each of the nodes ending in `A`, and each follows the instructions in
/// parallel, how many steps until they are all on a terminal node at the same time.
///
/// The ghosts each follow a fixed cycle of steps, so this can be determined using the least
/// common multiple of each of their cycle lengths
fn count_parallel_steps(instructions: &Vec<Instruction>, network: &Network) -> usize {
    network
        .keys()
        .filter(|k| k.ends_with("A"))
        .map(|&start| count_steps(start, part_2_terminal, instructions, network))
        .fold(1, |acc, steps| steps.lcm(&acc))
}

#[cfg(test)]
mod tests {
    use crate::day_8::*;

    fn example_networks() -> Vec<Network<'static>> {
        vec![
            vec![
                ("AAA", ("BBB", "CCC")),
                ("BBB", ("DDD", "EEE")),
                ("CCC", ("ZZZ", "GGG")),
                ("DDD", ("DDD", "DDD")),
                ("EEE", ("EEE", "EEE")),
                ("GGG", ("GGG", "GGG")),
                ("ZZZ", ("ZZZ", "ZZZ")),
            ]
            .into_iter()
            .collect(),
            vec![
                ("AAA", ("BBB", "BBB")),
                ("BBB", ("AAA", "ZZZ")),
                ("ZZZ", ("ZZZ", "ZZZ")),
            ]
            .into_iter()
            .collect(),
        ]
    }

    #[test]
    fn can_parse_input() {
        let input_0 = "\
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"
            .to_string();

        let input_1 = "\
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"
            .to_string();

        let networks = example_networks();

        let (instructions_0, network_0) = parse_input(&input_0);

        assert_eq!(instructions_0, vec![Right, Left]);
        assert_eq!(network_0, networks[0]);

        let (instructions_1, network_1) = parse_input(&input_1);

        assert_eq!(instructions_1, vec![Left, Left, Right]);
        assert_eq!(network_1, networks[1]);
    }

    #[test]
    fn can_count_steps() {
        let networks = example_networks();

        assert_eq!(
            count_steps("AAA", part_1_terminal, &vec![Right, Left], &networks[0]),
            2
        );
        assert_eq!(
            count_steps(
                "AAA",
                part_1_terminal,
                &vec![Left, Left, Right],
                &networks[1]
            ),
            6
        );
    }

    #[test]
    fn can_count_parallel_steps() {
        let input = "\
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"
            .to_string();

        let (instructions, network) = parse_input(&input);

        assert_eq!(count_parallel_steps(&instructions, &network), 6);
    }
}
