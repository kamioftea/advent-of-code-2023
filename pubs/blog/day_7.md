---
day: 7
tags: [post]
header: 'Day 7: Camel Cards'
---

Today the challenge is to sort poker hands by a slightly unorthodox method. Since the `HandType` of 
a hand will need to be used repeatedly when sorting the list, it makes sense to pre-calculate it 
and store it with the other data about the hand. The solution should then be inherent in the 
data model.

## Parsing

First I'll define a data model. I'll use enums for the cards and hand ranks, as they have the 
useful property that `#[derive(Eq, PartialEq, PartialOrd, Ord)]` will use the order the enum 
appears in the code, so if I list them in rank order, low to high, I get the correct ordering 
foe free. The ordering defaults mean I can also define the 9 number cards as `Num(i32)` and it 
will assume ordering them `Num(2)` to `Num(10)` as required. 

```rust
#[derive(Eq, PartialEq, Debug, Ord, PartialOrd, Hash)]
enum Card {
    Num(u32),
    Jack,
    Queen,
    King,
    Ace,
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
```

Card ends up needing to be a key for a `HashMap` of card counts, so implements `Hash` as well.

For hand, I don't want to repeatedly calculate its `HandType`, so I'll add that to the data type 
and calculate it during parsing.

```rust
#[derive(Eq, PartialEq, Debug)]
struct Hand {
    bid: i32,
    cards: Vec<Card>,
    hand_type: HandType,
}
```

I'll also define a test that will cover the desired behaviour of the parser. This one test 
covers a lot of implementation details, but it should still be clear what is failing if it does.

```rust
fn example_hands() -> Vec<Hand> {
    vec![
        Hand::new(765, vec![Num(3), Num(2), Num(10), Num(3), King], OnePair),
        Hand::new(684, vec![Num(10), Num(5), Num(5), Jack, Num(5)], ThreeOfAKind),
        Hand::new(28 , vec![King, King, Num(6), Num(7), Num(7)], TwoPair),
        Hand::new(220, vec![King, Num(10), Jack, Jack, Num(10)], TwoPair),
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
```

```rust
/// Parse the puzzle input
fn parse_input(input: &String) -> Vec<Hand> {
    input
        .lines()
        .map(parse_hand)
        .collect()
}

fn parse_hand(line: &str) -> Hand {
    let (card_spec, bid_spec) = line.split_once(" ").unwrap();
    let cards: Vec<Card> = parse_cards(card_spec);
    let hand_type = calculate_hand_type(&cards);

    Hand::new(bid_spec.parse().unwrap(), cards, hand_type)
}
```

I will get to calculate_hand_type next, but for parsing cards I can implement `TryFrom<char>` 
for `Card`. Then `parse_cards` can use `filter_map` to get the vector needed.

```rust
impl TryFrom<char> for Card {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Ace),
            'K' => Ok(King),
            'Q' => Ok(Queen),
            'J' => Ok(Jack),
            'T' => Ok(Num(10)),
            c => c.to_digit(10).map(|d| Num(d)).ok_or(()),
        }
    }
}

fn parse_cards(cards_spec: &str) -> Vec<Card> {
    cards_spec
        .chars()
        .filter_map(|c| c.try_into().ok())
        .collect()
}
```

For `calculate_hand_type`, I can determine what it is from how many different card values there 
are, and how many of the most common value(s) there are. `Itertools::counts` will provide both 
of these metrics, and then I can match on the possible permutations.

```rust
fn calculate_hand_type(cards: &Vec<Card>) -> HandType {
    let groups = cards.iter().counts();
    let distinct_count = groups.len();
    let max_group = groups.values().max().unwrap();

    match (distinct_count, max_group) {
        (1, 5) => FiveOfAKind,
        (2, 4) => FourOfAKind,
        (2, 3) => FullHouse,
        (3, 3) => ThreeOfAKind,
        (3, 2) => TwoPair,
        (4, 2) => OnePair,
        (5, 1) => HighCard,
        _ => unreachable!(),
    }
}
```

## Part 1 - The house always wins

Getting the puzzle solution requires

