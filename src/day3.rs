use std::collections::HashSet;
use std::hash::Hash;
use std::ops::Deref;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Vec<u8>> {
    input
        .split('\n')
        .map(|line| line.as_bytes().to_vec())
        .collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[Vec<u8>]) -> u32 {
    input
        .iter()
        .map(|bag| priority(common(bag.chunks(bag.len() / 2))))
        .sum()
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &[Vec<u8>]) -> u32 {
    input
        .chunks(3)
        .map(|bags| priority(common(bags.iter())))
        .sum()
}

/// Given an iterator of iterable collections, find the common element of those collections.
fn common<'a, I, C, T>(mut iterator: I) -> T
    where
        I: Iterator<Item=C>,
        C: IntoIterator<Item=&'a T>,
        T: Clone + Eq + Hash + 'a,
{
    let init = iterator.next().unwrap().into_iter().collect::<HashSet<_>>();
    let set = iterator.fold(init, |acc, elems| &acc & &elems.into_iter().collect());
    set.iter().next().unwrap().deref().clone()
}

pub fn priority(item: u8) -> u32 {
    if item <= b'Z' {
        item - b'A' + 27
    } else {
        item - b'a' + 1
    }.into()
}

#[cfg(test)]
mod tests {
    use super::{input_generator, priority, solve_part1, solve_part2};

    static INPUT: &str = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn priority_for_sample() {
        let cases = [
            (b'p', 16),
            (b'L', 38),
            (b'P', 42),
            (b'v', 22),
            (b't', 20),
            (b's', 19),
        ];

        for (input, expected) in cases {
            assert_eq!(priority(input), expected);
        }
    }

    #[test]
    fn solver_part1_match_example() {
        assert_eq!(solve_part1(&input_generator(INPUT)), 157);
    }

    #[test]
    fn solver_part2_match_example() {
        assert_eq!(solve_part2(&input_generator(INPUT)), 70);
    }
}
