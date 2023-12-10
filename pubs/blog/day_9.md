---
day: 9
tags: [post]
header: 'Day 9: Mirage Maintenance'
---

Today I'm extrapolating values from a list of regular sequences. Mostly I seem to be writing the 
word sequence a lot. It gets a little tricky to follow at times and is very recursive.

## Part 1 - I heard you liked sequences...
The parsing is low effort as the lists of numbers only need to be interpreted as a list of numbers
with no extra meanings. Test cases are similarly simplistic. 

I mostly defer to `Itertools` to solve today. `tuple_windows` handles generating the consecutive 
pairs, so I only need to do the subtraction. Likewise, `iterator` will recursively produce the 
next sequence from the previous one, and because it's lazily generated, I can use `take_while` to 
terminate the sequences when the deltas are all `0`.

Collapsing the sequence back is the same as summing the final element of each sub-sequence. I 
feel the code explains this better and more succinctly that I did in English...

```rust
fn extrapolate_sequence(sequence: &Vec<i64>) -> i64 {
    let sequences: Vec<Vec<i64>> = 
        iterate(sequence.clone(), build_delta_sequence)
            .take_while(|seq| seq.iter().any(|&v| v != 0))
            .collect();

    sequences
        .iter()
        .rev()
        .map(|seq| seq.last().unwrap_or(&0))
        .sum()
}

fn build_delta_sequence(sequence: &Vec<i64>) -> Vec<i64> {
    sequence
        .into_iter()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .collect()
}
```

Turning that into the puzzle output requires mapping all the input lines to the extrapolated 
value, and summing them. 

```rust
fn analyse_sequences(sequences: &Vec<Vec<i64>>) -> i64 {
    sequences.iter().map(extrapolate_sequence).sum()
}
```

## Part 2 - ...so I generated a sequence from your sequence.

Generating the previous value uses the same framework. Collapsing the sequences back to the 
extrapolated value requires taking the first sequence value and recursively subtracting instead 
of adding. I refactor the sequence of sequences generator into `build_delta_sequences`, add 
`extrapolate_sequence_backwards`, and have `analyse_sequences` take the extrapolator to apply as 
an argument.

```rust
fn extrapolate_sequence_backwards(sequence: &Vec<i64>) -> i64 {
    let sequences: Vec<Vec<i64>> = build_delta_sequences(sequence);

    sequences
        .iter()
        .rev()
        .map(|seq| seq.first().unwrap_or(&0))
        .fold(0, |acc, val| val - acc)
}

fn analyse_sequences(
    sequences: &Vec<Vec<i64>>,
    extrapolator: fn(sequence: &Vec<i64>) -> i64,
) -> i64 {
    sequences.iter().map(extrapolator).sum()
}
```

## Final thoughts

Today seemed quite quick for a weekend puzzle. Part of that might be how much I was able to lean 
on library functions to do the heavy lifting. It was a fun solve, mostly because there wasn't 
much boilerplate or input manipulation before I could work on the actual problem.
