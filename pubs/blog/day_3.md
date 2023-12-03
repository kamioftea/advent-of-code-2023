---
day: 3
tags: [post]
header: 'Day 3: Gear Ratios'
---

Today was the first 2D challenge of the year. There was an interesting parsing challenge as numbers span multiple 
cells, and part two had a multistep data pipeline to transform the data into the required structure.

## Parsing

I know at this point that I need to be able to link all the points that surround digits to the number those digits 
represent, and search those points for if they have a symbol. Once I've got the number parsed, I can go back to the 
length with `ilog10()`, so I have all the information to calculate surroundings with the number and the co-ordinates 
where the digits started. 

```rust
#[derive(Eq, PartialEq, Debug)]
struct PartNumber {
    number: u32,
    x: u32,
    y: u32,
}

impl PartNumber {
    fn new(number: u32, x: u32, y: u32) -> PartNumber {
        PartNumber { number, x, y }
    }
}
```

The symbols are only ever one character, and the solution I have in mind needs to look up if a symbol is at a point. 
I could instead use a set of points here, but I suspect I'll need to differentiate them in part two, and it's not extra 
effort to use a `HashMap` instead to retain that information. I'll add some type aliases to codify the domain language.

```rust
type Point = (u32, u32);

type SymbolLookup = HashMap<Point, char>;
```

I turn the examples into tests. These are quite verbose as it's awkward to test for equality without caring about 
the ordering. I'll dig into this later during refactoring. The interesting bit here is the parsing.

I opt to step through each line then each character, which when enumerated gives the `y` and `x` co-ordinates of the 
current character. Given a number can span multiple digits, that needs to be built as parsed, then added to the 
state on the first non-digit. I actually made a mistake here and noticed during refactoring that line breaks didn't 
properly break numbers. Fortunately there wasn't a pair of numbers at the end and start of consecutive lines in my 
test input.

```rust
fn parse_grid(input: &String) -> (Vec<PartNumber>, HashMap<Point, char>) {
    // Setup output variables to populate during parsing
    let mut parts = Vec::new();
    let mut symbols = HashMap::new();
    // Holds an in progress PartNumber whist its digits are being parsed
    let mut num: u32 = 0;
    let mut num_origin: Option<Point> = None;

    for (y, line) in input.lines().enumerate() {
        for (x, chr) in line.chars().enumerate() {
            // We only know we've completed a part number when we next see a non-digit character. Check for that here
            // and emit the `PartNumber`.
            if !chr.is_digit(10) {
                if let Some((x, y)) = num_origin {
                    parts.push(PartNumber::new(num, x, y))
                }
                num = 0;
                num_origin = None;
            }

            match chr {
                // Represents a blank space
                '.' => {}
                // For PartNumbers build the number digit by digit, recording the origin on the first digit seen
                c if c.is_digit(10) => {
                    num_origin = num_origin.or(Some((x, y)));
                    num = num * 10 + chr.to_digit(10).expect("Tested with is_digit");
                }
                // Anything else is an arbitrary part symbol
                _ => {
                    symbols.insert((x, y), chr);
                }
            }
        }
    }

    // push any in progress number at the end - this should be at the end of each line as noted
    if let Some((x, y)) = num_origin {
        parts.push(PartNumber::new(num, x, y))
    }

    (parts, symbols)
}
```

## Part 1 - All around my ~Hat~ PartNumber

The plan for part one is to iterate through the parsed numbers, filter out numbers that aren't adjacent to symbols, 
then reduce them to the solution with `sum()`. To implement that filter I first need to be able to find all the 
adjacent cells, so I'll set up a test for that.

```rust
#[test]
fn can_find_adjacent_points() {
    #[rustfmt::skip] // Positional coordinates
    let examples = vec![
        (PartNumber::new(99, 0, 0), vec![
                            (2, 0),
            (0, 1), (1, 1), (2, 1)
        ]),
        (PartNumber::new(1, 1, 1), vec![
            (0, 0), (1, 0), (2, 0),
            (0, 1)        , (2, 1),
            (0, 2), (1, 2), (2, 2),
        ]),
    ];

    for (part_number, expected_points) in examples {
        let actual_points = get_adjacent_points(&part_number);
        
        assert_eq!(
            actual_points.len(),
            expected_points.len(),
            "Points lists were not the same length.\nExpected: {:?}\nActual  : {:?}",
            expected_points,
            actual_points
        );
        
        for expected_point in expected_points {
            assert!(
                actual_points.contains(&expected_point),
                "{:?} is not in the list of points",
                expected_point
            )
        }
    }
}
```

The awkwardness of using `Vec`s that don't care about the ordering is here again.

