---
day: 6
tags: [ post ]
header: 'Day 6: Wait For It'
---

Today we're in a convoluted boat race. I'm glad to say that I noticed the maths to solve from the start rather than
implementing a naive solution that blows up in step two.

If the race lasts for a duration of `d` seconds, and you hold the button for `n` seconds it will travel at speed `n`
for `d - n` seconds. If the current record is `r` then for each `n` where $num\_seconds * (duration - num\_seconds) >
current\_record$, the boat will travel further than the record. To get the bounds we need the two points where $n *
(d - n) = r$. Rearranging this:

$$n * (d - n) = r$$

$$nd - n^2 = r$$

$$0 = n^2 - dn + r$$

Which is a quadratic equation in the form $ax^2 + bx + c$, where a = 1, b = -d, and c = r. The roots of
this can be obtained using the [quadratic formula](https://en.wikipedia.org/wiki/Quadratic_formula):

$$x = {-b \pm \sqrt{b^2-4ac} \over 2a}$$

Substituting the specific values gives:

$$n = {d \pm \sqrt{d^2-4r} \over 2}$$

Which we can use to get the lower and upper bounds of the number of seconds required to hold the button to exceed
the record.

## Parsing

Parsing the input was refreshingly simple today. The only slight bump was pairing up the numbers from each line, for
which there is `zip`

```rust
#[derive(Eq, PartialEq, Debug)]
struct Race {
    duration: i64,
    distance_record: i64,
}

impl Race {
    fn new(time: i64, distance_record: i64) -> Race {
        Race {
            duration: time,
            distance_record,
        }
    }
}

fn parse_input(input: &String) -> Vec<Race> {
    let mut lines = input.split("\n");
    parse_line(&lines.next().unwrap())
        .iter()
        .zip(parse_line(&lines.next().unwrap()))
        .map(|(&t, d)| Race::new(t, d))
        .collect()
}

fn parse_line(line: &str) -> Vec<i64> {
    line.split(" ")
        .filter_map(|word| word.parse().ok())
        .collect()
}
```

## Part 1 - Off to the races

Tests can be generated from the examples:

```rust
#[test]
    fn can_find_count_of_winning_hold_times() {
        assert_eq!(find_count_of_winning_hold_times(&Race::new(7, 9)), 4);
        assert_eq!(find_count_of_winning_hold_times(&Race::new(15, 40)), 8);
        assert_eq!(find_count_of_winning_hold_times(&Race::new(30, 200)), 9);
    }

    #[test]
    fn can_find_product_of_winning_hold_times() {
        assert_eq!(find_product_of_races(&parse_input(&example_input())), 288);
    }
```

Implementing part one only requires using the `+` and `-` variants of the quadratic formula to solve. The square
root and division will need the numbers to be manipulated as floats.

There was an out by one error to resolve here. For races where the root is not an exact integer the valid values are the
next integer higher than the lower bound (`lower_bound::ceil()`) to the previous integer below the upper
bound (`upper_bound::floor()`) __inclusive__. If the range is calculated as `upper_bound` - `lower_bound` the
upper_bound needs to be exclusive, which can be done by using `upper_bound::ceil()`. The issue then arises where there
is an exact integer, which was the case for the third race. `lower_bound::ceil()` remains unchanged, but should be
excluded because using that would equal the current record rather than exceed it. This can be fixed by
using `lower_bound::floor() + 1`, i.e. always round down to the highest excluded integer, then add one giving the
lowest, included integer.
That gives:

```rust
fn find_count_of_winning_hold_times(race: &Race) -> i64 {
    let duration = race.duration as f64;
    let record = race.distance_record as f64;

    let root_a = (duration + (duration.powf(2.0) - 4.0 * record).sqrt()) / 2.0;
    let root_b = (duration - (duration.powf(2.0) - 4.0 * record).sqrt()) / 2.0;

    // Inclusive. Floor here rounds down the the last excluded integer
    // before the range, so add one to get the inclusive
    let lower_bound = root_a.min(root_b).floor() as i64 + 1;
    // Exclusive. Ceil always gives the next integer beyond the range
    let upper_bound = root_a.max(root_b).ceil() as i64;

    upper_bound - lower_bound
}

fn find_product_of_races(races: &Vec<>) -> i64 {
    races
        .iter()
        .map(find_count_of_winning_hold_times)
        .product()
}
```

## Part 2: One last big push

With part one implemented efficiently already, part two needed to parse the output differently, and it was pretty 
much solved.

I ended up factoring out how the lines were parsed, and passing the content into the functions that solved each part, so
they control which parser is used.

The signature for parse_input becomes:

```rust
fn parse_input(input: &String, line_parser: fn(&str) -> Vec<i64>) -> Vec<Race> {
    /* ... */
}
```

Then I need to update tests to match, and add one for part 2, then implement a parser that combines all the digits.

```rust
#[test]
fn can_find_hold_times_for_combined_race() {
    assert_eq!(
        find_product_of_races(&parse_input(&example_input(), part_2_line_parser)),
        71503
    );
}

fn part_2_line_parser(line: &str) -> Vec<i64> {
    let num = line
        .chars()
        .filter_map(|chr| chr.to_digit(10))
        .fold(0i64, |acc, digit| acc * 10 + digit as i64);

    return vec![num];
}
```

That passes and produces the puzzle answer. The code is littered with debug statements as a legacy from, getting the 
bound calculations correct. I do a pass to tidy that up and make sure everything is named sensibly, but no other 
refactoring seems necessary.

## Final thoughts

I'm quite proud I identified the mathsy solution to this one from the get go. I haven't really had to use that 
formula since A-levels, so it was a bit rusty. Other colleagues doing advent of code have commented that the difficulty 
curve seems to be higher for odd days, and I think I agree, we'll see if that holds up tomorrow.
