---
day: 1
tags: [ post ]
header: 'Day 1: Trebuchet?'
---

Today was a journey: Part one I was able to solve quickly and with fairly simple code. Part two had a hidden gotcha
that wasn't evident in the test data, and had a hacky work-around once found that left my code a bit of a mess. A
few rounds of refactoring later I'm now pretty happy with it.

## Part 1 - Finding digits in strings

The input is a bunch of lines of strings that contain obfuscated two-digit numbers. There's some example data I can turn
into a test:

```rust
#[test]
fn can_parse_lines() {
    assert_eq!(parse_line("1abc2"), 12);
    assert_eq!(parse_line("pqr3stu8vwx"), 38);
    assert_eq!(parse_line("a1b2c3d4e5f"), 15);
    assert_eq!(parse_line("treb7uchet"), 77);
}
```

Which needs a parse line function. The plan was to:

* Iterate over the characters,
* Filter out non-digit characters,
* Take the first and the last from the remaining sequence,
* Combine these into a number

After implementing this as a filter using `char::is_digit(10)` and then parsing that as a number it felt a bit
clunky. Splitting the filter and map, give the parsing returned an option I was immediately unwrapping due to the
filter making it infallible was awkward. I had a look through the iter API and found `filter_map` which combined the
two into one step that still communicated what it was doing to future me.

The final example demonstrates that a string with a single digit should return that as the first and last, so no need to
do anything fancy other than take the first match and the last match independently, and combine them.

```rust
fn parse_line(line: &str) -> u32 {
    let digits: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();

    digits.first().unwrap_or(&0) * 10 + digits.last().unwrap_or(&0)
}
```

The puzzle solution is the sum of each line which can be tested using the same data, and the implementation can be
done by composing rust standard functions, and `parse_line`.

```rust
#[test]
fn can_sum_calibration_values() {
    let input = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
        .to_string();

    assert_eq!(sum_calibration_values(&input), 142)
}

fn sum_calibration_values(input: &String) -> u32 {
    input.lines().map(parse_line).sum()
}
```

That gave the correct answer. So far, so good.

## Part 2 - Also digits as words

Today's twist is that `one`, `two`, `three`, `four`, `five`, `six`, `seven`, `eight`, and `nine` also count as valid
digits. I already have working code that will find numerical digits, so my first thought is to replace the word digits
with the equivalent numeric one. There is some new examples, so I can turn those into substitutions. Some human error
sneaks in here.

```rust
fn can_substitute_digit_words() {
    assert_eq!(substitute_digit_strings("two1nine"), "219");
    assert_eq!(substitute_digit_strings("eightwothree"), "823");
    assert_eq!(substitute_digit_strings("abcone2threexyz"), "abc123xyz");
    assert_eq!(substitute_digit_strings("xtwone3four"), "x2134");
    assert_eq!(substitute_digit_strings("4nineeightseven2"), "49872");
    assert_eq!(substitute_digit_strings("zoneight234"), "z18234");
    assert_eq!(substitute_digit_strings("7pqrstsixteen"), "7pqrst6teen");
    // Five isn't covered above so add a simple case for that too
    assert_eq!(substitute_digit_strings("five"), "5");
}

#[test]
fn can_sum_calibration_values_with_string_digits() {
    let input = "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
        .to_string();

    assert_eq!(sum_calibration_values_with_substitution(&input), 281)
}
```

I write a very naive solution...

```rust
fn substitute_digit_strings(line: &str) -> String {
    line.replace("one", "1")
        .replace("two", "2")
        .replace("three", "3")
        .replace("four", "4")
        .replace("five", "5")
        .replace("six", "6")
        .replace("seven", "7")
        .replace("eight", "8")
        .replace("nine", "9")
}
```

...which fails on the second example. I'd missed that it overlaps as `eightwo...`, not `eighttwo`, and because the
replacements happen in numeric order, the one is replaced first, so I'm seeing the assertion fail with `eigh23 != 823`.

Fixing the tests only requires correcting the strings.

```rust
fn can_substitute_digit_words() {
    //...
    assert_eq!(substitute_digit_strings("eightwothree"), "8wo3");
    //...
    assert_eq!(substitute_digit_strings("xtwone3four"), "x2ne34");
    //...
    assert_eq!(substitute_digit_strings("zoneight234"), "z1ight234");
    //...
}
```

Fixing the code is a bit more tedious. I realise I probably need a regex, and after digging through Ecosia and the regex
crate docs, I settle on using a similarly verbose replacement function.

The regex itself is ok, but compiling a new one each time is wasteful, but it can't be declared as a static variable
because it can't statically compile the Regex. Luckily someone's written a `lazy_static!` macro to fix just this
problem.

```rust
lazy_static! {
    static ref PATTERN: Regex = 
        Regex::new(r"(one|two|three|four|five|six|seven|eight|nine)")
              .unwrap();
}
```