The implementation needs to:
* Determine the length of the number 
* Include all the adjacent numbers on the row above (if it's in the grid)
* Include the cell before (if in the grid) and after the number on its row
* Include all the adjacent numbers on the row below

I don't need to care about grid overflow, because those points will not exist in the `SymbolLookup`, so will be 
filtered out later. Due to using unsigned co-ordinates, I do need to care about underflow. I could convert all the grid 
co-ords to i32s here, but I decide to code around underflow as this complexity is contained here. I again use a 
mutable `Vec` to collect points as they're determined, and the code falls out from the bullet points above.

```rust
fn get_adjacent_points(part_number: &PartNumber) -> Vec<Point> {
    let mut points = Vec::new();
    let length = part_number.number.ilog10() as usize + 1;
    let start = part_number.x.checked_sub(1).unwrap_or(0);
    let end = part_number.x + length;

    for x in start..=end {
        if part_number.y > 0 {
            points.push((x, part_number.y - 1))
        }

        if x < part_number.x || x >= end {
            points.push((x, part_number.y))
        }

        points.push((x, part_number.y + 1))
    }

    points
}
```

I add tests from the puzzle description for which of the example numbers are valid, and what their sum should be. 
The implementations for these can be composed from built-in functions, and what I've already written. 

```rust
fn sum_valid_part_numbers(part_numbers: &Vec<PartNumber>, symbol_lookup: &SymbolLookup) -> u32 {
    part_numbers
        .iter()
        .filter(|&part_number| has_adjacent_symbol(part_number, symbol_lookup))
        .map(|part_number| part_number.number)
        .sum()
}

fn has_adjacent_symbol(part_number: &PartNumber, symbol_lookup: &SymbolLookup) -> bool {
    return get_adjacent_points(part_number)
        .iter()
        .any(|point| symbol_lookup.contains_key(point));
}
```

This passes the test, and solves part one when run for today's puzzle input.

## Part 2 - Grinding some gears

Now I have to find all the gears (symbolised by `*`) that have exactly two part numbers - as these numbers double up 
as a "gear ratio". I think about implementing the equivalent of `has_adjacent_symbol` to loop through each `*` 
symbol and find adjacent numbers. I reject that idea because numbers have variable length that will require repeatedly 
calculating which cells a number occupied, or building a cache for that, which would be a bit messy. Using what I've 
already got I can go the other way. 

* Find all the points adjacent to a number is already implemented. If I use that to list all the pairs of numbers 
  with points that adjacent to that number,
* Look up the symbol for each of those points, and filter to only those matching a `*`,
* Group those up by the point co-ordinates,
* Filter to where the group is exactly two numbers.

With a vague plan, I need to write a test that will verify the list of gears. Since the order of the numbers is 
irrelevant I need to account for them being either order in my test cases. Rather than make the test logic 
convoluted, I decide to implement a type for a `Gear` with custom equality,

```rust
#[derive(Eq, Debug)]
struct Gear {
    part_1: u32,
    part_2: u32,
}

impl Gear {
    fn new(part_1: u32, part_2: u32) -> Gear {
        Gear { part_1, part_2 }
    }
}

impl PartialEq for Gear {
    fn eq(&self, other: &Self) -> bool {
        (self.part_1 == other.part_1 && self.part_2 == other.part_2)
            || (self.part_1 == other.part_2 && self.part_2 == other.part_1)
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}
```

I can then write a test:

```rust
#[test]
fn can_find_gears() {
    let expected_gears = vec![Gear::new(467, 35), Gear::new(755, 598)];

    assert_eq!(
        find_gears(&example_part_numbers(), &example_symbol_lookup()),
        expected_gears
    )
}
```

The collections API lets me step through the plan in code (if I include `group_by` from Itertools). It takes a bit 
to align all the types correctly, but I end up with this:

```rust
fn find_gears(part_numbers: &Vec<PartNumber>, symbol_lookup: &SymbolLookup) -> Vec<Gear> {
    part_numbers
        .iter()
        // Explode the numbers into pairs of (number, adjacent_point)
        .flat_map(|part_number| {
            get_adjacent_points(part_number)
                .into_iter()
                .map(|point| (part_number.number, point))
                .collect::<Vec<(u32, Point)>>()
        })
        // Filter out any that don't match a `*` symbol
        .filter(|(_, point)| {
            symbol_lookup
                .get(point)
                .filter(|&symbol| *symbol == '*')
                .is_some()
        })
        // Each `*` appears once for each number it appears next to - group by the point
        .group_by(|(_, point)| point.clone())
        .into_iter()
        // Now that the grouping is done only the list of numbers is useful
        .map(|(_, group)| group.map(|(part, _)| part).collect::<Vec<u32>>())
        // 
        .filter(|parts| parts.len() == 2)
        .map(|parts| Gear::new(parts[0], parts[1]))
        .collect()
}
```

This passes the tests, but fails for being too low when used on actual input. Some debugging later and I find out it 
is because `group_by` only groups up keys if they appear consecutively, creating new groups if they're split. The 
quick and dirty fix is to sort by the co-ordinates so that each only appears in one group. Not ideal, but I can do 
better in a bit, I'll get it working first.

```rust
fn find_gears(part_numbers: &Vec<PartNumber>, symbol_lookup: &SymbolLookup) -> Vec<Gear> {
  part_numbers
      .iter()
      // ...
      .sorted_by(|(_, a), (_, b)| a.cmp(b))
      .group_by(|(_, point)| point.clone())
      // ...
      .collect();
}
```

This works and the timer for the day is stopped. I have some tidying up still to do.

## Refactoring

First I should have added a failing test that catches the `group_by` bug. I'll add it now so that I can be sure I 
don't accidentally break that.

```rust
#[test]
    fn can_find_gears_with_shared_part_number() {
        let example_grid = "\
1...3
.*.*.
..4.."
            .to_string();

        let (part_numbers, symbol_lookup) = parse_grid(&example_grid);
        let expected_gears = vec![Gear::new(1, 4), Gear::new(3, 4)];

        assert_eq!(find_gears(&part_numbers, &symbol_lookup), expected_gears)
    }
```

There are then two things I want to fix about `find_gears` itself. The `sorted_by` and `group_by` is a little 
awkward, and sorting is expensive when I can add the points to a HashMap in one pass. Second whilst it is satisfying 
to turn the plan directly into code, it'll be harder to parse it the other way round when reading this later, so I 
need to break it up a bit to make it more readable.

These are the refactorings I do:

* Extract the lambdas for exploding the PartNumbers into (number, point) pairs and filtering to symbol points into 
  named functions.
* Break up the chain, storing the list of pairs in a named variable.
* Swap out the `sorted_by` and `group_by` for a for loop over that iterable, building a mutable `HashMap` as I go
* This also lets me remove the `map` that turns each group into only the number, as I can build it in the required 
  format.

```rust
fn find_gears(part_numbers: &Vec<PartNumber>, symbol_lookup: &SymbolLookup) -> Vec<Gear> {
  // Since PartNumbers can have variable length it is easier to start with all the points adjacent to part numbers
  // and then filter to part number / `*` point pairs` ...
  let part_nums_adjacent_to_gear_points = part_numbers
      .iter()
      .flat_map(explode_adjacent_points)
      .filter(|(_, point)| is_point_a_gear_symbol(point, symbol_lookup));

  // ... Then invert the relationship by grouping the numbers by the `*` they are adjacent to
  let mut part_numbers_per_gear_point: HashMap<Point, Vec<u32>> = HashMap::new();
  for (part_number, point) in part_nums_adjacent_to_gear_points {
    part_numbers_per_gear_point
        .entry(point)
        .or_insert(Vec::new())
        .push(part_number)
  }

  // Any that have the required two numbers are the `Gear`s to return
  part_numbers_per_gear_point
      .values()
      .filter(|parts| parts.len() == 2)
      .map(|parts| Gear::new(parts[0], parts[1]))
      .collect()
}

