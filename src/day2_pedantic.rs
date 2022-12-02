//! Pedantic implementation for fun and profit
use std::error::Error;
use std::fmt::{Display, Formatter};

use aoc_runner_derive::{aoc, aoc_generator};

use Outcome::*;
use Play::*;

#[aoc_generator(day2, part1, pedantic)]
pub fn input_generator1(input: &str) -> Vec<(Play, Play)> {
    parse_input(input).unwrap()
}

#[aoc_generator(day2, part2, pedantic)]
pub fn input_generator2(input: &str) -> Vec<(Play, Outcome)> {
    parse_input(input).unwrap()
}

#[aoc(day2, part1, pedantic)]
pub fn solve_part1(input: &[(Play, Play)]) -> u32 {
    input
        .iter()
        .map(|(his, mine)| mine.as_score() + mine.versus(his).as_score())
        .sum()
}

#[aoc(day2, part2, pedantic)]
pub fn solve_part2(input: &[(Play, Outcome)]) -> u32 {
    input
        .iter()
        .map(|(his, outcome)|
            (match outcome {
                Lose => his.beats(),
                Draw => *his,
                Win => his.defeated_by(),
            }).as_score() + outcome.as_score()
        )
        .sum()
}

fn parse_input<T1, T2>(input: &str) -> Result<Vec<(T1, T2)>, ParsingError>
    where
        T1: TryFrom<char, Error=ParsingError>,
        T2: TryFrom<char, Error=ParsingError>,
{
    input
        .split('\n')
        .map(parse_line)
        .collect()
}

fn parse_line<T1, T2>(line: &str) -> Result<(T1, T2), ParsingError>
    where
        T1: TryFrom<char, Error=ParsingError>,
        T2: TryFrom<char, Error=ParsingError>,
{
    let mut chars = line.chars();

    let first = chars.next().ok_or_else(|| ParsingError::new("missing first play"))?.try_into()?;
    chars.next().ok_or_else(|| ParsingError::new("missing space between plays"))?;
    let second = chars.next().ok_or_else(|| ParsingError::new("missing second play"))?.try_into()?;

    if let Some(c) = chars.next() {
        return Err(ParsingError::new(&format!("unexpected input char '{}'", c)));
    }

    Ok((first, second))
}

#[derive(Debug)]
pub struct ParsingError {
    message: String,
}

impl ParsingError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string()
        }
    }
}

impl Display for ParsingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parsing error: {}", &self.message)
    }
}

impl Error for ParsingError {}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Play {
    Rock,
    Paper,
    Scissors,
}

impl Play {
    pub fn beats(&self) -> Self {
        match self {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        }
    }

    pub fn defeated_by(&self) -> Self {
        self.beats().beats()
    }

    pub fn versus(&self, other: &Play) -> Outcome {
        if self == other {
            Draw
        } else if &self.beats() == other {
            Win
        } else {
            Lose
        }
    }

    pub fn as_score(&self) -> u32 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
}

impl TryFrom<char> for Play {
    type Error = ParsingError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            'A' | 'X' => Rock,
            'B' | 'Y' => Paper,
            'C' | 'Z' => Scissors,
            _ => return Err(ParsingError::new(&format!("invalid play '{}'", value)))
        })
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Outcome {
    Lose,
    Draw,
    Win,
}

impl Outcome {
    pub fn as_score(&self) -> u32 {
        match self {
            Lose => 0,
            Draw => 3,
            Win => 6,
        }
    }
}

impl TryFrom<char> for Outcome {
    type Error = ParsingError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            'X' => Outcome::Lose,
            'Y' => Outcome::Draw,
            'Z' => Outcome::Win,
            _ => return Err(ParsingError::new(&format!("invalid outcome '{}'", value)))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{input_generator1, input_generator2, Outcome::*, Play::*, solve_part1, solve_part2};

    static INPUT: &str = "A Y\nB X\nC Z";

    #[test]
    fn input_generator1_builds_vec() {
        let expect = vec![(Rock, Paper), (Paper, Rock), (Scissors, Scissors)];
        assert_eq!(input_generator1(INPUT), expect);
    }

    #[test]
    fn input_generator2_builds_vec() {
        let expect = vec![(Rock, Draw), (Paper, Lose), (Scissors, Win)];
        assert_eq!(input_generator2(INPUT), expect);
    }

    #[test]
    fn solver_part1_match_example() {
        assert_eq!(solve_part1(&input_generator1(INPUT)), 15);
    }

    #[test]
    fn solver_part2_match_example() {
        assert_eq!(solve_part2(&input_generator2(INPUT)), 12);
    }
}