The replacement function can match on the digit words and because it's only getting input from regex matches I know
that's exhaustive even if the compiler doesn't. I went for a fairly verbose match expression here instead of looking
the digit string up in a Map or Vec as I feel this more clearly expresses the intent.

```rust
fn substitute_digit_strings(line: &str) -> String {
    let replacement = |caps: &Captures| -> String {
        match &caps[0] {
            "one" => "1".to_string(),
            "two" => "2".to_string(),
            "three" => "3".to_string(),
            "four" => "4".to_string(),
            "five" => "5".to_string(),
            "six" => "6".to_string(),
            "seven" => "7".to_string(),
            "eight" => "8".to_string(),
            "nine" => "9".to_string(),
            _ => unreachable!()
        }
    };

    PATTERN.replace_all(
        line,
        &replacement
    ).to_string()
}
```

The solution is then composing the substitution with the existing parser.

```rust
fn sum_calibration_values_with_substitution(input: &String) -> u32 {
    input
        .lines()
        .map(|line| parse_line(substitute_digit_strings(line).as_str()))
        .sum()
}
```

The tests pass, but the puzzle solution it generates from the actual input is incorrect. The nature of the input - a
number lacking any context, means the error message can only be an unhelpful "The number is too low". I followed the
link to Reddit where `u/Zefick` had helpfully posted:

> ### [2023 Day 1]For those who stuck on Part 2
>
> The right calibration values for string "eighthree" is 83 and for "sevenine" is 79.
>
> The examples do not cover such cases.

I can however keep my replacement solution if I leave the letters that can overlap on each side. This also means I
can go back to the basic replacement as the order no longer matters as the replacements no longer conflict, so it's back
to the order not mattering. There is a bit of a code smell in that the tests are now relying on an implementation
detail, but this is a quick fix to get the existing code working.

```rust
#[test]
fn can_substitute_digit_words() {
    assert_eq!(substitute_digit_strings("two1nine"), "t2o1e9");
    assert_eq!(substitute_digit_strings("eightwothree"), "e8t2ot3e");
    assert_eq!(substitute_digit_strings("abcone2threexyz"), "abco1e2t3exyz");
    assert_eq!(substitute_digit_strings("xtwone3four"), "xt2o1e34");
    assert_eq!(substitute_digit_strings("4nineeightseven2"), "4n9ee8t7n2");
    assert_eq!(substitute_digit_strings("zoneight234"), "zo1e8t234");
    assert_eq!(substitute_digit_strings("7pqrstsixteen"), "7pqrst6teen");
    assert_eq!(substitute_digit_strings("five"), "5e");
}

fn substitute_digit_strings(line: &str) -> String {
    line.replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "4")
        .replace("five", "5e")
        .replace("six", "6")
        .replace("seven", "7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e")
}
```

The tests pass, and the correct answer is now generated when run against the puzzle input 🎉.

## Refactoring

The solution, whilst working isn't sitting right with me. The trick with keeping the surrounding digits works, but
is not clear when read. I can add some comments explaining the logic, but I'd like to find a better way. The tests
exposing the substitution hack also feels wrong. I can refactor the part two tests to just use a modified parse_input
that just tests on the first and last digit, i.e. testing the expected behavior not the implementation details.

Some colleagues post ideas on Slack that they used regexes with lookahead to match both the numeric and textual
digits, without consuming the match and missing the overlaps. That avoided the substitution, and could handle
overlaps. The usual Rust regex crate doesn't support lookaheads, but I can emulate it by asking for a single match,
then start again from the character after the start of the match.

I also want to address an issue from the first regex attempt where the regex and replacer were only tangentially tied
together, which is more of an issue now that I'll need separate pairs for part one and part two. I start off
reaching for a recursive function as I need to track quite a few moving parts. I need to get back into the rust
mindset because this would have been much clearer with a loop and mutable variables, and the borrow checker has my
back for the memory safety / impurity that recursion can be used to avoid. I'll go through the steps in code:

