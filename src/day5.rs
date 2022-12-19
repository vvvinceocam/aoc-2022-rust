use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};

type Input = (Stacks, Vec<Instruction>);

type Stacks = Vec<Vec<char>>;

#[derive(Debug, Eq, PartialEq)]
pub struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(' ').collect();
        Ok(Self {
            count: parts[1].parse().unwrap(),
            from: parts[3].parse().unwrap(),
            to: parts[5].parse().unwrap(),
        })
    }
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Input {
    let (lines, instructions) = input.split_once("\n\n").unwrap();
    let mut lines = lines.split('\n').rev();
    let mut stacks = vec![vec![]; (lines.next().unwrap().len() + 1) / 4];
    for line in lines {
        line.chars()
            .skip(1)
            .step_by(4)
            .enumerate()
            .filter(|(_, item)| *item != ' ')
            .for_each(|(col, item)| stacks[col].push(item));
    }
    let instructions = instructions
        .split('\n')
        .map(|line| line.parse().unwrap())
        .collect();
    (stacks, instructions)
}

#[aoc(day5, part1)]
pub fn solve_part1((stacks, instructions): &Input) -> String {
    let mut stacks = stacks.clone();
    for instruction in instructions {
        for _ in 0..instruction.count {
            let value = stacks[instruction.from - 1].pop().unwrap();
            stacks[instruction.to - 1].push(value);
        }
    }
    stacks.iter().map(|stack| stack.last().unwrap()).collect()
}

#[aoc(day5, part2)]
pub fn solve_part2((stacks, instructions): &Input) -> String {
    let mut stacks = stacks.clone();
    let mut values = vec![];
    for instruction in instructions {
        for _ in 0..instruction.count {
            values.push(stacks[instruction.from - 1].pop().unwrap());
        }
        for _ in 0..instruction.count {
            stacks[instruction.to - 1].push(values.pop().unwrap());
        }
    }
    stacks.iter().map(|stack| stack.last().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use crate::day5::Instruction;

    use super::{input_generator, solve_part1, solve_part2};

    static INPUT: &str = "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n\nmove 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2";

    #[test]
    fn input_generator_builds_vec() {
        let expect = (
            vec![
                vec!['Z', 'N'],
                vec!['M', 'C', 'D'],
                vec!['P'],
            ],
            vec![
                Instruction { count: 1, from: 2, to: 1 },
                Instruction { count: 3, from: 1, to: 3 },
                Instruction { count: 2, from: 2, to: 1 },
                Instruction { count: 1, from: 1, to: 2 },
            ]
        );
        assert_eq!(input_generator(INPUT), expect);
    }

    #[test]
    fn solver_part1_match_example() {
        assert_eq!(&solve_part1(&input_generator(INPUT)), "CMZ");
    }

    #[test]
    fn solver_part2_match_example() {
        assert_eq!(&solve_part2(&input_generator(INPUT)), "MCD");
    }
}

