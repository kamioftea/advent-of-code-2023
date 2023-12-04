---
day: 4
tags: [ post ]
header: 'Day 4: Scratchcards'
---

## Parsing

I tried out implementing the parsing as implementing the `From<&str>` trait, I'm not sure that this added much value
in this case. The tests were generated from the puzzle examples. The prefix, e.g. `Card 1:` can be ignored by using
`filter_map` to discard any words that don't parse to a valid number. With that plan, parsing is splitting the line
into two parts on the `|`, then parsing each half into a `HashSet` in the same way.

```rust
/// Represents a scratchcard (one line of input)
#[derive(Eq, PartialEq, Debug)]
struct Scratchcard {
    winning_numbers: HashSet<i32>,
    numbers_you_have: HashSet<i32>,
}

impl From<&str> for Scratchcard {
    /// Parse a string in the format 
    /// `Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53`
    fn from(value: &str) -> Self {
        fn parse_set(numbers: &str) -> HashSet<i32> {
            numbers
                .split(" ")
                .filter_map(|num| num.parse().ok())
                .collect()
        }

        let (winning_str, your_str) = value
            .split_once(" | ")
            .expect(format!("Invalid scratchcard {}", value).as_str());

        Scratchcard {
            winning_numbers: parse_set(winning_str),
            numbers_you_have: parse_set(your_str),
        }
    }
}
```

## Part 1 - Points mean prizes

Solving the puzzle now requires scoring each card based on the number of matches between the winning numbers and the
card's numbers. Starting at 1 if there's a match, and doubling for each match beyond the first. There are scores for
each of the example cards, and the total score (which will be the puzzle input), so these can be added as tests.

Finding the number of matches can be done by counting the intersection of the two `HashSet`s. Doubling numbers can be
done by left shifting, but the case where there are no matches needs to be specially handled. The code ends up being
quite technical, but I think it's ok with a note about what the scoring criteria are.

```rust
impl Scratchcard {
    fn score(&self) -> i32 {
        let matches = self
            .numbers_you_have
            .intersection(&self.winning_numbers)
            .count();

        matches.checked_sub(1).map(|power| 1 << power).unwrap_or(0)
    }
}
```

Summing the scores is then a `map` and `sum` over the list of cards.

```rust
/// Part 1 solution - calculate and sum the scores for all cards
fn sum_scores(scratchcards: &Vec<Scratchcard>) -> i32 {
    scratchcards.iter().map(Scratchcard::score).sum()
}
```

This passes the tests and solves part one.

## Part 2 - You get a card, and you get a card...

Instead of scoring points, scratchcards now get you more scratchcards from further down the list. Specifically you
add one card duplicating each of the next `match_count` scratchcards down the list. Doing this is a case of
simulating the scoring, keeping a mutable list of the number of each card, and using a nested for loop to step through
the list of `Scratchcards` (the outer loop) and to add cards to subsequent indices for each match (the inner loop). I
split `Scratchcard::match_count` out of `Scratchcard::score` into its own function to use to get the range of each inner
loop.

The list should be initialised with one card in each index to represent the starting point. Once these have completed
summing the final list of card counts gives the total.

```rust
fn calculate_total_cards(scratchcards: &Vec<Scratchcard>) -> i32 {
    let mut counts: Vec<i32> = (0..scratchcards.len()).map(|_| 1).collect();

    for (card_index, scratchcard) in scratchcards.iter().enumerate() {
        for insert_index in 1..=scratchcard.match_count() {
            counts[card_index + insert_index] += counts[card_index]
        }
    }

    counts.iter().sum()
}
```

This passes tests and gives the correct answer for the puzzle input.

## Refactoring

The code is already fairly well laid out. There are some islands of complexity that could do with some comments /
better structure to properly communicate intent:

* I explain the one-liner that is doing the doubling in `Scratchcard::score`.
* I extract a variable and tweak the names in `calculate_total_cards`.

```rust
impl Scratchcard {
    /// The first match scores one, each subsequent match doubles the score
    fn score(&self) -> i32 {
        let matches = self.match_count();

        // Left shift needs to start from one, then each shift doubles the number.
        // This means the first match should start at 1 and shift it 0 times.
        // Handily the special case (0 matches scores 0 points) is the only case
        // that hits the `None` branch of `checked_sub`.
        matches.checked_sub(1).map(|power| 1 << power).unwrap_or(0)
    }
}

/// Part 2 solution - each card wins a copy of the next n cards where n is the number of winning matches. This is
/// guaranteed not to overflow the list of available cards.
fn calculate_total_cards(scratchcards: &Vec<Scratchcard>) -> i32 {
    // At the start there is one of each card
    let mut counts: Vec<i32> = (0..scratchcards.len()).map(|_| 1).collect();

    for (current_card_index, scratchcard) in scratchcards.iter().enumerate() {
        // Each copy of the card (original + those added by previous loops) adds one card at each step
        let copies_of_current_card = counts[current_card_index];

        for insert_offset in 1..=scratchcard.match_count() {
            counts[current_card_index + insert_offset] += copies_of_current_card
        }
    }

    counts.iter().sum()
}
```

## Final thoughts

The challenges so far have been a bit of a roller coaster in terms of effort, and today did not require much
code. I feel that it was slightly more technical than previous days. Apart from day one being a bit of an anomaly it
feels like this fits the expected curve: the skill required should roughly increase throughout the month, but effort
spikes at weekends.
