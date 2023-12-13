//! This is my solution for [Advent of Code - Day 12: _Hot Springs_](https://adventofcode.com/2023/day/12)
//!
//!

use crate::day_12::SpringCondition::*;
use itertools::Itertools;
use std::collections::HashSet;
use std::fs;

#[derive(Eq, PartialEq, Debug, Hash, Copy, Clone)]
enum SpringCondition {
    Operational(u8),
    Damaged(u8),
    Unknown(u8),
}

impl SpringCondition {
    fn value(&self) -> u8 {
        match self {
            Operational(n) | Damaged(n) | Unknown(n) => *n,
        }
    }

    fn is_operational(&self) -> bool {
        if let Operational(_) = self {
            true
        } else {
            false
        }
    }

    fn is_damaged(&self) -> bool {
        if let Damaged(_) = self {
            true
        } else {
            false
        }
    }

    fn is_unknown(&self) -> bool {
        if let Unknown(_) = self {
            true
        } else {
            false
        }
    }

    fn with_value(&self, value: u8) -> SpringCondition {
        match self {
            Operational(_) => Operational(value),
            Damaged(_) => Damaged(value),
            Unknown(_) => Unknown(value),
        }
    }

    fn same_type(&self, other: &SpringCondition) -> bool {
        return (self.is_operational() && other.is_operational())
            || (self.is_damaged() && other.is_damaged())
            || (self.is_unknown() && other.is_unknown());
    }
}

#[derive(Eq, PartialEq, Debug, Hash, Clone)]
struct ConditionRow {
    springs: Vec<SpringCondition>,
    damaged_counts: Vec<u8>,
}

impl ConditionRow {
    fn new(springs: Vec<SpringCondition>, damaged_counts: Vec<u8>) -> ConditionRow {
        ConditionRow {
            springs,
            damaged_counts,
        }
    }

    fn spring_at(&self, pos: u8) -> Option<&SpringCondition> {
        let mut curr = 0;

        for group in self.springs.iter() {
            curr += group.value();
            if curr > pos {
                return Some(group);
            }
        }

        None
    }

    fn decide(&self, start: u8, replacement: SpringCondition) -> ConditionRow {
        let mut new_springs = Vec::new();
        let mut new_counts = self.damaged_counts.clone();
        new_counts.reverse();
        let mut start_remaining = start;
        let mut replace_remaining = replacement.value();

        fn push_group(vec: &mut Vec<SpringCondition>, new_group: SpringCondition) {
            if let Some(prev_group) = vec.pop() {
                if prev_group.same_type(&new_group) {
                    vec.push(new_group.with_value(prev_group.value() + new_group.value()));
                } else {
                    vec.push(prev_group);
                    vec.push(new_group);
                }
            } else {
                vec.push(new_group)
            }
        }

        for group in self.springs.iter() {
            if start_remaining > 0 {
                start_remaining -= group.value();
                if group.is_damaged() {
                    let maybe_count = new_counts.pop();
                    assert_eq!(
                        Some(group.value()),
                        maybe_count,
                        "{:?}\n.replace({}, {:?}) => {:?} rem {}",
                        self,
                        start,
                        replacement,
                        new_springs,
                        replace_remaining
                    );
                }
            } else if replace_remaining == 0 {
                push_group(&mut new_springs, group.clone())
            } else if replace_remaining <= group.value() {
                push_group(&mut new_springs, replacement);

                if replace_remaining < group.value() {
                    push_group(
                        &mut new_springs,
                        group.with_value(group.value() - replace_remaining),
                    )
                }

                replace_remaining = 0;
            } else {
                replace_remaining -= group.value()
            }
        }

        new_counts.reverse();

        ConditionRow::new(new_springs, new_counts)
    }

    fn explode(&self, repeats: usize) -> ConditionRow {
        let mut springs = self.springs.clone();
        for _ in 1..repeats {
            springs.push(Unknown(1));
            springs.extend(self.springs.clone());
        }

        ConditionRow::new(springs, self.damaged_counts.repeat(repeats)).decide(0, Operational(0))
    }
}

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-12-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 12.
pub fn run() {
    let contents = fs::read_to_string("res/day-12-input.txt").expect("Failed to read file");

    let rows = parse_input(&contents);

    println!("The sum of permutations is: {}", sum_permutations(&rows));

    println!(
        "The sum of unfolded permutations is: {}",
        sum_exploded_permutations(&rows)
    );
}

