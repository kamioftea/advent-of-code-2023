---
day: 1
tags: post
header: 'Day 1: Calorie Counting'
---
After some preamble about this year's theme (delving into the jungle to find magical Star fruit), I have my first task:
I need to help the elves on the expedition identify which of them has the most snacks. Each elf has written down the 
calories of each item their category, separating themselves from the previous elf with a blank line.

## Parse the input

```text
1000           |
2000           |---- Elf 1 has three snacks totalling 6,000 calories.
3000           |
          
4000           |---- Elf 2 has one snack with 4,000 calories.
          
5000           |---- Elf 3 has two snacks totalling 11,000 calories. 
6000           |
```

So the first thing to do is turn that into a useful representation. I'm not aware of a handy way to split an iterator 
using a delimiter with Rust, so I'll need to do this myself. Each line will either be a string that is parsable as 
an unsigned integer, or blank which will error when parsed. Given I'm building a single expedition from an iterator of 
lines I'll use `iter::fold` here, with the accumulator being a pair of `(Expedition, Elf)`, as we also need to keep 
track of the list of calories for the elf we are currently parsing. 

1. If it parses to an `Ok<u32>` concatenate the parsed number to the `Vec` representing the current elf we're parsing.
2. If it parses to an `Err` of any kind, add the now finished elf to the expedition accumulator, and start a new 
   empty elf.

Rust's very powerful `match` pattern matching means we can express this exact logic, and also have the compiler confirm
we've not missed any possible results of the parse. The concatenation here is a bit clunky - I'll come back to that when
I refactor this.

```rust
// Some type aliases to make the code match the domain
type Elf = Vec<u32>;
type Expedition = Vec<Elf>;

fn parse_input(input: &String) -> Expedition {
    let (expedition, current_elf) =
        input.lines()
             .fold::<(Expedition, Elf), _>(
                 (Vec::new(), Vec::new()),
                 |(expedition, current_elf), line|
                     match line.parse::<u32>() {
                         Ok(calories) => (
                             expedition, 
                             [current_elf, vec!(calories)].concat()
                         ),
                         Err(_) => (
                             [expedition, vec!(current_elf)].concat(), 
                             Vec::new()
                         )
                     },
             );

    [expedition, vec!(current_elf)].concat()
}
```

The puzzle provides some sample data that can be used as a test case:

```rust
#[cfg(test)]
mod tests {
    use crate::day_1::{parse_input};

    fn sample_expedition() -> Expedition {
        vec![
            vec![1000, 2000, 3000],
            vec![4000],
            vec![5000, 6000],
            vec![7000, 8000, 9000],
            vec![10000],
        ]
    }

    #[test]
    fn can_parse_sample_input() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000".to_string();

        assert_eq!(parse_input(&input), sample_expedition());
    }
}
```

## Part 1 - Find the most calories carried

To get the elves with the most calories I need to sum each elf's list, and find the maximum total. This can be done 
with a few chained iterators. Accept a pointer to the expedition as it doesn't need to be modified and this way the 
function only need to borrow the expedition, not consume it.

```rust
fn can_find_most_calories(expedition: &Expedition) -> u32 {
   expedition
       .into_iter()
       .map(|elf| elf.iter().sum())
       .max()
       .unwrap_or(0)
}
```

And the puzzle description has also provided a test case here:

```rust
 #[test]
 fn can_find_most_calories() {
     assert_eq!(find_most_calories(&sample_expedition()), 24000)
 }
```

I have standard code in my daily template to parse the day's input, so I'll plug that into `find_most_calories` to get 
the answer to part one.

```rust
pub fn run() {
    let contents = 
        fs::read_to_string("res/day-1-input").expect("Failed to read file");
    let expedition = parse_input(&contents);

    println!(
        "The most calories carried by one elf is: {}",
        find_most_calories()
    );
}
// The most calories carried by one elf is: 72240
```

## Part 2 - Find some backup elves

Finding one elf with the most is not enough, the Jungle is dangerous, and elves can get quite hungry. I need to 
have an idea of some backup elves who have good food supplies. To this end I need to find the total calories carried
by the top _three_ elves when ranked by calories carried.

Firstly I now know that I only need the list of total calories, so I'll do some refactoring to extract that, so it 
can be re-used.

```rust
fn build_elf_calorie_totals(expedition: &Expedition) -> Vec<u32> {
   expedition
        .into_iter()
        .map(|calories| calories.iter().sum())
        .collect()
}
// ...
fn find_most_calories(elf_calorie_totals: &Vec<u32>) -> u32 {
   *elf_calorie_totals.iter()
                      .max()
                      .unwrap_or(&0)
}
// ...
fn sample_elf_calorie_totals() -> Vec<u32> {
   build_elf_calorie_totals(&sample_expedition())
}

#[test]
fn can_find_most_calories() {
   assert_eq!(find_most_calories(&sample_elf_calorie_totals()), 24000)
}
// ...
pub fn run() {
   let contents =
       fs::read_to_string("res/day-1-input").expect("Failed to read file");
   let expedition = parse_input(&contents);
   let elf_calorie_totals = build_elf_calorie_totals(&expedition);

   println!(
      "The most calories carried by one elf is: {}",
      find_most_calories(&elf_calorie_totals)
   );
}
```

