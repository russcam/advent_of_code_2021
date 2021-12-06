use std::collections::VecDeque;

const INPUT: &str = include_str!("../../input/day_6.txt");

struct Ages {
    counts: VecDeque<usize>,
    day: usize,
}

impl Ages {
    pub fn from_fishes<I: Iterator<Item = usize>>(fishes: I) -> Self {
        let mut counts = VecDeque::from(vec![0; 9]);
        for fish in fishes {
            counts[fish] += 1;
        }
        Self { counts, day: 0 }
    }

    pub fn advance_days(&mut self, days: usize) {
        for _ in 0..days {
            if let Some(spawned) = self.counts.pop_front() {
                self.counts[6] += spawned;
                self.counts.push_back(spawned);
            }
        }
        self.day += days;
    }

    pub fn count(&self) -> usize {
        self.counts.iter().sum()
    }
}

fn main() {
    let mut ages = Ages::from_fishes(INPUT.split(',').map(|s| s.parse().unwrap()));

    ages.advance_days(80);

    println!("fish after 80 days: {}", ages.count());

    ages.advance_days(256 - 80);

    println!("fish after 256 days: {}", ages.count());
}
