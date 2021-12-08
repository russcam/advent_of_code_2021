use std::collections::{HashMap, HashSet, VecDeque};
use std::iter::FromIterator;

const INPUT: &str = include_str!("../../input/day_8.txt");

pub struct SignalPatterns {
    outputs: Vec<usize>,
    value: usize,
}

impl From<&str> for SignalPatterns {
    fn from(s: &str) -> Self {
        let mut split = s.splitn(2, " | ");

        let mut inputs = {
            let mut digits: Vec<Digit> = split
                .next()
                .map(|d| d.split(' ')
                    .map(Digit::from))
                .unwrap()
                .collect();
            digits.sort_by(|d1, d2| d1.len.cmp(&d2.len));
            VecDeque::from_iter(digits.into_iter().map(|i| i.hash))
        };

        let outputs: Vec<Vec<char>> = split
            .next()
            .map(|d| d
                .split(' ')
                .map(|dd| {
                    let mut ch: Vec<_> = dd.chars().collect();
                    ch.sort_unstable();
                    ch
                })
                .collect::<Vec<_>>()
            )
            .unwrap();

        let mut digits = vec![None; 10];
        digits[1] = inputs.pop_front();
        digits[7] = inputs.pop_front();
        digits[4] = inputs.pop_front();
        digits[8] = inputs.pop_back();

        // calculate 3
        for i in 0..3 {
            if inputs[i].difference(digits[1].as_ref().unwrap()).count() == 3 {
                digits[3] = Some(inputs.remove(i).unwrap());
                break;
            }
        }

        // [2|5, 5|2, 0|6|9, 6|9|0, 9|0|6];
        // calculate 9
        for i in 2..5 {
            if inputs[i].difference(digits[3].as_ref().unwrap()).count() == 1 {
                digits[9] = Some(inputs.remove(i).unwrap());
                break;
            }
        }

        // calculate 2 and 5
        if inputs[0].difference(digits[9].as_ref().unwrap()).count() == 0 {
            digits[5] = inputs.pop_front();
            digits[2] = inputs.pop_front();
        } else {
            digits[2] = inputs.pop_front();
            digits[5] = inputs.pop_front();
        }

        // calculate 0 and 6
        if inputs[0].difference(digits[5].as_ref().unwrap()).count() == 1 {
            digits[6] = inputs.pop_front();
            digits[0] = inputs.pop_front();
        } else {
            digits[0] = inputs.pop_front();
            digits[6] = inputs.pop_front();
        }

        let map: HashMap<_, _> = digits
            .into_iter()
            .enumerate()
            .map(|(i, h)| {
                let mut v : Vec<char> = h.unwrap().into_iter().collect();
                v.sort_unstable();
                (v, i)
            })
            .collect();

        let outputs: Vec<_> = outputs
            .into_iter()
            .map(|d| *map.get(&d).unwrap())
            .collect();

        let value = outputs
            .iter()
            .rev()
            .enumerate()
            .map(|(i, v)| v * 10usize.pow(i as u32))
            .sum();

        SignalPatterns { outputs, value }
    }
}

pub struct Digit {
    len: usize,
    hash: HashSet<char>
}

impl Digit {
    pub fn from(s: &str) -> Self {
        let hash: HashSet<char> = s.chars().collect();
        let len = s.len();
        Self { len, hash }
    }
}

fn main() {
    let signal_patterns : Vec<SignalPatterns> = INPUT
        .lines()
        .map(SignalPatterns::from)
        .collect();

    let count: usize = signal_patterns
        .iter()
        .map(|p| p.outputs.iter().filter(|v| matches!(v, 1 | 4 | 7 | 8)).count())
        .sum();

    println!("count of 1, 4, 7 and 8 in outputs: {}", count);

    let sum: usize = signal_patterns
        .iter()
        .map(|p| p.value)
        .sum();

    println!("sum of outputs: {}", sum);
}