fn parse_input(input: &String) -> Vec<ConditionRow> {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> ConditionRow {
    let (spring_spec, damaged_counts_spec) = line.split_once(" ").unwrap();

    ConditionRow::new(
        parse_springs(spring_spec),
        parse_damaged_counts(damaged_counts_spec),
    )
}

fn parse_springs(spec: &str) -> Vec<SpringCondition> {
    let mut springs = Vec::new();

    for (chr, group) in spec.chars().group_by(|&c| c).into_iter() {
        springs.push(match chr {
            '?' => Unknown(group.count() as u8),
            '.' => Operational(group.count() as u8),
            '#' => Damaged(group.count() as u8),
            _ => unreachable!(),
        });
    }

    springs
}

fn parse_damaged_counts(spec: &str) -> Vec<u8> {
    spec.split(",").filter_map(|num| num.parse().ok()).collect()
}

fn count_permutations(row: &ConditionRow) -> usize {
    let mut permutations = 0;

    let mut visited = HashSet::new();
    let mut to_visit: Vec<ConditionRow> = Vec::new();
    to_visit.push(row.clone());

    while let Some(current) = to_visit.pop() {
        if visited.contains(&current.springs) {
            continue;
        }

        visited.insert(current.springs.clone());

        let unknown_count = current
            .springs
            .clone()
            .into_iter()
            .filter(SpringCondition::is_unknown)
            .count();
        if unknown_count == 0 {
            let actual_counts: Vec<u8> = current
                .springs
                .iter()
                .flat_map(|condition| {
                    if let Damaged(n) = condition {
                        Some(n.clone())
                    } else {
                        None
                    }
                })
                .collect();

            if actual_counts == current.damaged_counts {
                permutations += 1
            }

            continue;
        }

        let mut count_index = 0;
        let mut spring_index = 0;
        let mut non_operational_group_start = None;
        let mut starts_with_damaged = false;
        let mut has_damaged = false;

        for spring_group in vec![current.springs.clone(), vec![Operational(1)]].concat() {
            let expected_count = *current.damaged_counts.get(count_index).unwrap_or(&0);

            match (spring_group, non_operational_group_start) {
                (Operational(n), None) => spring_index += n,
                (Operational(_), Some(start_pos)) => {
                    let non_op_length = spring_index - start_pos;
                    if non_op_length < expected_count && !has_damaged {
                        to_visit.push(current.decide(start_pos, Operational(non_op_length)))
                    } else if non_op_length >= expected_count {
                        match current.spring_at(start_pos + expected_count) {
                            None => {
                                to_visit.push(current.decide(start_pos, Damaged(expected_count)));
                            }
                            Some(Damaged(_)) => {}
                            Some(_) => {
                                to_visit.push(
                                    current
                                        .decide(start_pos, Damaged(expected_count))
                                        .decide(start_pos + expected_count, Operational(1)),
                                );
                            }
                        }
                        if !starts_with_damaged {
                            to_visit.push(current.decide(start_pos, Operational(1)));
                        }
                    };

                    break;
                }
                (Damaged(n), None) if n > expected_count => break,

                (Damaged(n), None) if n == expected_count => {
                    count_index += 1;
                    spring_index += n;
                }

                (Damaged(n), None) => {
                    starts_with_damaged = true;
                    has_damaged = true;
                    non_operational_group_start =
                        non_operational_group_start.or(Some(spring_index));
                    spring_index += n;
                }
                (Unknown(n), None) => {
                    has_damaged = false;
                    starts_with_damaged = false;
                    non_operational_group_start =
                        non_operational_group_start.or(Some(spring_index));
                    spring_index += n;
                }
                (Damaged(n), Some(_)) => {
                    has_damaged = true;
                    spring_index += n;
                }
                (Unknown(n), Some(_)) => spring_index += n,
            }
        }
    }

    permutations
}

fn sum_permutations(rows: &Vec<ConditionRow>) -> usize {
    let len = rows.len();
    rows.iter()
        .enumerate()
        .map(|(i, row)| {
            println!("row {:4} / {}", i, len);
            count_permutations(row)
        })
        .sum()
}

fn sum_exploded_permutations(rows: &Vec<ConditionRow>) -> usize {
    let len = rows.len();
    rows.iter()
        .enumerate()
        .map(|(i, row)| {
            println!("row {:4} / {}", i, len);
            count_permutations(&row.explode(5))
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day_12::*;

    fn example_condition_rows() -> Vec<ConditionRow> {
        vec![
            ConditionRow::new(vec![Unknown(3), Operational(1), Damaged(3)], vec![1, 1, 3]),
            ConditionRow::new(
                vec![
                    Operational(1),
                    Unknown(2),
                    Operational(2),
                    Unknown(2),
                    Operational(3),
                    Unknown(1),
                    Damaged(2),
                    Operational(1),
                ],
                vec![1, 1, 3],
            ),
            ConditionRow::new(
                vec![
                    Unknown(1),
                    Damaged(1),
                    Unknown(1),
                    Damaged(1),
                    Unknown(1),
                    Damaged(1),
                    Unknown(1),
                    Damaged(1),
                    Unknown(1),
                    Damaged(1),
                    Unknown(1),
                    Damaged(1),
                    Unknown(1),
                    Damaged(1),
                    Unknown(1),
                ],
                vec![1, 3, 1, 6],
            ),
            ConditionRow::new(
                vec![
                    Unknown(4),
                    Operational(1),
                    Damaged(1),
                    Operational(3),
                    Damaged(1),
                    Operational(3),
                ],
                vec![4, 1, 1],
            ),
            ConditionRow::new(
                vec![
                    Unknown(4),
                    Operational(1),
                    Damaged(6),
                    Operational(2),
                    Damaged(5),
                    Operational(1),
                ],
                vec![1, 6, 5],
            ),
            ConditionRow::new(vec![Unknown(1), Damaged(3), Unknown(8)], vec![3, 2, 1]),
        ]
    }

    #[test]
    fn can_parse_input() {
        let input = "\
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"
            .to_string();

        assert_eq!(parse_input(&input), example_condition_rows());
    }

    #[test]
    fn can_count_permutations() {
        let examples = example_condition_rows();
        assert_eq!(count_permutations(&examples[0]), 1);
        assert_eq!(count_permutations(&examples[1]), 4);
        assert_eq!(count_permutations(&examples[2]), 1);
        assert_eq!(count_permutations(&examples[3]), 1);
        assert_eq!(count_permutations(&examples[4]), 4);
        assert_eq!(count_permutations(&examples[5]), 10);
        assert_eq!(count_permutations(&parse_line("#??.???..? 1,1,1")), 8);
        assert_eq!(count_permutations(&parse_line("#?#.???..? 1,1,1")), 4);
    }

    #[test]
    fn can_sum_permutations() {
        assert_eq!(sum_permutations(&example_condition_rows()), 21);
    }
    #[test]
    fn can_sum_exploded_permutations() {
        assert_eq!(sum_exploded_permutations(&example_condition_rows()), 525152);
    }

    #[test]
    fn can_explode_row() {
        let examples = example_condition_rows();
        assert_eq!(
            examples[0].explode(5),
            parse_line("???.###????.###????.###????.###????.### 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3")
        )
    }

    #[test]
    fn can_decide_value() {
        assert_eq!(
            ConditionRow::new(vec![Unknown(5)], Vec::new()).decide(0, Operational(2)),
            ConditionRow::new(vec![Operational(2), Unknown(3)], Vec::new())
        );
        assert_eq!(
            ConditionRow::new(vec![Operational(2), Unknown(5)], Vec::new())
                .decide(2, Operational(2)),
            ConditionRow::new(vec![Operational(4), Unknown(3)], Vec::new())
        );
        assert_eq!(
            ConditionRow::new(vec![Damaged(2), Unknown(5)], Vec::new()).decide(2, Operational(1)),
            ConditionRow::new(vec![Damaged(2), Operational(1), Unknown(4)], Vec::new())
        );
        assert_eq!(
            ConditionRow::new(
                vec![Damaged(2), Unknown(1), Damaged(2), Unknown(5)],
                Vec::new()
            )
            .decide(0, Damaged(6)),
            ConditionRow::new(vec![Damaged(6), Unknown(4)], Vec::new())
        );
        assert_eq!(
            ConditionRow::new(
                vec![Damaged(2), Unknown(1), Damaged(2), Unknown(5)],
                Vec::new()
            )
            .decide(0, Damaged(3)),
            ConditionRow::new(vec![Damaged(5), Unknown(5)], Vec::new())
        );
        assert_eq!(
            ConditionRow::new(
                vec![
                    Operational(1),
                    Damaged(1),
                    Operational(1),
                    Damaged(3),
                    Operational(1),
                    Damaged(1),
                    Operational(1),
                    Damaged(1),
                    Unknown(1),
                    Damaged(1),
                    Unknown(1),
                    Damaged(1),
                    Unknown(1)
                ],
                Vec::new()
            )
            .decide(9, Damaged(6)),
            ConditionRow::new(
                vec![
                    Operational(1),
                    Damaged(1),
                    Operational(1),
                    Damaged(3),
                    Operational(1),
                    Damaged(1),
                    Operational(1),
                    Damaged(6)
                ],
                Vec::new()
            )
        );
    }
}
