use std::collections::{BTreeMap, HashMap};

const INPUT: &str = include_str!("../../input/day_14.txt");

struct Polymer {
    end_chars: (char, char),
    pairs: HashMap<(char, char), usize>,
    rules: HashMap<(char, char), char>,
}

impl Polymer {
    pub fn new(
        end_chars: (char, char),
        pairs: HashMap<(char, char), usize>,
        rules: HashMap<(char, char), char>,
    ) -> Self {
        Self {
            end_chars,
            pairs,
            rules,
        }
    }

    pub fn process(&mut self) {
        let pairs = self.pairs.drain();
        let mut new_pairs = HashMap::new();
        for (pair @ (a, b), count) in pairs {
            match self.rules.get(&pair) {
                Some(c) => {
                    *new_pairs.entry((a, *c)).or_insert(0) += count;
                    *new_pairs.entry((*c, b)).or_insert(0) += count;
                }
                None => *new_pairs.entry(pair).or_insert(0) += count,
            }
        }
        self.pairs = new_pairs;
    }

    pub fn output(&self) -> usize {
        let mut counts = BTreeMap::new();
        for ((a, b), count) in &self.pairs {
            *counts.entry(*a).or_insert(0) += count;
            *counts.entry(*b).or_insert(0) += count;
        }

        // all chars will be doubled *except* the first and last char of the original template.
        // add these in
        *counts.entry(self.end_chars.0).or_insert(0) += 1;
        *counts.entry(self.end_chars.1).or_insert(0) += 1;
        (counts.values().max().unwrap() - counts.values().min().unwrap()) / 2
    }
}

fn main() {
    let mut lines = INPUT.lines();
    let mut template = lines.next().unwrap().chars();
    let pairs = {
        let mut h = HashMap::new();
        for c in template
            .clone()
            .collect::<Vec<_>>()
            .windows(2)
            .map(|w| (w[0], w[1]))
        {
            *h.entry(c).or_insert(0) += 1;
        }
        h
    };

    let rules: HashMap<_, _> = lines
        .skip(1)
        .map(|l| {
            let (input, output) = l.split_once(" -> ").unwrap();
            let mut input_chars = input.chars();
            (
                (input_chars.next().unwrap(), input_chars.next().unwrap()),
                output.chars().next().unwrap(),
            )
        })
        .collect();

    let end_chars = (template.next().unwrap(), template.last().unwrap());

    let mut polymer = Polymer::new(end_chars, pairs, rules);
    for _ in 0..10 {
        polymer.process();
    }

    println!(
        "most common element - least common element after 10 iterations: {}",
        polymer.output()
    );

    for _ in 0..30 {
        polymer.process();
    }

    println!(
        "most common element - least common element after 40 iterations: {}",
        polymer.output()
    );
}
