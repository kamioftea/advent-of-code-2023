//! This is my solution for [Advent of Code - Day 7: _Camel Cards_](https://adventofcode.com/2023/day/7)
//!
//!

use crate::day_7::HandType::*;
use itertools::Itertools;
use std::cmp::Ordering;
use std::fs;
use Card::*;

#[derive(Eq, PartialEq, Debug, Ord, PartialOrd, Hash, Copy, Clone)]
enum Card {
    Joker,
    Number(u32),
    Jack,
    Queen,
    King,
    Ace,
}

impl TryFrom<char> for Card {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Ace),
            'K' => Ok(King),
            'Q' => Ok(Queen),
            'J' => Ok(Jack),
            'T' => Ok(Number(10)),
            c => c.to_digit(10).map(|d| Number(d)).ok_or(()),
        }
    }
}

#[derive(Eq, PartialEq, Debug, Ord, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Eq, PartialEq, Debug)]
struct Hand {
    bid: i32,
    cards: Vec<Card>,
    hand_type: HandType,
}

impl Hand {
    fn new(bid: i32, cards: Vec<Card>, hand_type: HandType) -> Hand {
        Hand {
            bid,
            cards,
            hand_type,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand_type
            .cmp(&other.hand_type)
            .then(self.cards.cmp(&other.cards))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-7-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 7.
pub fn run() {
    let contents = fs::read_to_string("res/day-7-input.txt").expect("Failed to read file");

    println!(
        "The total winnings with Jacks are: {}",
        total_winnings(&parse_input(&contents, parse_cards_part_1))
    );

    println!(
        "The total winnings with Jokers are: {}",
        total_winnings(&parse_input(&contents, parse_cards_part_2))
    );
}

fn parse_input(input: &String, card_parser: fn(&str) -> Vec<Card>) -> Vec<Hand> {
    input
        .lines()
        .map(|line| parse_hand(line, card_parser))
        .collect()
}

fn parse_hand(line: &str, card_parser: fn(&str) -> Vec<Card>) -> Hand {
    let (card_spec, bid_spec) = line.split_once(" ").unwrap();
    let cards: Vec<Card> = card_parser(card_spec);
    let hand_type = calculate_hand_type(&cards);

    Hand::new(bid_spec.parse().unwrap(), cards, hand_type)
}

fn parse_cards_part_1(card_spec: &str) -> Vec<Card> {
    card_spec
        .chars()
        .filter_map(|c| c.try_into().ok())
        .collect()
}

fn parse_cards_part_2(card_spec: &str) -> Vec<Card> {
    parse_cards_part_1(card_spec)
        .iter()
        .map(|&c| if c == Jack { Joker } else { c })
        .collect()
}

fn calculate_hand_type(cards: &Vec<Card>) -> HandType {
    let groups = cards.iter().counts();
    let distinct_count = groups.len();
    let max_group = groups.values().max().unwrap();
    let joker_count = groups.get(&Joker).unwrap_or(&0);

    match (distinct_count, max_group, joker_count) {
        (1, _, _) => FiveOfAKind,
        (2, 4, 0) => FourOfAKind,
        (2, 4, _) => FiveOfAKind, // Only 2 values of card, joker(s) change to match the other cards
        (2, 3, 0) => FullHouse,
        (2, 3, _) => FiveOfAKind, // Only 2 values of card, jokers change to match the other cards
        (3, 3, 0) => ThreeOfAKind,
        (3, 3, _) => FourOfAKind, // Either three jokers pair up with a singleton, or singleton joker matches triple
        (3, 2, 0) => TwoPair,
        (3, 2, 1) => FullHouse,   // Singleton joker matches one of the pairs
        (3, 2, 2) => FourOfAKind, // Two jokers match the other pair
        (4, _, 0) => OnePair,
        (4, _, _) => ThreeOfAKind, // Two jokers match any of the singletons, singleton joker matches the pair
        (5, _, 0) => HighCard,     // Joker pairs up with any of the other values
        (5, _, _) => OnePair,
        _ => unreachable!(),
    }
}

fn total_winnings(hands: &Vec<Hand>) -> i32 {
    hands
        .iter()
        .sorted()
        .enumerate()
        .map(|(i, hand)| (i + 1) as i32 * hand.bid)
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day_7::*;
    use std::cmp::Ordering::*;

    fn example_hands() -> Vec<Hand> {
        vec![
            Hand::new(
                765,
                vec![Number(3), Number(2), Number(10), Number(3), King],
                OnePair,
            ),
            Hand::new(
                684,
                vec![Number(10), Number(5), Number(5), Jack, Number(5)],
                ThreeOfAKind,
            ),
            Hand::new(
                28,
                vec![King, King, Number(6), Number(7), Number(7)],
                TwoPair,
            ),
            Hand::new(220, vec![King, Number(10), Jack, Jack, Number(10)], TwoPair),
            Hand::new(483, vec![Queen, Queen, Queen, Jack, Ace], ThreeOfAKind),
        ]
    }

    #[test]
    fn can_parse_input() {
        let input = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"
            .to_string();

        assert_eq!(parse_input(&input, parse_cards_part_1), example_hands());
    }

    #[test]
    fn can_determine_hand_type() {
        assert_eq!(
            calculate_hand_type(&parse_cards_part_1("AAAAA")),
            FiveOfAKind
        );
        assert_eq!(
            calculate_hand_type(&parse_cards_part_1("AA8AA")),
            FourOfAKind
        );
        assert_eq!(calculate_hand_type(&parse_cards_part_1("23332")), FullHouse);
        assert_eq!(
            calculate_hand_type(&parse_cards_part_1("TTT98")),
            ThreeOfAKind
        );
        assert_eq!(calculate_hand_type(&parse_cards_part_1("23432")), TwoPair);
        assert_eq!(calculate_hand_type(&parse_cards_part_1("A23A4")), OnePair);
        assert_eq!(calculate_hand_type(&parse_cards_part_1("23456")), HighCard);
    }

    #[test]
    fn can_sort_hands() {
        let examples = vec![
            ("AAAAA 1", "AA8AA 1", Greater),
            ("A23A4 1", "AA8AA 1", Less),
            ("AA8AA 1", "AA8AA 1", Equal),
            ("32T3K 1", "T55J5 1", Less),
            ("32T3K 1", "KK677 1", Less),
            ("T55J5 1", "KK677 1", Greater),
            ("QQQJA 1", "T55J5 1", Greater),
            // bid is ignored
            ("AA8AA 1", "AA8AA 17", Equal),
            // Same hand type falls back to card by card comparison
            ("33332 1", "2AAAA 1", Greater),
            ("77888 1", "77788 1", Greater),
        ];

        for (str_a, str_b, expected) in examples {
            let hand_a = parse_hand(&str_a, parse_cards_part_1);
            let hand_b = parse_hand(&str_b, parse_cards_part_1);

            assert_eq!(
                hand_a.cmp(&hand_b),
                expected,
                "{:?} be {} {:?}",
                hand_a,
                if expected == Equal {
                    "equal to".to_string()
                } else {
                    format!("{:?} than", expected)
                },
                hand_b,
            );
        }
    }

    #[test]
    fn can_find_total_winnings() {
        assert_eq!(total_winnings(&example_hands()), 6440)
    }

    #[test]
    fn can_find_total_winnings_part_2() {
        let input = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"
            .to_string();

        let hands = parse_input(&input, parse_cards_part_2);

        assert_eq!(total_winnings(&hands), 5905)
    }
}
