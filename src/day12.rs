use std::cmp::Ordering;
use std::collections::{BinaryHeap, BTreeMap};

use aoc_runner_derive::{aoc, aoc_generator};

type Cell = (usize, usize);

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    cell: Cell,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.cell.cmp(&other.cell))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Map {
    width: usize,
    height: usize,
    inner: Vec<Vec<u8>>,
}

impl Map {
    fn new(data: Vec<Vec<u8>>) -> Self {
        Self {
            width: data[0].len(),
            height: data.len(),
            inner: data,
        }
    }

    #[inline]
    fn get_cell(&self, (x, y): Cell) -> u8 {
        self.inner[y][x]
    }

    fn get_neighbors(&self, cell @ (x, y): Cell) -> Vec<Cell> {
        let mut neighbors = vec![];
        if x > 0 {
            neighbors.push((x - 1, y));
        }
        if y > 0 {
            neighbors.push((x, y - 1));
        }
        if x < self.width - 1 {
            neighbors.push((x + 1, y));
        }
        if y < self.height - 1 {
            neighbors.push((x, y + 1));
        }
        let value = self.get_cell(cell);
        neighbors.into_iter()
            .filter(|neighbor| reachable(value, self.get_cell(*neighbor)))
            .collect()
    }

    fn find_cells_by_value(&self, expected_value: u8) -> Vec<Cell> {
        self.inner.iter()
            .enumerate()
            .flat_map(|(y, row)|
                row.iter()
                    .enumerate()
                    .map(move |(x, value)| ((x, y), value)))
            .filter(|(_, value)| **value == expected_value)
            .map(|(cell, _)| cell)
            .collect()
    }

    fn shortest_path(&self, from: Cell, to: Cell) -> Option<usize> {
        let mut dist = BTreeMap::<Cell, usize>::new();
        let mut heap = BinaryHeap::new();

        dist.insert(from, 0);
        heap.push(State { cost: 0, cell: from });

        while let Some(State { cost, cell }) = heap.pop() {
            if cell == to {
                return Some(cost);
            }

            if cost > dist[&cell] {
                continue;
            }

            for neighbor in self.get_neighbors(cell) {
                let next = State { cost: 1 + cost, cell: neighbor };
                if next.cost < *dist.get(&next.cell).unwrap_or(&usize::MAX) {
                    heap.push(next);
                    dist.insert(next.cell, next.cost);
                }
            }
        }

        None
    }
}

#[inline]
fn reachable(from: u8, to: u8) -> bool {
    match (from, to) {
        (b'S', to) => to == b'a',
        (from, b'E') => from == b'z',
        (from, to) if from < to => to - from == 1,
        _ => true,
    }
}

#[aoc_generator(day12)]
fn input_generator(input: &str) -> Map {
    let data = input
        .lines()
        .map(|line| line.bytes().collect())
        .collect();
    Map::new(data)
}

#[aoc(day12, part1)]
fn solve_part1(map: &Map) -> usize {
    let start = *map.find_cells_by_value(b'S').first().unwrap();
    let end = *map.find_cells_by_value(b'E').first().unwrap();
    map.shortest_path(start, end).unwrap()
}

#[aoc(day12, part2)]
fn solve_part2(map: &Map) -> usize {
    let start = map.find_cells_by_value(b'S');
    let lowers = map.find_cells_by_value(b'a');
    let end = *map.find_cells_by_value(b'E').first().unwrap();

    start.iter().chain(lowers.iter()).flat_map(move |start| map.shortest_path(*start, end)).min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::{input_generator, solve_part1, solve_part2};

    static INPUT: &str = "\
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
";

    #[test]
    fn solver_part1_match_example() {
        assert_eq!(solve_part1(&input_generator(INPUT)), 31);
    }

    #[test]
    fn solver_part2_match_example() {
        assert_eq!(solve_part2(&input_generator(INPUT)), 29);
    }
}
