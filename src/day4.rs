use aoc_runner_derive::{aoc, aoc_generator};

type AssignmentPair = ((u32, u32), (u32, u32));

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<AssignmentPair> {
    input
        .split('\n')
        .map(|line| {
            let mut pair = line.split(',')
                .map(|pair| {
                    let mut assignments = pair.split('-')
                        .map(|id| id.parse().unwrap());
                    (assignments.next().unwrap(), assignments.next().unwrap())
                });
            (pair.next().unwrap(), pair.next().unwrap())
        })
        .collect()
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &[AssignmentPair]) -> usize {
    input
        .iter()
        .filter(|((a, b), (x, y))| (a <= x && b >= y) || (x <= a && y >= b))
        .count()
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &[AssignmentPair]) -> usize {
    input
        .iter()
        .filter(|((a, b), (x, y))| !((b < x) || (y < a)))
        .count()
}

#[cfg(test)]
mod tests {
    use super::{input_generator, solve_part1, solve_part2};

    static INPUT: &str = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn input_generator_builds_vec() {
        let expect = vec![
            ((2, 4), (6, 8)),
            ((2, 3), (4, 5)),
            ((5, 7), (7, 9)),
            ((2, 8), (3, 7)),
            ((6, 6), (4, 6)),
            ((2, 6), (4, 8)),
        ];
        assert_eq!(input_generator(INPUT), expect);
    }

    #[test]
    fn solver_part1_match_example() {
        assert_eq!(solve_part1(&input_generator(INPUT)), 2);
    }

    #[test]
    fn solver_part2_match_example() {
        assert_eq!(solve_part2(&input_generator(INPUT)), 4);
    }
}

