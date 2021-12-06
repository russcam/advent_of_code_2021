const INPUT: &str = include_str!("../../input/day_6.txt");

struct Ages {
    counts: [usize; 9],
    day: usize,
}

impl Ages {
    pub fn from_fishes(fishes: &[u8]) -> Self {
        let mut counts = [0usize; 9];

        for fish in fishes {
            let idx = *fish as usize;
            counts[idx] += 1;
        }

        Self { counts, day: 0 }
    }

    pub fn advance_days(&mut self, days: usize) {
        for _ in 0..days {
            let mut new = 0;
            let mut new_counts = [0usize; 9];
            for age in 0..self.counts.len() {
                let count = self.counts[age];
                if age == 0 {
                    new_counts[6] += count;
                    new += count;
                } else {
                    new_counts[age - 1] += count;
                }
            }
            new_counts[8] = new;
            self.counts = new_counts;
        }
        self.day += days;
    }

    pub fn count(&self) -> usize {
        self.counts.iter().sum()
    }
}

fn main() {
    let fishes: Vec<u8> = INPUT.split(',').map(|s| s.parse().unwrap()).collect();
    let mut ages = Ages::from_fishes(&fishes);

    ages.advance_days(80);

    println!("fish after 80 days: {}", ages.count());

    ages.advance_days(256 - 80);

    println!("fish after 256 days: {}", ages.count());
}
