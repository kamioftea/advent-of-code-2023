//! This is my solution for [Advent of Code - Day 4: _Scratchcards_](https://adventofcode.com/2023/day/4)
//!
//! [`Scratchcard`] handles parsing ([`Scratchcard::from`]), and scoring ([`Scratchcard::match_count`] and
//! [`Scratchcard::score`]).
//!
//! Part 1 is solved by [`sum_scores`], part 2 by [`calculate_total_cards`].

use std::collections::HashSet;
use std::fs;

/// Represents a scratchcard (one line of input)
#[derive(Eq, PartialEq, Debug)]
struct Scratchcard {
    winning_numbers: HashSet<i32>,
    numbers_you_have: HashSet<i32>,
}

impl From<&str> for Scratchcard {
    /// Parse a string in the format `Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53`
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

impl Scratchcard {
    /// The number of numbers you have that match a winning number
    fn match_count(&self) -> usize {
        let matches = self
            .numbers_you_have
            .intersection(&self.winning_numbers)
            .count();

        matches
    }

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

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-4-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 4.
pub fn run() {
    let contents = fs::read_to_string("res/day-4-input.txt").expect("Failed to read file");

    let scratchcards = parse_input(&contents);

    println!(
        "The sum of scratchcard scores is: {}",
        sum_scores(&scratchcards)
    );
    println!(
        "The total number of scratchcards is: {}",
        calculate_total_cards(&scratchcards)
    );
}

/// Parse each line as a card
fn parse_input(input: &String) -> Vec<Scratchcard> {
    input.lines().map(Scratchcard::from).collect()
}

/// Part 1 solution - calculate and sum the scores for all cards
fn sum_scores(scratchcards: &Vec<Scratchcard>) -> i32 {
    scratchcards.iter().map(Scratchcard::score).sum()
}

/// Part 2 solution - each card wins a copy of the next n cards where n is the number of winning matches. This is
/// guaranteed not to overflow the list of available cards.
fn calculate_total_cards(scratchcards: &Vec<Scratchcard>) -> i32 {
    // At the start there is one of each card
    let mut counts: Vec<i32> = (0..scratchcards.len()).map(|_| 1).collect();

    for (current_card_index, scratchcard) in scratchcards.iter().enumerate() {
        // Each copy of the card (original + those added by previous loops) adds one card at each
        let copies_of_current_card = counts[current_card_index];

        for insert_offset in 1..=scratchcard.match_count() {
            counts[current_card_index + insert_offset] += copies_of_current_card
        }
    }

    counts.iter().sum()
}

#[cfg(test)]
mod tests {
    use crate::day_4::*;

    fn example_scratchcards() -> Vec<Scratchcard> {
        return vec![
            Scratchcard {
                winning_numbers: vec![41, 48, 83, 86, 17].into_iter().collect(),
                numbers_you_have: vec![83, 86, 6, 31, 17, 9, 48, 53].into_iter().collect(),
            },
            Scratchcard {
                winning_numbers: vec![13, 32, 20, 16, 61].into_iter().collect(),
                numbers_you_have: vec![61, 30, 68, 82, 17, 32, 24, 19].into_iter().collect(),
            },
            Scratchcard {
                winning_numbers: vec![1, 21, 53, 59, 44].into_iter().collect(),
                numbers_you_have: vec![69, 82, 63, 72, 16, 21, 14, 1].into_iter().collect(),
            },
            Scratchcard {
                winning_numbers: vec![41, 92, 73, 84, 69].into_iter().collect(),
                numbers_you_have: vec![59, 84, 76, 51, 58, 5, 54, 83].into_iter().collect(),
            },
            Scratchcard {
                winning_numbers: vec![87, 83, 26, 28, 32].into_iter().collect(),
                numbers_you_have: vec![88, 30, 70, 12, 93, 22, 82, 36].into_iter().collect(),
            },
            Scratchcard {
                winning_numbers: vec![31, 18, 13, 56, 72].into_iter().collect(),
                numbers_you_have: vec![74, 77, 10, 23, 35, 67, 36, 11].into_iter().collect(),
            },
        ];
    }

    #[test]
    fn can_parse_input() {
        let input = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            .to_string();

        assert_eq!(parse_input(&input), example_scratchcards())
    }

    #[test]
    fn can_score_cards() {
        let expected_scores = vec![8, 2, 2, 1, 0, 0];

        for (i, card) in example_scratchcards().iter().enumerate() {
            assert_eq!(
                card.score(),
                expected_scores[i],
                "Card {:?} should score {}",
                card,
                expected_scores[i]
            )
        }
    }

    #[test]
    fn can_sum_scores() {
        assert_eq!(sum_scores(&example_scratchcards()), 13)
    }

    #[test]
    fn can_calculate_total_cards() {
        assert_eq!(calculate_total_cards(&example_scratchcards()), 30)
    }
}
