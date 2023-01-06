use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Move {
    Up,
    Right,
    Down,
    Left,
}

impl Move {
    fn from_str(char: &str) -> Self {
        match char {
            "U" => Move::Up,
            "R" => Move::Right,
            "D" => Move::Down,
            "L" => Move::Left,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Copy, Ord, PartialOrd)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    #[inline]
    fn distance(&self, other: &Position) -> usize {
        usize::max(self.x.abs_diff(other.x), self.y.abs_diff(other.y))
    }

    fn apply_move(&mut self, step: Move) {
        match step {
            Move::Up => self.y -= 1,
            Move::Right => self.x += 1,
            Move::Down => self.y += 1,
            Move::Left => self.x -= 1,
        }
    }

    fn follow(&mut self, previous: &Position) {
        if self.distance(previous) > 1 {
            self.x += (previous.x - self.x).signum();
            self.y += (previous.y - self.y).signum();
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct State {
    knots: Vec<Position>,
}

impl State {
    fn new(knots_count: usize) -> Self {
        Self {
            knots: vec![Position { x: 0, y: 0 }; knots_count]
        }
    }

    fn move_knots(&mut self, step: Move) {
        self.knots[0].apply_move(step);
        for i in 1..self.knots.len() {
            let previous = self.knots[i - 1];
            self.knots[i].follow(&previous);
        }
    }
}

fn follow(knots_count: usize, moves: &[Move]) -> usize {
    let mut positions = moves.iter().scan(State::new(knots_count), |state, step| {
        state.move_knots(*step);
        Some(state.clone())
    })
        .map(|state| *state.knots.last().unwrap())
        .collect::<Vec<_>>();
    positions.sort();
    positions.dedup();
    positions.len()
}

#[aoc_generator(day9)]
fn input_generator(input: &str) -> Vec<Move> {
    input.lines().flat_map(|line| {
        let mut elems = line.split_whitespace();
        vec![Move::from_str(elems.next().unwrap()); elems.next().unwrap().parse().unwrap()]
    }).collect()
}

#[aoc(day9, part1)]
fn solve_part1(moves: &[Move]) -> usize {
    follow(2, moves)
}

#[aoc(day9, part2)]
fn solve_part2(moves: &[Move]) -> usize {
    follow(10, moves)
}

#[cfg(test)]
mod tests {
    use crate::day9::Move;

    use super::{input_generator, solve_part1, solve_part2};

    static INPUT1: &str = "\
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    static INPUT2: &str = "\
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn input_generator_builds_vec() {
        let expected = vec![
            Move::Right,
            Move::Right,
            Move::Right,
            Move::Right,
            Move::Up,
            Move::Up,
            Move::Up,
            Move::Up,
            Move::Left,
            Move::Left,
            Move::Left,
            Move::Down,
            Move::Right,
            Move::Right,
            Move::Right,
            Move::Right,
            Move::Down,
            Move::Left,
            Move::Left,
            Move::Left,
            Move::Left,
            Move::Left,
            Move::Right,
            Move::Right,
        ];
        assert_eq!(input_generator(INPUT1), expected);
    }

    #[test]
    fn solver_part1_match_example() {
        assert_eq!(solve_part1(&input_generator(INPUT1)), 13);
    }

    #[test]
    fn solver_part2_match_example() {
        assert_eq!(solve_part2(&input_generator(INPUT2)), 36);
    }
}
