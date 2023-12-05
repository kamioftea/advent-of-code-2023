---
day: 5
tags: [post]
header: 'Day 5: If You Give A Seed A Fertilizer'
---

Today's puzzle is tracking some ids as they go through a sequence of transformations. There is a lot of parsing work 
to do to get the data into useful structure, and then the naive implementation applies the mappings as written.

## Data model

There's not much novel in the parsing, so I will instead explain the resulting data model.

The entries of the almanac are all mapping between a source and destination `Category`. Technically I don't 
need to track these, but doing so ties the code back to what it represents, so I would like to include these in the data 
model.
```rust
#[derive(Eq, PartialEq, Debug, Hash, Copy, Clone)]
enum Category {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}
```

An `Id` is a number linked to one of the categories.
```rust
#[derive(Eq, PartialEq, Debug)]
struct Id {
    category: Category,
    value: i32,
}
```

A `Range` is a contiguous range of ids with in a section of the almanac, represented as `start`, the lower bound of the 
range, and its `length`, along with the `delta` it should use to modify the ids that fall within the range.
```rust
#[derive(Eq, PartialEq, Debug)]
struct Range {
    start: i32,
    length: i32,
    delta: i32,
}
```

A section of the Almanac that represents the complete transformation from ids of one `Category` to another, using 
the source and destination categories and each range of ids that should be modified when transforming. Any ids not 
included in those ranges should keep the same value when mapped.
```rust
#[derive(Eq, PartialEq, Debug)]
struct AlmanacMap {
    source: Category,
    destination: Category,
    ranges: Vec<Range>,
}
```

I convert the example spec into this representation, and use that as a test.  The parsing splits the inputs on each 
blank line, parses the seed numbers into `Id`s in the `Seed` category, and builds a `HashMap<Category, AlmanacMap>`, 
from the sections. Notably the ranges within each `AlmanacMap` are 

## Part one - the seed bone's connected to the soil bone, ...

An `Id` can be advanced to the next category by
- Finding the section of the almanac that has a source `Category` matching the id
- Loop through the defined ranges in sorted order;
    - If the id is before that range then no mapping applies,
    - If the id is in the range apply the delta,
    - Otherwise, advance to the next one...
- If the source id is above the final range, no mapping applies.

```rust
fn progress_id(id: &Id, almanac: &HashMap<Category, AlmanacMap>) -> Id {
    let mapper = almanac.get(&id.category).unwrap();
    for range in &mapper.ranges {
        if id.value < range.start {
            return Id::new(mapper.destination, id.value);
        }
        if id.value < range.start + range.length {
            return Id::new(mapper.destination, id.value + range.delta);
        }
    }

    Id::new(mapper.destination, id.value)
}
```

Extending that to map the `Seed` ids to `Location` ids, can be done by applying that same mapping recursively until 
it is in the right category.

```rust
fn progress_id_to(
    id: Id, 
    category: Category,
    almanac: &HashMap<Category, AlmanacMap>
) -> Id {
    return if id.category == category {
        id
    } else {
        progress_id_to(progress_id(&id, almanac), category, almanac)
    };
}
```

The puzzle solution is then mapping the whole list of seeds, and finding the minimum.

```rust
fn find_nearest_location(
    seeds: &Vec<Id>, 
    almanac: &HashMap<Category, AlmanacMap>
) -> i64 {
    seeds
        .iter()
        .map(|seed| progress_id_to(seed.clone(), Location, almanac).value)
        .min()
        .unwrap_or(0)
}
```

This passes the tests. It fails with an integer overflow on the real data. A find and replace to use `i64`s instead of 
`i32`s throughout, and it runs and calculates the correct answer for part one.

## Part 2 - Too many seeds

The twist is that the seeds are not a list of ids, but pairs representing the start and length of ranges of seed ids.
I do try to run the naive solution by transforming them into the list of seeds they represent and running the part 
one solution, but the numbers involved are large enough it won't complete in a short enough time.

Since the mapping is done in continuous ranges I can store the ids in ranges, transform each subset in a range 
that has the same mapping to a smaller range in the next category, and so on. 

```rust
#[derive(Eq, PartialEq, Debug, Clone)]
struct IdRange {
    category: Category,
    start: i64,
    length: i64,
}

fn ids_to_ranges(ids: &Vec<Id>) -> Vec<IdRange> {
    ids.into_iter()
       .tuples()
       .map(|(start_id, length_id)| {
           IdRange::new(start_id.category, start_id.value, length_id.value)
       })
       .collect()
}
```

