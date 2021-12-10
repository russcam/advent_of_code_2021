use once_cell::sync::Lazy;
use std::collections::HashMap;

const INPUT: &str = include_str!("../../input/day_10.txt");

#[derive(Debug)]
struct IllegalOutput {
    actual: char,
}

impl IllegalOutput {
    pub fn error_score(&self) -> usize {
        match self.actual {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => unreachable!(),
        }
    }
}

static PAIR: Lazy<HashMap<char, char>> = Lazy::new(|| {
    let mut h = HashMap::new();
    h.insert('(', ')');
    h.insert('[', ']');
    h.insert('{', '}');
    h.insert('<', '>');
    h
});

#[derive(Debug)]
struct SyntaxChecker<'a> {
    line: &'a str,
}

impl<'a> SyntaxChecker<'a> {
    pub fn is_illegal(&self) -> Option<IllegalOutput> {
        let mut opens = Vec::with_capacity(self.line.len());
        for actual in self.line.chars() {
            match actual {
                '(' | '[' | '{' | '<' => opens.push(actual),
                ')' | ']' | '}' | '>' => {
                    if let Some(o) = opens.pop() {
                        let expected = PAIR[&o];
                        if expected != actual {
                            return Some(IllegalOutput { actual });
                        }
                    }
                }
                _ => unreachable!(),
            }
        }

        None
    }

    pub fn completion_score(&self) -> usize {
        let mut opens = Vec::with_capacity(self.line.len());
        for actual in self.line.chars() {
            match actual {
                '(' | '[' | '{' | '<' => opens.push(PAIR[&actual]),
                ')' | ']' | '}' | '>' => {
                    let _ = opens.pop();
                }
                _ => unreachable!(),
            }
        }

        opens.into_iter().rev().fold(0, |acc, c| {
            let point = match c {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => unreachable!(),
            };
            (5 * acc) + point
        })
    }
}

impl<'a> From<&'a str> for SyntaxChecker<'a> {
    fn from(line: &'a str) -> Self {
        Self { line }
    }
}

fn main() {
    let checkers = INPUT.lines().map(SyntaxChecker::from).collect::<Vec<_>>();

    let (illegal, incomplete): (Vec<_>, Vec<_>) =
        checkers.iter().partition(|c| c.is_illegal().is_some());

    let sum = illegal
        .iter()
        .map(|o| o.is_illegal().unwrap().error_score())
        .sum::<usize>();

    println!("sum of illegal lines: {}", sum);

    let mut completion_scores = incomplete
        .iter()
        .map(|o| o.completion_score())
        .collect::<Vec<_>>();

    completion_scores.sort_unstable();
    let middle = completion_scores.len() / 2;

    println!(
        "middle score of incomplete lines: {}",
        completion_scores[middle]
    );
}
