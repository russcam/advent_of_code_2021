use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../../input/day_12.txt");

type Edge<'a> = (&'a str, &'a str);

#[derive(Debug)]
struct Graph<'a> {
    caves: HashMap<&'a str, Vec<&'a str>>,
}

impl<'a> Graph<'a> {
    pub fn new() -> Self {
        Self {
            caves: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, edge: Edge<'a>) {
        self.add(edge.0, edge.1);
        self.add(edge.1, edge.0);
    }

    fn add(&mut self, src: &'a str, dest: &'a str) {
        self.caves.entry(src).or_insert_with(Vec::new).push(dest);
    }

    pub fn part_1(&self) -> Vec<Vec<&'a str>> {
        let mut paths = Vec::new();
        self.walk("start", HashSet::new(), &mut paths, Vec::new(), true);
        paths
    }

    pub fn part_2(&self) -> Vec<Vec<&'a str>> {
        let mut paths = Vec::new();
        self.walk("start", HashSet::new(), &mut paths, Vec::new(), false);
        paths
    }

    // could make this faster by summing completed paths rather than collecting the actual paths.
    fn walk(
        &self,
        cave: &'a str,
        mut seen: HashSet<&'a str>,
        paths: &mut Vec<Vec<&'a str>>,
        mut path: Vec<&'a str>,
        mut seen_twice: bool,
    ) {
        if seen.contains(cave) {
            if seen_twice {
                return;
            } else {
                seen_twice = true;
            }
        }

        path.push(cave);
        if cave == "end" {
            paths.push(path.clone());
            return;
        }

        // track small caves we've seen
        if cave.starts_with(char::is_lowercase) {
            seen.insert(cave);
        }

        for neighbour in self.caves[cave].iter().filter(|n| **n != "start") {
            self.walk(neighbour, seen.clone(), paths, path.clone(), seen_twice);
        }
    }
}

fn main() {
    let edges = INPUT
        .lines()
        .map(|l| l.split_once('-').unwrap())
        .collect::<Vec<_>>();

    let mut graph = Graph::new();
    for edge in edges {
        graph.add_edge(edge);
    }

    let mut paths = graph.part_1();
    println!("paths with small caves at most once: {}", paths.len());

    paths = graph.part_2();
    println!("paths with one small cave at most twice: {}", paths.len());
}
