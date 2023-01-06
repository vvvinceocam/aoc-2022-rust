use aoc_runner_derive::{aoc, aoc_generator};

struct Map {
    cells: Vec<Vec<u8>>,
}

impl Map {
    #[inline]
    pub fn width(&self) -> usize {
        self.cells[0].len()
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.cells.len()
    }

    pub fn inner_cells(&self) -> impl Iterator<Item=Cell> {
        (1..self.width() - 1).flat_map(move |x| (1..self.height() - 1).map(move |y| Cell { map: self, x, y }))
    }
}

struct Cell<'a> {
    map: &'a Map,
    x: usize,
    y: usize,
}

impl<'a> Cell<'a> {
    #[inline]
    pub fn value(&self) -> u8 {
        self.map.cells[self.y][self.x]
    }

    pub fn paths(&self) -> Vec<Vec<u8>> {
        let width = self.map.width();
        let height = self.map.height();
        vec![
            (0..self.x).rev().map(|x| self.map.cells[self.y][x]).collect(),
            (self.x + 1..width).map(|x| self.map.cells[self.y][x]).collect(),
            (0..self.y).rev().map(|y| self.map.cells[y][self.x]).collect(),
            (self.y + 1..height).map(|y| self.map.cells[y][self.x]).collect(),
        ]
    }
}

#[aoc_generator(day8)]
fn input_generator(input: &str) -> Map {
    Map {
        cells: input.lines().map(|line| line.bytes().map(|b| b - b'0').collect()).collect()
    }
}

#[aoc(day8, part1)]
fn solve_part1(map: &Map) -> usize {
    let edges = 2 * (map.width() + map.height()) - 4;
    edges + map.inner_cells().filter(|cell| {
        cell.paths().iter().filter(|path| path.iter().all(|step| *step < cell.value())).count() > 0
    }).count()
}

#[aoc(day8, part2)]
fn solve_part2(map: &Map) -> usize {
    map.inner_cells().map(|cell| {
        cell.paths().iter().map(|path| {
            path.split_inclusive(|step| *step >= cell.value()).next().map(|slice| slice.len()).unwrap_or(0)
        }).product()
    }).max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::{input_generator, solve_part1, solve_part2};

    static INPUT: &str = "\
30373
25512
65332
33549
35390";

    #[test]
    fn input_generator_builds_vec() {
        let expected = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];
        assert_eq!(input_generator(INPUT).cells, expected);
    }

    #[test]
    fn solver_part1_match_example() {
        assert_eq!(solve_part1(&input_generator(INPUT)), 21);
    }

    #[test]
    fn solver_part2_match_example() {
        assert_eq!(solve_part2(&input_generator(INPUT)), 8);
    }
}