The mapping process follows a similar structure to before. Starting with the lower bound of the input `IdRange` loop 
through the almanac ranges until the mapping or gap that applies to that is. If the end of that gap or range is 
after the input `IdRange` ends then the whole range is mapped in one step. Otherwise, create the subrange and map 
that, then continue the same process from with the lower bound now being the start of the next range or gap. 

```rust
fn progress_id_range(
    id_range: &IdRange, 
    almanac: &HashMap<Category, AlmanacMap>
) -> Vec<IdRange> {
    let mut new_id_ranges = Vec::new();
    let mut current = id_range.start;
    let end = id_range.start + id_range.length;

    let mapper = almanac.get(&id_range.category).unwrap();

    // ranges are already sorted, so walk through them adding mapped ranges where 
    // they overlap
    for mapper_range in &mapper.ranges {
        // If there is overlap in the gap before the next id starts add an output 
        // range with unchanged ids
        if mapper_range.start > current {
            let id_range_end = mapper_range.start.min(end);
            new_id_ranges.push(IdRange::new(
                mapper.destination.clone(),
                current,
                id_range_end - current,
            ));

            current = mapper_range.start;
        }

        // Short circuit if we've reached the end of the input range
        if (current >= end) {
            break;
        }

        // if there is an overlap with the range, add the overlap to the output
        if mapper_range.start + mapper_range.length > current {
            let id_range_end = (mapper_range.start + mapper_range.length).min(end);
            new_id_ranges.push(IdRange::new(
                mapper.destination.clone(),
                current + mapper_range.delta,
                id_range_end - current,
            ));

            current = mapper_range.start + mapper_range.length;
        }

        // Short circuit if we've reached the end of the input range
        if current >= end {
            break;
        }
    }

    // If the current range extends beyond the mappings in the almanac, add the 
    // remaining ids unmapped to the output
    if (current < end) {
        new_id_ranges.push(IdRange::new(mapper.destination, current, end - current))
    }

    new_id_ranges
}
```

The recursive function that steps through the categories now needs to use `flat_map` to handle that multiple ranges 
might be returned, and the final answer is the smallest lower bound in the resulting `Location` id ranges.

```rust
fn progress_id_ranges_to(
    id_ranges: Vec<IdRange>,
    category: Category,
    almanac: &Almanac,
) -> Vec<IdRange> {
    let current_category = id_ranges.get(0).unwrap().category;

    if current_category == category {
        id_ranges
    } else {
        progress_id_ranges_to(
            id_ranges
                .iter()
                .flat_map(|range| progress_id_range(range, almanac))
                .collect(),
            category,
            almanac,
        )
    }
}

fn find_nearest_location_from_ranges(seeds: Vec<IdRange>, almanac: &Almanac) -> i64 {
    progress_id_ranges_to(seeds, Location, almanac)
        .iter()
        .map(|range| range.start)
        .min()
        .unwrap()
}
```

This finds the correct answer in ~3ms, unoptimised.

## Refactorings

I tidy up some of the namings, especially renaming `Range` to `AlmanacRange` and the same with related properties and 
variable names. This makes it explicit whether something is a range of entity ids, or a range of ids that will be 
transformed when a category mapping is applied. 

The other big change I make is getting rid of `Id` and most of the part one solution code. Instead., I use the part 
two code to solve the same solution with ranges of single ids.

```rust
/// For part one each seed is a single id, which can be represented as a range
/// of length 1
fn ids_as_single_seeds(ids: &Vec<i64>) -> Vec<IdRange> {
    ids.into_iter()
        .map(|&start| IdRange::new(Seed, start, 1))
        .collect()
}

/// For part two each pair of numbers represents a range, in the 
/// format `start length`
fn ids_to_ranges(ids: &Vec<i64>) -> Vec<IdRange> {
    ids.into_iter()
        .tuples()
        .map(|(&start, &length)| IdRange::new(Seed, start, length))
        .collect()
}
```

## Final thoughts

Having part one have an obvious solution that doesn't scale to part two is a common puzzle type for Advent of Code, 
and at least the first one each year usually catches me out. It was satisfying to figure out a more efficient way of 
mapping the ranges, but there was a lot of tedious parsing to get there, so today was a bit of a drag. I did make 
things harder for my self by actually tracking the categories rather than stepping through the sections, since 
appeared in transformation order. I feel the code makes more sense for having the domain context represented in that 
way.