The easiest way to achieve what I want is to sort the elves, then take the top three. I can work on efficiency later.

```rust
fn find_top_three_total(elf_calorie_totals: &Vec<u32>) -> u32 {
   let mut totals = elf_calorie_totals.to_owned();
   totals.sort();
   // Sorting gives lowest first
   totals.reverse();
   totals.iter().take(3).sum()
}
// ...
#[test]
fn can_find_top_three_total() {
   assert_eq!(find_top_three_totals(&sample_elf_calorie_totals()), 45000)
}
// ...
pub fn run() {
   let contents =
       fs::read_to_string("res/day-1-input").expect("Failed to read file");
   let expedition = parse_input(&contents);
   let elf_calorie_totals = build_elf_calorie_totals(&expedition);

   println!(
      "The most calories carried by one elf is: {}",
      find_most_calories(&elf_calorie_totals)
   );

   println!(
      "The total calories carried by the top three elves is: {}",
      find_top_three_total(&elf_calorie_totals)
   );
}

// The most calories carried is: 72240
// The total calories carried by the top three elves is: 210957
```

## Optimisation and other refactorings

There are a couple of inefficient things I'm doing that are not necessary given the needs of the puzzle.

1. The logic actually solving the puzzle only cares about the calorie totals, storing the intermediate Elves
   (aliased `Vec<u32>`s) is not needed and will be taking quite a bit of time to allocate.
2. Sorting the full list of elves when we only need the top three.

So I'll first create a new parsing function that builds the totals per elf directly. I'm also starting to get back
into the Rust mindset more. Rather than folding I'll use a couple of mutable local variables for the expedition and the
running total for the current elf. The compiler can detect race conditions and other perils of mutable state, 
so it is safer to use it than other languages I'm used to.

```rust
fn parse_input_to_calorie_totals(input: &String) -> Vec<u32> {
   let mut elf_calorie_totals = Vec::new();
   let mut current_elf_total = 0;

   input.lines().for_each(
      |line| match line.parse::<u32>() {
         Ok(calories) => current_elf_total = current_elf_total + calories,
         Err(_) => {
            elf_calorie_totals.push(current_elf_total);
            current_elf_total = 0;
         }
      }
   );

   if current_elf_total > 0 {
      elf_calorie_totals.push(current_elf_total)
   }

   elf_calorie_totals
}
// ...
#[test]
fn can_parse_sample_input() {
   let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000".to_string();
   
   assert_eq!(parse_input_to_calorie_totals(&input), sample_elf_calorie_totals());
}
// ...
pub fn run() {
   let contents =
       fs::read_to_string("res/day-1-input").expect("Failed to read file");
   let _elf_calorie_totals = parse_input_to_calorie_totals(&contents);
   // ...
}
```

This runs roughly five times faster.

For the second optimisation, I can fold over the calorie totals keeping only the highest three calorie totals seen so
far. The types here got quite complicated, so I pulled the reducing function out into a concrete function with the type
signature I knew wanted, so that it wasn't complicated by Rust inferring more general types than are needed. Again
`match` can be used quite expressively to show how we check each current candidate elf in turn to see if the new elf's
calorie total is a better candidate.

Note I've also decided to return the separate results here so that this one calculation can be used for both part 
one and two.

```rust
fn find_top_three_calorie_totals(elf_calorie_totals: &Vec<u32>) -> (u32, u32, u32) {
   elf_calorie_totals.iter().fold((0, 0, 0), bubble_calorie_total_into_top_three)
}

fn bubble_calorie_total_into_top_three(
   top_3: (u32, u32, u32), 
   &elf: &u32
) -> (u32, u32, u32) {
    match top_3 {
        (a, b, _) if a < elf => (elf, a, b),
        (a, b, _) if b < elf => (a, elf, b),
        (a, b, c) if c < elf => (a, b, elf),
        _ => top_3
    }
}
// ...
#[test]
fn can_find_top_three_calories() {
   assert_eq!(
      find_top_three_calorie_totals(&sample_elf_calorie_totals()), 
      (24000, 11000, 10000)
   )
}
// ...
pub fn run() {
   let contents = fs::read_to_string("res/day-1-input")
       .expect("Failed to read file");
   let elf_calorie_totals = parse_input_to_calorie_totals(&contents);
   let (first, second, third) = find_top_three_calorie_totals(&elf_calorie_totals);

   println!(
      "The most calories carried is: {}",
      first
   );

   println!(
      "The total calories carried by the top three elves is: {}",
      first + second + third
   );
}
// The most calories carried by one elf is: 72240
// The total calories carried by the top three elves is: 210957
// 
// Finished in 2.00ms

```

Which is twice again faster, for an overall 10x speed up.

I've done a bit more tidying removing dead code, simplifying the naming e.g. `parse_input_to_calorie_totals` back to 
`parse_input` now there are no longer two versions to differentiate. 
