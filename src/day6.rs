use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<u8> {
    input.bytes().collect()
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &[u8]) -> usize {
    solve(input, 4)
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &[u8]) -> usize {
    solve(input, 14)
}

fn solve(input: &[u8], wide: usize) -> usize {
    input
        .windows(wide)
        .enumerate()
        .find(|(_, window)|  all_uniques(window))
        .unwrap()
        .0 + wide
}

fn all_uniques(group: &[u8]) -> bool {
    group
        .iter()
        .fold(0u32, |acc, elem| acc | (1 << (elem - b'a')))
        .count_ones() as usize == group.len()
}

#[cfg(test)]
mod tests {
    use super::{input_generator, solve_part1, solve_part2};

    static INPUT: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";

    #[test]
    fn solver_part1_match_example() {
        assert_eq!(solve_part1(&input_generator(INPUT)), 10);
    }

    #[test]
    fn solver_part2_match_example() {
        assert_eq!(solve_part2(&input_generator(INPUT)), 29);
    }
}