use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<(i32, i32)> {
    input
        .split('\n')
        .map(|line| {
            let bytes = line.as_bytes();
            ((bytes[0] - b'A') as i32, (bytes[2] - b'X') as i32)
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[(i32, i32)]) -> i32 {
    input
        .iter()
        .map(|(him, me)|
            match (him - me).rem_euclid(3) {
                0 => 3,
                1 => 0,
                _ => 6,
            } + me + 1
        )
        .sum()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[(i32, i32)]) -> i32 {
    input
        .iter()
        .map(|(him, outcome)|
            match outcome {
                0 => *him - 1,
                1 => *him,
                _ => *him + 1,
            }.rem_euclid(3) + 1 + outcome * 3
        )
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{input_generator, solve_part1, solve_part2};

    static INPUT: &str = "A Y\nB X\nC Z";

    #[test]
    fn input_generator_builds_vec() {
        let expect = vec![(0, 1), (1, 0), (2, 2)];
        assert_eq!(input_generator(INPUT), expect);
    }

    #[test]
    fn solver_part1_match_example() {
        assert_eq!(solve_part1(&input_generator(INPUT)), 15);
    }

    #[test]
    fn solver_part2_match_example() {
        assert_eq!(solve_part2(&input_generator(INPUT)), 12);
    }
}
