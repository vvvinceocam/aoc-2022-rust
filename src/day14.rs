use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

use crate::utils::Point;

fn range(from: &Point<usize>, to: &Point<usize>) -> Vec<Point<usize>> {
    if from.x < to.x {
        (from.x..=to.x).zip([from.y].iter().cycle()).map(|(x, y)| Point { x, y: *y }).collect()
    } else if from.x > to.x {
        (to.x..=from.x).zip([from.y].iter().cycle()).map(|(x, y)| Point { x, y: *y }).collect()
    } else if from.y < to.y {
        [from.x].iter().cycle().zip(from.y..=to.y).map(|(x, y)| Point { x: *x, y }).collect()
    } else if from.y > to.y {
        [from.x].iter().cycle().zip(to.y..=from.y).map(|(x, y)| Point { x: *x, y }).collect()
    } else {
        [from.clone()].into_iter().collect()
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Space {
    limit: usize,
    blocks: HashSet<Point<usize>>,
}

impl Space {
    fn new(blocks: HashSet<Point<usize>>) -> Self {
        let limit = dbg!(blocks.iter().map(|p| p.y).max().unwrap());

        Self {
            limit,
            blocks,
        }
    }
}

#[aoc_generator(day14)]
fn input_generator(input: &str) -> Space {
    let blocks = input.lines()
        .flat_map(|line|
            line.split(" -> ")
                .map(|point| point.parse().unwrap())
                .collect::<Vec<Point<usize>>>()
                .windows(2)
                .map(|ab| range(&ab[0], &ab[1]))
                .collect::<Vec<_>>()
        )
        .flatten()
        .collect();
    Space::new(blocks)
}

#[aoc(day14, part1)]
fn solve_part1(space: &Space) -> usize {
    let mut space = space.clone();
    let mut count = 0;

    loop {
        let mut block = Point { x: 500, y: 0 };
        count += 1;

        loop {
            if block.y > space.limit {
                return count - 1;
            }

            if !space.blocks.contains(&Point { x: block.x, y: block.y + 1 }) {
                block.y += 1;
            } else if !space.blocks.contains(&Point { x: block.x - 1, y: block.y + 1 }) {
                block.x -= 1;
                block.y += 1;
            } else if !space.blocks.contains(&Point { x: block.x + 1, y: block.y + 1 }) {
                block.x += 1;
                block.y += 1;
            } else {
                space.blocks.insert(block.clone());
                break;
            }
        }
    }
}

#[aoc(day14, part2)]
fn solve_part2(space: &Space) -> usize {
    let mut space = space.clone();
    let mut count = 0;

    loop {
        let mut block = Point { x: 500, y: 0 };
        count += 1;

        loop {
            if block.y == space.limit + 1 {
                space.blocks.insert(block.clone());
                break;
            } else if !space.blocks.contains(&Point { x: block.x, y: block.y + 1 }) {
                block.y += 1;
            } else if !space.blocks.contains(&Point { x: block.x - 1, y: block.y + 1 }) {
                block.x -= 1;
                block.y += 1;
            } else if !space.blocks.contains(&Point { x: block.x + 1, y: block.y + 1 }) {
                block.x += 1;
                block.y += 1;
            } else if block.y == 0 {
                return count;
            } else {
                space.blocks.insert(block.clone());
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{input_generator, solve_part1, solve_part2, Space, Point};

    static INPUT: &str = "\
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn solver_part1_match_example() {
        assert_eq!(solve_part1(&input_generator(INPUT)), 24);
    }

    #[test]
    fn solver_part2_match_example() {
        assert_eq!(solve_part2(&input_generator(INPUT)), 93);
    }
}


