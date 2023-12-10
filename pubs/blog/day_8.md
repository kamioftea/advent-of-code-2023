---
day: 8
tags: [post]
header: 'Day 8: Haunted Wasteland'
---

Today I need to follow a bunch of cryptic maps to escape from a sandstorm. 

## Data model

The data can mostly be represented as is. I create an enum for the Left/right instructions. 
I'm happy to store the nodes in a HashMap<&str, (&str, &str)>, but I'll label the `Node` tuple and 
`Network` with type aliases. The input representation being close to the internal structure also 
makes it low effort to convert the examples into tests, especially using multiple cursors to 
add the boilerplate in bulk.

```rust
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

type Node<'a> = (&'a str, &'a str);

type Network<'a> = HashMap<&'a str, Node<'a>>;
```

Parsing is also fairly standard, breaking the input into the directions (which can use the 
Instruction::try_from defined above) and network, then the network into lines and doing a bit of 
clean up.

```rust
fn parse_instructions(line: &str) -> Vec<Instruction> {
    line.chars().filter_map(|c| c.try_into().ok()).collect()
}

fn parse_network(network_spec: &str) -> Network {
    network_spec.lines().map(parse_node).collect()
}

fn parse_node(node_spec: &str) -> (&str, Node) {
    let (label, connections) = node_spec.split_once(" = ").unwrap();
    let (left, right) = connections.split_once(", ").unwrap();

    (
        label,
        (left.trim_start_matches("("), right.trim_end_matches(")")),
    )
}
```

## Part 1 - Do the monkey!

For part one, I simulate following the map step by step. There's not much to say here, the 
puzzle instructions map directly into code, and this passes the tests and produces the correct 
solution for the puzzle input.

```rust
fn count_steps(instructions: &Vec<Instruction>, network: &Network) -> usize {
    let mut steps = 0;
    let mut position = "AAA";
    let instruction_length = instructions.len();

    while position != "ZZZ" {
        let direction = instructions.get(steps % instruction_length).unwrap();
        let &(left, right) = network.get(position).unwrap();

        position = if *direction == Left { left } else { right };
        steps += 1;
    }

    steps
}
```

## Part two - Ghosts wandering forever

I initially tried to replicate part one, but stepping each ghost per instruction.

```rust
fn count_parallel_steps(instructions: &Vec<Instruction>, network: &Network) -> usize {
    let mut steps = 0;
    let positions: Vec<&str> = network
        .keys()
        .filter(|k| k.ends_with("A"))
        .map(|&k| k)
        .collect();
    let instruction_length = instructions.len();
    
    while !positions.iter().all(|p| p.ends_with("Z")) {
        let direction = instructions.get(steps % instruction_length).unwrap();
        for position in positions.iter_mut() {
            let &(left, right) = network.get(position).unwrap();
            
            *position = if *direction == Left { left } else { right };
        }
        
        steps += 1;
    }
    
    steps
}
```

It quickly became clear that wouldn't complete.

I then tried multiplying all the loop lengths together, which involved being able to call 
`count_steps` from part one with a variable start position, and a more relaxed predicate for 
when a terminal state had been reached.

```rust
fn count_parallel_steps(instructions: &Vec<Instruction>, network: &Network) -> usize {
    network
        .keys()
        .filter(|k| k.ends_with("A"))
        .map(|&start| count_steps(start, part_2_terminal, instructions, network))
        .product()
}
```

That was too high, so I tried using the lowest common multiplier, which is implemented for me in 
a handy crate that extends `Integer`. 

```rust
fn part_1_terminal(position: &str) -> bool {
    position == "ZZZ"
}

fn part_2_terminal(position: &str) -> bool {
    position.ends_with("Z")
}

fn count_parallel_steps(instructions: &Vec<Instruction>, network: &Network) -> usize {
    network
        .keys()
        .filter(|k| k.ends_with("A"))
        .map(|&start| count_steps(start, part_2_terminal, instructions, network))
        .fold(1, |acc, steps| steps.lcm(&acc))
}
```

And that... worked? It gave the right answer anyway.

## Final thoughts

Today, especially part two, was quite unsatisfying. The lowest common multiplier (LCM) solution 
needed very specific input to work, and there was nothing about the scenario depicted that 
hinted at it being a viable solution. It was a guess whilst I was investigating the puzzle input.
The example was also very small, and didn't really give any insight into the structure of the puzzle 
data either that might have guided someone towards the solution either. Eric Wastl has to write 
a lot of puzzles and wrangle them into a narrative, so they're not always going to land with 
everyone. Hopefully tomorrow will back to the usual great standard.
