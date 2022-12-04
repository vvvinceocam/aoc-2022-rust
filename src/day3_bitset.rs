use std::ops::BitAnd;

use aoc_runner_derive::aoc;

use crate::day3::{input_generator, priority};

#[aoc(day3, part1, bitset)]
pub fn solve_part1(input: &[Vec<u8>]) -> u32 {
    input
        .iter()
        .map(|bag| priority(common(bag.chunks(bag.len() / 2))))
        .sum()
}

#[aoc(day3, part2, bitset)]
pub fn solve_part2(input: &[Vec<u8>]) -> u32 {
    input
        .chunks(3)
        .map(|bags| priority(common(bags.iter().map(Vec::as_slice))))
        .sum()
}

/// Given an iterator of bytes slices, find the common element of those collections.
fn common<'a, I>(iterator: I) -> u8
    where
        I: Iterator<Item=&'a [u8]>,
{
    extract_item(iterator.map(into_bitset).fold(u128::MAX, u128::bitand))
}

fn into_bitset(items: &[u8]) -> u128 {
    items.iter().fold(0, |acc, item| acc | (1 << (*item as u128 - 1)))
}

fn extract_item(bitset: u128) -> u8 {
    (0..127).find(|i| (bitset >> *i) == 1).unwrap() as u8 + 1
}

#[cfg(test)]
mod tests {
    use super::{input_generator, solve_part1, solve_part2, extract_item, into_bitset};

    static INPUT: &str = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn solver_part1_match_example() {
        assert_eq!(solve_part1(&input_generator(INPUT)), 157);
    }

    #[test]
    fn solver_part2_match_example() {
        assert_eq!(solve_part2(&input_generator(INPUT)), 70);
    }

    #[test]
    fn reversible_bitset() {
        assert_eq!(extract_item(into_bitset(&[8])), 8)
    }
}
