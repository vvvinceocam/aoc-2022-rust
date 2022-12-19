use std::collections::HashMap;
use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Eq, PartialEq)]
enum TerminalLine {
    ChangeDirectory { name: String },
    List,
    File {
        name: String,
        size: usize,
    },
    Directory {
        name: String,
    },
}

impl FromStr for TerminalLine {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TerminalLine::*;
        if let Some(folder) = s.strip_prefix("$ cd ") {
            Ok(ChangeDirectory { name: folder.to_string() })
        } else if s == "$ ls" {
            Ok(List)
        } else if let Some(folder) = s.strip_prefix("dir ") {
            Ok(Directory { name: folder.to_string() })
        } else {
            let (size, name) = s.split_once(' ').unwrap();
            Ok(
                File {
                    name: name.to_string(),
                    size: size.parse().unwrap(),
                }
            )
        }
    }
}

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
    fn insert(&mut self, name: String, node: usize) {
        match self {
            Self::Directory { children, } => {
                children.insert(name, node);
            }
            _ => {
                panic!("Can't insert on File")
            }
        }
    }

    fn get(&self, name: &String) -> usize {
        match self {
            Self::Directory { children, } => {
                *children.get(name).unwrap()
            }
            _ => {
                panic!("Can't insert on File")
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct FileSystem {
    nodes: Vec<Node>,
}

impl FileSystem {
    pub fn from_terminal_lines<I>(lines: I) -> Self
        where
            I: Iterator<Item=TerminalLine>
    {
        let mut nodes = Vec::new();

        nodes.push(
            Node::Directory {
                children: HashMap::new(),
            }
        );

        let mut parents: Vec<usize> = vec![];

        for line in lines {
            use TerminalLine::*;

            match line {
                List => {}
                ChangeDirectory { name } => {
                    match name.as_str() {
                        "/" => {
                            parents.clear();
                        }
                        ".." => {
                            parents.pop();
                        }
                        _ => {
                            parents.push(nodes[*parents.last().unwrap_or(&0)].get(&name));
                        }
                    };
                }
                Directory { .. } | File { .. } => {
                    let (name, node) = match line {
                        Directory { name } => (
                            name,
                            Node::Directory {
                                children: HashMap::new(),
                            }),
                        File { name, size } => (
                            name,
                            Node::File { size }
                        ),
                        _ => unreachable!(),
                    };
                    nodes.push(node);
                    let id = nodes.len() - 1;
                    nodes.get_mut(*parents.last().unwrap_or(&0)).unwrap().insert(name, id);
                }
            }
        }

        Self {
            nodes
        }
    }

    fn traverse_dirs(&self, node: usize)  -> (usize, usize)
    {
        match self.nodes.get(node).unwrap() {
            Node::File {size} => (0, *size),
            Node::Directory {children} => {
                let mut acc = 0;
                let mut size = 0;
                for index in children.values() {
                    let (subacc, subsize) = self.traverse_dirs(*index);
                    acc += subacc;
                    size += subsize;
                }
                if size <= 100_000 {
                    acc += size;
                }
                (acc, size)
            }
        }
    }


    fn select_folder(&self, node: usize, threshold: usize)  -> (usize, usize)
    {
        match self.nodes.get(node).unwrap() {
            Node::File {size} => (usize::MAX, *size),
            Node::Directory {children} => {
                let mut size = 0;
                let mut acc = usize::MAX;
                for index in children.values() {
                    let (subacc, subsize) = self.select_folder(*index, threshold);
                    size += subsize;
                    if subacc < acc && subacc >= threshold {
                        acc = subacc;
                    }
                }
                if size >= threshold && size < acc {
                    acc = size;
                }
                dbg!((acc, size))
            }
        }
    }
}

#[aoc_generator(day7)]
fn input_generator(input: &str) -> FileSystem {
    FileSystem::from_terminal_lines(input.lines().map(|line| line.parse().unwrap()))
}

#[aoc(day7, part1)]
fn solve_part1(fs: &FileSystem) -> usize {
    fs.traverse_dirs(0).0
}

#[aoc(day7, part2)]
fn solve_part2(fs: &FileSystem) -> usize {
    let total = fs.traverse_dirs(0).1;
    let threshold = 30_000_000 - (70_000_000 - total);
    fs.select_folder(0, threshold).0
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

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
