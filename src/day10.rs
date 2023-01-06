use aoc_runner_derive::{aoc, aoc_generator};

use Instruction::*;

#[derive(Debug)]
enum Instruction {
    Noop,
    AddXLostCycle,
    AddX(i64),
}

#[aoc_generator(day10)]
fn input_generator(input: &str) -> Vec<Instruction> {
    let mut program = Vec::new();
    for line in input.lines() {
        let mut parts = line.split_whitespace();
        match (parts.next(), parts.next()) {
            (Some("noop"), None) => program.push(Noop),
            (Some("addx"), Some(n)) => {
                program.push(AddXLostCycle);
                program.push(AddX(n.parse().unwrap()))
            }
            _ => unreachable!(),
        }
    }
    program
}

#[inline]
fn execute(state: i64, instruction: &Instruction) -> i64
{
    match instruction {
        AddX(n) => state + *n,
        _ => state,
    }
}

#[aoc(day10, part1)]
fn solve_part1(program: &[Instruction]) -> usize {
    let steps = [20usize, 60, 100, 140, 180, 220];
    program.iter().scan(1, |x, inst| {
        let prev = *x;
        *x = execute(*x, inst);
        Some(prev)
    })
        .zip(1usize..)
        .filter(|(_, cycle)| steps.contains(cycle))
        .map(|(x, cycle)| cycle * (x as usize))
        .sum()
}

#[aoc(day10, part2)]
fn solve_part2(program: &[Instruction]) -> String {
    let lines = program.iter().enumerate().scan(1, |x, (cycle, inst)| {
        let column = cycle as i64 % 40;
        let pixel = if *x - 1 <= column && column <= *x + 1 { '#' } else { '.' };
        *x = execute(*x, inst);
        Some(pixel)
    }).collect::<Vec<_>>()
        .chunks(40)
        .map(String::from_iter)
        .collect::<Vec<_>>();
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::{input_generator, solve_part1, solve_part2};

    static INPUT: &str = "\
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn solver_part1_match_example() {
        assert_eq!(solve_part1(&input_generator(INPUT)), 13140);
    }


    #[test]
    fn solver_part2_match_example() {
        let expected = "\
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";
        assert_eq!(solve_part2(&input_generator(INPUT)), expected);
    }
}