/// Turn a PartNumber into a list of pairs of the (bare number, point) for each point it is adjacent to
fn explode_adjacent_points(part_number: &PartNumber) -> Vec<(u32, Point)> {
  get_adjacent_points(part_number)
      .into_iter()
      .map(|point| (part_number.number, point))
      .collect::<Vec<(u32, Point)>>()
}

/// Returns true if a given 2D co-ordinate maps to a `*` symbol
fn is_point_a_gear_symbol(point: &Point, symbol_lookup: &SymbolLookup) -> bool {
  symbol_lookup
      .get(point)
      .filter(|&symbol| *symbol == '*')
      .is_some()
}
```

This passes locally, but fails in the GitHub CI pipeline. Turns out I'd not done the trick to ignore ordering when 
comparing the resulting `Vec<Gear>` with the example result. Up to this point I had been testing:

* Are the two collections the same length?
* Does each expected item appear in the actual collection being verified?

This was common enough I decided to factor it out into a test helper I could reuse for both today's tests and any 
similar scenarios in future days. This involves a bit of type wizardry to make it work on both the various `Vec`s 
used and the `HashMap` used for the symbol lookup.

```rust
#[cfg(test)]
pub(crate) mod test {
    use std::fmt::Debug;

    pub(crate) fn assert_contains_in_any_order<T>(
        actual: impl IntoIterator<Item = T>,
        expected: impl IntoIterator<Item = T>,
    ) where
        T: Debug + Eq,
    {
        let actual_vec: Vec<T> = actual.into_iter().collect();
        let expected_vec: Vec<T> = expected.into_iter().collect();
        assert_eq!(
            actual_vec.len(),
            expected_vec.len(),
            "The actual length of the does not match the expected length"
        );

        for expected_value in expected_vec {
            assert!(
                actual_vec.contains(&expected_value),
                "{:?} was not found",
                expected_value,
            );
        }
    }
}
```

I can then rewrite `can_find_gears_with_shared_part_number` as:

```rust
#[test]
fn can_find_gears_with_shared_part_number() {
  // ...
  assert_contains_in_any_order(find_gears(&part_numbers, &symbol_lookup), expected_gears)
}
```

...and make similar refactorings in the other tests that use that same pattern.

## Final thoughts

Today's puzzle was very satisfying to solve, even with the `group_by` debugging. It took a while to get it working, 
and then longer to get it into a state I'm happy with, but that came with exercising some Rust tools I'm out of 
practice with. I also have a useful helper function I expect to reuse throughout the month.
