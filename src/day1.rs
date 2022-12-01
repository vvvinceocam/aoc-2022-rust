use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<Vec<u32>> {
    input
        .split("\n\n")
        .map(|elf| {
            elf.split("\n")
                .map(|cal| cal.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[Vec<u32>]) -> u32 {
    input.iter().map(|elf| elf.iter().sum()).max().unwrap()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[Vec<u32>]) -> u32 {
    let mut elves = input.iter().map(|elf| elf.iter().sum()).collect::<Vec<u32>>();
    elves.select_nth_unstable_by(3, |a, b| b.cmp(a)).0.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::{solve_part1, solve_part2};

    #[test]
    fn valid_solver_part1() {
        let input = vec![
            vec![1000, 2000, 3000],
            vec![4000],
            vec![5000, 6000],
            vec![7000, 8000, 9000],
            vec![10_000],
        ];

        assert_eq!(solve_part1(&input), 24_000);
    }

    #[test]
    fn valid_solver_part2() {
        let input = vec![
            vec![1000, 2000, 3000],
            vec![4000],
            vec![5000, 6000],
            vec![7000, 8000, 9000],
            vec![10_000],
        ];

        assert_eq!(solve_part2(&input), 45_000);
    }
}