```rust
// Tie the regex and parser together
struct ValueExtractor {
    pattern: Regex,
    digit_mapper: fn(&str) -> u32,
}

// Make the tests match the API I now want hiding implementation details. 
// `can_substitute_digit_words` is removed
#[test]
fn can_parse_lines() {
    let part_1_extractor = part_1_extractor();

    assert_eq!(parse_line("1abc2", &part_1_extractor), 12);
    assert_eq!(parse_line("pqr3stu8vwx", &part_1_extractor), 38);
    assert_eq!(parse_line("a1b2c3d4e5f", &part_1_extractor), 15);
    assert_eq!(parse_line("treb7uchet", &part_1_extractor), 77);

    let part_2_extractor = part_2_extractor();

    assert_eq!(parse_line("two1nine", &part_2_extractor), 29);
    assert_eq!(parse_line("eightwothree", &part_2_extractor), 83);
    assert_eq!(parse_line("abcone2threexyz", &part_2_extractor), 13);
    assert_eq!(parse_line("xtwone3four", &part_2_extractor), 24);
    assert_eq!(parse_line("4nineeightseven2", &part_2_extractor), 42);
    assert_eq!(parse_line("zoneight234", &part_2_extractor), 14);
    assert_eq!(parse_line("7pqrstsixteen", &part_2_extractor), 76);

    assert_eq!(parse_line("five", &part_2_extractor), 55);
    assert_eq!(parse_line("eighthree", &part_2_extractor), 83);
}

#[test]
fn can_sum_calibration_values() {
    let part_1_input = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
        .to_string();

    assert_eq!(
        sum_calibration_values(&part_1_input, &part_1_extractor()),
        142
    );

    let part_2_input = "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
        .to_string();

    assert_eq!(
        sum_calibration_values(&part_2_input, &part_2_extractor()),
        281
    );
}

// Parse line has the biggest change. It now needs to get the first and 
// last possibly overlapping regex matches and combine those into the
// final value.
fn parse_line(line: &str, extractor: &ValueExtractor) -> u32 {
    // My habit of an inner recursive function called iter is a
    // hangover from when I learnt Scala...
    fn iter(
        line: &str,
        extractor: &ValueExtractor,
        pos: usize,
        tens: Option<u32>,
        units: Option<u32>,
    ) -> u32 {
        match extractor.pattern.find_at(line, pos) {
            // If we find a match...
            Some(m) => {
                // ...turn it into a digit
                let value = (extractor.digit_mapper)(m.as_str());

                // ... recurse
                iter(
                    line,
                    extractor,
                    // Start from next character  
                    m.start() + 1,
                    // Set the first digit only if it's not already set
                    tens.or(Some(value)),
                    // Always set the last seen value
                    Some(value),
                )
            }
            // As the base case - no more matches found - return the 
            // combined number as before.
            None => tens.unwrap_or(0) * 10 + units.unwrap_or(0),
        }
    }

    // Kick off the recursive function
    iter(line, extractor, 0, None, None)
}

// The separate part functions can now be combined
fn sum_calibration_values(input: &String, extractor: &ValueExtractor) -> u32 {
    input.lines().map(|line| parse_line(line, &extractor)).sum()
}

// And the change between the two parts is now nicely contained
fn part_1_extractor() -> ValueExtractor {
    ValueExtractor {
        pattern: Regex::new(r"\d").unwrap(),
        digit_mapper: |d| d.parse().unwrap(),
    }
}

fn part_2_extractor() -> ValueExtractor {
    ValueExtractor {
        pattern:
        Regex::new(
            r"(\d|one|two|three|four|five|six|seven|eight|nine)"
        ).unwrap(),
        digit_mapper: |d| match d {
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9,
            _ => d.parse().unwrap(),
        },
    }
}
```

This is going in the right direction I think, but there are still improvements to make. The recursive function is hard
to follow. It is also parsing values that are later throw away. I'd like to pull out the stepping through the regex 
matches, and tracking the position to start the next seek from, and the parsing. Ideally I'd like to go back to the 
style before where the regex produces a Vec of strings it matched, and the first and last are pulled out and parsed. 
The itertools library has an `unfold` generator to help do exactly that. I feel the function could do with some 
comments to help explain it, but the complexity caused by needing to support overlapping regex matches is now 
encapsulated and `parse_line` is much clearer, and quite close to what it looked like for the naive regex solution.

```rust
/// Return regex matches that might overlap
///
/// ```rust
/// let pattern = Regex::new(r"(eight|three)").unwrap();
/// let res: Vec<&str> = overlapping_matches("eighthree", &pattern);
/// assert_eq!(res, vec!("eight", "three"));
/// ```
fn overlapping_matches<'a>(line: &'a str, pattern: &Regex) -> Vec<&'a str> {
    unfold(0usize, |pos| {
        // Find the next match
        let digit = pattern.find_at(line, *pos);
        // The next iteration should start from the next character after
        // the match to allow for overlaps
        *pos = digit.map(|m| m.start()).unwrap_or(0) + 1;
        // For convenience return just the match contents
        digit.map(|m| m.as_str())
    })
        .collect()
}

fn parse_line(line: &str, extractor: &ValueExtractor) -> u32 {
    let matches: Vec<&str> = overlapping_matches(line, &extractor.pattern);

    let tens = matches
        .first()
        .map(|&s| (extractor.digit_mapper)(s))
        .unwrap_or(0);

    let units = matches
        .last()
        .map(|&s| (extractor.digit_mapper)(s))
        .unwrap_or(0);

    tens * 10 + units
}
```

## Final thoughts

It's never great when the examples don't cover an awkward to debug gotcha. I'm very glad of the hint to get unstuck, 
even if it was a spoiler. It's still great to get back into the swing of advent of code, and it turned out to be an 
interesting challenge to refactor the code. I also needed to use a lifetime to tie the life of the returned matches to 
the line they reference. Being able to do that on day one is a step-up in my recollection of Rust compared to previous 
years, which is encouraging.
