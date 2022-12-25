use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Eq, PartialEq)]
enum Node {
    File {
        size: usize,
    },
    Directory {
        children: HashMap<String, usize>,
    },
}

impl Node {
    fn get(&self, name: &str) -> usize {
        match self {
            Self::Directory { children, } => {
                *children.get(name).unwrap()
            }
            _ => {
                panic!("Can't get node")
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct FileSystem {
    nodes: Vec<Node>,
}

impl FileSystem {
    fn insert_node(&mut self, node: Node) -> usize {
        self.nodes.push(node);
        self.nodes.len() - 1
    }

    fn link(&mut self, parent_id: usize, name: String, id: usize) {
        if let Node::Directory { ref mut children } = self.nodes.get_mut(parent_id).expect("missing node") {
            children.insert(name, id);
        } else {
            panic!("not a dir");
        }
    }

    pub fn create_dir(&mut self, parent_id: usize, name: &str) -> usize {
        let dir = Node::Directory {
            children: HashMap::new(),
        };
        let id = self.insert_node(dir);
        self.link(parent_id, name.to_string(), id);
        id
    }

    pub fn create_file(&mut self, parent_id: usize, name: &str, size: usize) -> usize {
        let file = Node::File {
            size
        };
        let id = self.insert_node(file);
        self.link(parent_id, name.to_string(), id);
        id
    }

    pub fn new() -> Self {
        Self {
            nodes: vec![
                Node::Directory {
                    children: HashMap::new(),
                }
            ]
        }
    }

    pub fn from_terminal_output<'a, I>(lines: I) -> Self
        where
            I: Iterator<Item=&'a str>
    {
        let mut fs = FileSystem::new();
        let mut path: Vec<usize> = vec![];
        let mut parent = 0;

        for line in lines {
            match line.split_whitespace().collect::<Vec<_>>().as_slice() {
                ["$", "cd", "/"] => {
                    path.clear();
                    parent = 0;
                }
                ["$", "cd", ".."] => {
                    parent = path.pop().unwrap();
                }
                ["$", "cd", name] => {
                    path.push(parent);
                    parent = fs.nodes[parent].get(name);
                }
                ["$", "ls"] => {
                    // do nothing
                }
                ["dir", name] => {
                    fs.create_dir(parent, name);
                }
                [size, name] => {
                    let size = size.parse().unwrap();
                    fs.create_file(parent, name, size);
                }
                _ => {
                    unreachable!();
                }
            }
        }

        fs
    }

    pub fn bottom_up_traversal<F1, F2, T>(&self, node: usize, on_file: &F1, on_dir: &F2) -> T
        where
            F1: Fn(usize) -> T,
            F2: Fn(Vec<T>) -> T,
    {
        match self.nodes.get(node).unwrap() {
            Node::File { size } => {
                on_file(*size)
            }
            Node::Directory { children } => {
                on_dir(children.values().map(|node| self.bottom_up_traversal(*node, on_file, on_dir)).collect())
            }
        }
    }
}

#[aoc_generator(day7)]
fn input_generator(input: &str) -> FileSystem {
    FileSystem::from_terminal_output(input.lines())
}

#[aoc(day7, part1)]
fn solve_part1(fs: &FileSystem) -> usize {
    let on_file = |size| (0, size);
    let on_dir = |children: Vec<(usize, usize)>| {
        let (acc, size) = children.iter().fold((0, 0), |acc, child| (acc.0 + child.0, acc.1 + child.1));
        if size <= 100_000 {
            (acc + size, size)
        } else {
            (acc, size)
        }
    };
    fs.bottom_up_traversal(0, &on_file, &on_dir).0
}

#[aoc(day7, part2)]
fn solve_part2(fs: &FileSystem) -> usize {
    let total = fs.bottom_up_traversal(0, &|size| size, &|children| children.iter().sum());
    let threshold = 30_000_000 - (70_000_000 - total);

    let on_file = |size| (0, size);
    let on_dir = |children: Vec<_>| {
        let size = children.iter().map(|(_, size)| size).sum();
        let candidate = *children.iter()
            .map(|(candidate, _)| candidate)
            .chain(&[size])
            .filter(|candidate| **candidate >= threshold)
            .min()
            .unwrap_or(&usize::MAX);
        (candidate, size)
    };
    fs.bottom_up_traversal(0, &on_file, &on_dir).0
}

#[cfg(test)]
mod tests {
    use super::{FileSystem, Node};
    use super::{input_generator, solve_part1, solve_part2};

    static INPUT: &str = "\
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn input_generator_builds_vec() {
        use Node::*;
        let expect = FileSystem {
            nodes: vec![
                Directory {
                    children: [
                        ("a".to_string(), 1),
                        ("b.txt".to_string(), 2),
                        ("c.dat".to_string(), 3),
                        ("d".to_string(), 4),
                    ].into_iter().collect()
                },
                Directory {
                    children: [
                        ("e".to_string(), 5),
                        ("f".to_string(), 6),
                        ("g".to_string(), 7),
                        ("h.lst".to_string(), 8),
                    ].into_iter().collect()
                },
                File { size: 14848514 },
                File { size: 8504156 },
                Directory {
                    children: [
                        ("j".to_string(), 10),
                        ("d.log".to_string(), 11),
                        ("d.ext".to_string(), 12),
                        ("k".to_string(), 13),
                    ].into_iter().collect()
                },
                Directory {
                    children: [
                        ("i".to_string(), 9),
                    ].into_iter().collect()
                },
                File { size: 29116 },
                File { size: 2557 },
                File { size: 62596 },
                File { size: 584 },
                File { size: 4060174 },
                File { size: 8033020 },
                File { size: 5626152 },
                File { size: 7214296 },
            ]
        };

        assert_eq!(input_generator(INPUT), expect);
    }

    #[test]
    fn solver_part1_match_example() {
        assert_eq!(solve_part1(&input_generator(INPUT)), 95437);
    }

    #[test]
    fn solver_part2_match_example() {
        assert_eq!(solve_part2(&input_generator(INPUT)), 24933642);
    }
}
