use std::convert::Infallible;
use std::fmt::Debug;
use std::mem;
use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Eq, PartialEq, Clone)]
enum Operation {
    Square,
    Add(usize),
    Multiply(usize),
}

impl Operation {
    pub fn exec(&self, level: usize) -> usize {
        match self {
            Operation::Square => level * level,
            Operation::Add(n) => level + *n,
            Operation::Multiply(n) => level * *n,
        }
    }
}

impl FromStr for Operation {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        parts.next(); // skip first "old"
        Ok(match (parts.next(), parts.next()) {
            (Some("*"), Some("old")) => Operation::Square,
            (Some("*"), Some(n)) => Operation::Multiply(n.parse().unwrap()),
            (Some("+"), Some(n)) => Operation::Add(n.parse().unwrap()),
            _ => unreachable!(),
        })
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Selector {
    divider: usize,
    on_true: usize,
    on_false: usize,
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Monkey {
    items: Vec<usize>,
    op: Operation,
    selector: Selector,
}

impl Monkey {
    fn predict(&self, level: usize) -> usize {
        self.op.exec(level)
    }

    fn select(&self, level: usize) -> usize {
        if level % self.selector.divider == 0 {
            self.selector.on_true
        } else {
            self.selector.on_false
        }
    }
}

impl FromStr for Monkey {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().skip(1);

        let items = lines.next().unwrap().split(':').nth(1).unwrap().split(',').map(|part| part.trim().parse().unwrap()).collect();
        let op = lines.next().unwrap().split('=').nth(1).unwrap().trim().parse().unwrap();
        let mut selector = lines.map(line_last_item);

        Ok(Self {
            items,
            op,
            selector: Selector {
                divider: selector.next().unwrap(),
                on_true: selector.next().unwrap(),
                on_false: selector.next().unwrap(),
            },
        })
    }
}

fn line_last_item<T, E>(line: &str) -> T
    where
        T: FromStr<Err=E>,
        E: Debug,
{
    line.split_whitespace().last().unwrap().parse().unwrap()
}

#[aoc_generator(day11)]
fn input_generator(input: &str) -> Vec<Monkey> {
    input.split("\n\n").map(|block| block.parse().unwrap()).collect()
}

fn solve<F>(monkeys: &[Monkey], rounds: usize, inibitor: F) -> usize
    where
        F: Fn(usize) -> usize
{
    let mut monkeys = monkeys.to_vec();
    let mut counters = vec![0; monkeys.len()];

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let mut items = vec![];
            mem::swap(&mut items, &mut monkeys[i].items);
            for item in items {
                counters[i] += 1;
                let (j, item) = {
                    let level = inibitor(monkeys[i].predict(item));
                    let next_monkey = monkeys[i].select(level);
                    (next_monkey, level)
                };
                monkeys[j].items.push(item);
            }
        }
    }

    counters.select_nth_unstable_by(2, |a, b| b.cmp(a));
    counters[0] * counters[1]
}

#[aoc(day11, part1)]
fn solve_part1(monkeys: &[Monkey]) -> usize {
    let inibitor = |x: usize| x / 3;
    solve(monkeys, 20, inibitor)
}

#[aoc(day11, part2)]
fn solve_part2(monkeys: &[Monkey]) -> usize {
    let module: usize = monkeys.iter().map(|monkey| monkey.selector.divider).product();
    let inibitor = |x: usize| x % module;
    solve(monkeys, 10_000, inibitor)
}

#[cfg(test)]
mod tests {
    use super::{input_generator, Monkey, Operation, Selector, solve_part1, solve_part2};

    static INPUT: &str = "\
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn input_generator_builds_vec() {
        let expected = vec![
            Monkey {
                items: vec![79, 98],
                op: Operation::Multiply(19),
                selector: Selector {
                    divider: 23,
                    on_true: 2,
                    on_false: 3,
                },
            },
            Monkey {
                items: vec![54, 65, 75, 74],
                op: Operation::Add(6),
                selector: Selector {
                    divider: 19,
                    on_true: 2,
                    on_false: 0,
                },
            },
            Monkey {
                items: vec![79, 60, 97],
                op: Operation::Square,
                selector: Selector {
                    divider: 13,
                    on_true: 1,
                    on_false: 3,
                },
            },
            Monkey {
                items: vec![74],
                op: Operation::Add(3),
                selector: Selector {
                    divider: 17,
                    on_true: 0,
                    on_false: 1,
                },
            },
        ];
        assert_eq!(input_generator(INPUT), expected);
    }

    #[test]
    fn solver_part1_match_example() {
        assert_eq!(solve_part1(&input_generator(INPUT)), 10605);
    }

    #[test]
    fn solver_part2_match_example() {
        assert_eq!(solve_part2(&input_generator(INPUT)), 2713310158);
    }
}
