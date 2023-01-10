use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::convert::Infallible;
use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Eq, PartialEq, Clone)]
enum Packet {
    Int(usize),
    List(Vec<Packet>),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (xs@Packet::List(_), y@Packet::Int(_)) => xs.cmp(&Packet::List(vec![y.clone()])),
            (x@Packet::Int(_), ys@Packet::List(_)) => Packet::List(vec![x.clone()]).cmp(ys),
            (Packet::Int(x), Packet::Int(y)) => x.cmp(y),
            (Packet::List(xs), Packet::List(ys)) => {
                let mut xs = xs.iter();
                let mut ys = ys.iter();
                loop {
                    match (xs.next(), ys.next()) {
                        (None, None) => break Equal,
                        (Some(_), None) => break Greater,
                        (None, Some(_)) => break Less,
                        (Some(x), Some(y)) => {
                            match x.cmp(y) {
                                Equal => continue,
                                ord => {
                                    break ord;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for Packet {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stream = s.chars().peekable();
        let mut stack: Vec<Vec<Packet>> = vec![];
        let mut num_buffer = String::new();

        loop {
            while char::is_numeric(*stream.peek().unwrap()) {
                num_buffer.push(stream.next().unwrap());
            }

            if !num_buffer.is_empty() {
                stack.last_mut().unwrap().push(Packet::Int(num_buffer.parse().unwrap()));
                num_buffer.clear();
            } else {
                match stream.next() {
                    None => break,
                    Some(',') => continue,
                    Some('[') => {
                        stack.push(vec![]);
                    }
                    Some(']') => {
                        let sublist = stack.pop().unwrap();
                        if let Some(list) = stack.last_mut() {
                            list.push(Packet::List(sublist));
                        } else {
                            return Ok(Packet::List(sublist));
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
        unreachable!()
    }
}

#[aoc_generator(day13, part1)]
fn input_generator1(input: &str) -> Vec<(Packet, Packet)> {
    input.split("\n\n")
        .map(|block| {
            let mut packets = block.lines().map(|line| line.parse().unwrap());
            (packets.next().unwrap(), packets.next().unwrap())
        })
        .collect()
}

#[aoc_generator(day13, part2)]
fn input_generator2(input: &str) -> Vec<Packet> {
    input.lines()
        .filter(|line| !line.is_empty())
        .flat_map(FromStr::from_str)
        .collect()
}

#[aoc(day13, part1)]
fn solve_part1(packets_groups: &[(Packet, Packet)]) -> usize {
    packets_groups.iter()
        .enumerate()
        .filter(|(_, (p1, p2))| p1 < p2)
        .map(|(i, _)| i + 1)
        .sum()
}

#[aoc(day13, part2)]
fn solve_part2(packets: &[Packet]) -> usize {
    let mut packets = packets.to_vec();
    let p2 = Packet::List(vec![
        Packet::List(vec![
            Packet::Int(2)
        ])
    ]);
    let p6 = Packet::List(vec![
        Packet::List(vec![
            Packet::Int(6)
        ])
    ]);
    packets.push(p2.clone());
    packets.push(p6.clone());
    packets.sort();
    let i2 = packets.binary_search(&p2).unwrap() + 1;
    let i6 = packets.binary_search(&p6).unwrap() + 1;
    i2 * i6
}

#[cfg(test)]
mod tests {
    use super::{input_generator1, input_generator2, solve_part1, solve_part2, Packet};

    static INPUT: &str = "\
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
";

    #[test]
    fn input_generator_builds_vec() {
        let expected = vec![
            (
                Packet::List(vec![
                    Packet::Int(1),
                    Packet::Int(1),
                    Packet::Int(3),
                    Packet::Int(1),
                    Packet::Int(1),
                ]),
                Packet::List(vec![
                    Packet::Int(1),
                    Packet::Int(1),
                    Packet::Int(5),
                    Packet::Int(1),
                    Packet::Int(1),
                ]),
            ),
            (
                Packet::List(vec![
                    Packet::List(vec![
                        Packet::Int(1),
                    ]),
                    Packet::List(vec![
                        Packet::Int(2),
                        Packet::Int(3),
                        Packet::Int(4),
                    ]),
                ]),
                Packet::List(vec![
                    Packet::List(vec![
                        Packet::Int(1),
                    ]),
                    Packet::Int(4),
                ])
            ),
            (
                Packet::List(vec![
                    Packet::Int(9),
                ]),
                Packet::List(vec![
                    Packet::List(vec![
                        Packet::Int(8),
                        Packet::Int(7),
                        Packet::Int(6),
                    ]),
                ])
            ),
            (
                Packet::List(vec![
                    Packet::List(vec![
                        Packet::Int(4),
                        Packet::Int(4),
                    ]),
                    Packet::Int(4),
                    Packet::Int(4),
                ]),
                Packet::List(vec![
                    Packet::List(vec![
                        Packet::Int(4),
                        Packet::Int(4),
                    ]),
                    Packet::Int(4),
                    Packet::Int(4),
                    Packet::Int(4),
                ])
            ),
            (
                Packet::List(vec![
                    Packet::Int(7),
                    Packet::Int(7),
                    Packet::Int(7),
                    Packet::Int(7),
                ]),
                Packet::List(vec![
                    Packet::Int(7),
                    Packet::Int(7),
                    Packet::Int(7),
                ]),
            ),
            (
                Packet::List(vec![]),
                Packet::List(vec![
                    Packet::Int(3),
                ]),
            ),
            (
                Packet::List(vec![
                    Packet::List(vec![
                        Packet::List(vec![]),
                    ]),
                ]),
                Packet::List(vec![
                    Packet::List(vec![]),
                ]),
            ),
            (
                Packet::List(vec![
                    Packet::Int(1),
                    Packet::List(vec![
                        Packet::Int(2),
                        Packet::List(vec![
                            Packet::Int(3),
                            Packet::List(vec![
                                Packet::Int(4),
                                Packet::List(vec![
                                    Packet::Int(5),
                                    Packet::Int(6),
                                    Packet::Int(7),
                                ]),
                            ]),
                        ]),
                    ]),
                    Packet::Int(8),
                    Packet::Int(9),
                ]),
                Packet::List(vec![
                    Packet::Int(1),
                    Packet::List(vec![
                        Packet::Int(2),
                        Packet::List(vec![
                            Packet::Int(3),
                            Packet::List(vec![
                                Packet::Int(4),
                                Packet::List(vec![
                                    Packet::Int(5),
                                    Packet::Int(6),
                                    Packet::Int(0),
                                ]),
                            ]),
                        ]),
                    ]),
                    Packet::Int(8),
                    Packet::Int(9),
                ]),
            ),
        ];

        assert_eq!(input_generator1(INPUT), expected);
    }

    #[test]
    fn solver_part1_match_example() {
        assert_eq!(solve_part1(&input_generator1(INPUT)), 13);
    }

    #[test]
    fn solver_part2_match_example() {
        assert_eq!(solve_part2(&input_generator2(INPUT)), 140);
    }

}