* Sorting the hands by `HandType` then card-by-card in written order
* Multiplying their index in the sorted array (starting from 1)

Rust has some great defaults for orderings. The Enums work as expected if defined in order 
low-to-high, and `Vec` has a default sorting that compares items at index 0, then 1, etc, so 
a `Vec<Card>` also sorts as we would like by default.

I could reorder `Hand` to `hand_type`, `cards`, `bid` and it would work by default, as the assumed 
ordering for a `struct` is to sort by the first field in source order, then the second, and so on.
The specification doesn't include sorting by bid, and I'd like to stick to that. This means I 
need to manually define the ordering for a hand. In rust this means implementing `Ord` for `Hand`, 
which also requires an implementation for `PartialOrd`, but that can be implemented in terms of 
`Ord`.

```rust
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
```

With that I can reduce the input to the solution with standard iterable functions (and `sorted` 
from `Itertools`). The only awkwardness is that `Iterator::enumerate` starts at `0`.

```rust
fn total_winnings(hands: &Vec<Hand>) -> i32 {
    hands
        .iter()
        .sorted()
        .enumerate()
        .map(|(i, hand)| (i + 1) as i32 * hand.bid)
        .sum()
}
```

## Part 2 - Stuck in the middle with you

Part two replaces jacks with jokers, which:

* Can count as any card when determining the `HandType`
* Are ranked lower than non-joker cards when comparing card-by-card.

This requires a few changes:

* Make the parsing of cards configurable, with a dedicated parser for each type,
* Add `Joker` to `Card`. I can put this first in the `enum` to give it the lowest rank,
* Update `calculate_hand_type` to account for jokers.

I add a `fn (&str) -> Vec<Card>` to `parse_input` and `parse_hand`. I can then implement the part 
two parser by mapping any jacks to jokers:

```rust
fn parse_cards_part_2(cards_spec: &str) -> Vec<Card> {
    parse_cards_part_1(cards_spec)
        .into_iter()
        .map(|c| if c == Jack { Joker } else { c })
        .collect()
}
```

After a bit of pondering, I decide it's best to add number of jokers to the metrics used to 
identify the card types, and list out the possibilities as the number of variants doesn't 
increase too much. It is a bit harder to follow though, so I add some notes too.

```rust
fn calculate_hand_type(cards: &Vec<Card>) -> HandType {
    let groups = cards.iter().counts();
    let distinct_count = groups.len();
    let max_group = groups.values().max().unwrap();
    let joker_count = groups.get(&Joker).unwrap_or(&0);

    match (distinct_count, max_group, joker_count) {
        (1, 5, _) => FiveOfAKind,  //
        (2, 4, 0) => FourOfAKind,  //
        (2, 4, _) => FiveOfAKind,  // Only 2 values, joker(s) change to match the 
                                   // other card(s)
        (2, 3, 0) => FullHouse,    //
        (2, 3, _) => FiveOfAKind,  // Only 2 values, jokers change to match the 
                                   // other cards
        (3, 3, 0) => ThreeOfAKind, //
        (3, 3, _) => FourOfAKind,  // Three jokers match a singleton, or single 
                                   // joker matches the triple
        (3, 2, 0) => TwoPair,      //
        (3, 2, 1) => FullHouse,    // Singleton joker matches one of the pairs
        (3, 2, 2) => FourOfAKind,  // Two jokers match the other pair
        (4, 2, 0) => OnePair,      //
        (4, 2, _) => ThreeOfAKind, // Two jokers match one of the singletons, 
                                   // a single joker matches the pair
        (5, 1, 0) => HighCard,     //
        (5, 1, 1) => OnePair,      // Joker pairs up with any one of the other values
        _ => unreachable!(),
    }
}
```

It turns out `rustfmt` will line up trailing comments, but only for blocks where every line has 
a trailing comment.

The solution now works for both parts, with the only difference being in the card parsing. There's
no major refactoring to do as the data model works well for both parts.

## Final thoughts

Today was a satisfying puzzle. Rust's sensible ordering defaults and pattern matching syntax 
made it natural to write the logic in an expressive, step-by-step way. Once the data model was 
defined, the solution almost wrote itself.
