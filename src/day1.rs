use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<Vec<u32>> {
    input
        .split("\n\n")
        .map(|elf| {
            elf.split('\n')
                .map(|cal| cal.parse::<u32>().unwrap())
                .collect()
        })
        .collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[Vec<u32>]) -> u32 {
    input.iter().map(|elf| elf.iter().sum()).max().unwrap()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[Vec<u32>]) -> u32 {
    input.iter()
        .map(|elf| elf.iter().sum())
        .collect::<Vec<u32>>()
        .select_nth_unstable_by(3, |a, b| b.cmp(a))
        .0.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::{input_generator, solve_part1, solve_part2};

    static INPUT: &str = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";

    #[test]
    fn input_generator_builds_vec() {
        let expect = vec![
            vec![1000, 2000, 3000],
            vec![4000],
            vec![5000, 6000],
            vec![7000, 8000, 9000],
            vec![10_000],
        ];
        assert_eq!(input_generator(INPUT), expect);
    }

    #[test]
    fn solver_part1_match_example() {
        assert_eq!(solve_part1(&input_generator(INPUT)), 24_000);
    }

    #[test]
    fn solver_part2_match_example() {
        assert_eq!(solve_part2(&input_generator(INPUT)), 45_000);
    }
}
