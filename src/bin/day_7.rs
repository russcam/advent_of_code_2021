const INPUT: &str = include_str!("../../input/day_7.txt");

struct Positions {
    values: Vec<i32>,
    sorted: bool,
}

struct Outcome {
    position: i32,
    fuel: i32,
}

impl Positions {
    pub fn new(values: Vec<i32>) -> Self {
        Self {
            values,
            sorted: false,
        }
    }

    fn median(&mut self) -> i32 {
        if !self.sorted {
            self.values.sort_unstable();
            self.sorted = true;
        }

        let mid = self.values.len() / 2;
        if self.values.len() % 2 == 0 {
            Self::calc_mean(&self.values[(mid - 1)..=mid]) as i32
        } else {
            self.values[mid]
        }
    }

    fn mean(&self) -> f32 {
        Self::calc_mean(&self.values)
    }

    fn calc_mean(positions: &[i32]) -> f32 {
        let sum: i32 = positions.iter().sum();
        sum as f32 / positions.len() as f32
    }

    fn seq_sum(number: i32) -> i32 {
        (number * (number + 1)) / 2
    }

    pub fn part_1(&mut self) -> Outcome {
        let position = self.median();
        let fuel: i32 = self.values.iter().map(|p| (p - position).abs()).sum();
        Outcome { position, fuel }
    }

    pub fn part_2(&mut self) -> Outcome {
        let position = self.mean() as i32;
        let mut fuel = 0;
        for value in &self.values {
            let diff = (position - value).abs();
            fuel += Self::seq_sum(diff)
        }

        Outcome { position, fuel }
    }
}

fn main() {
    let values: Vec<i32> = INPUT.split(',').map(|s| s.parse().unwrap()).collect();

    let mut positions = Positions::new(values);

    let part_1 = positions.part_1();
    println!(
        "part 1 position: {}, fuel: {}",
        part_1.position, part_1.fuel
    );

    let part_2 = positions.part_2();
    println!(
        "part 2 position: {}, fuel: {}",
        part_2.position, part_2.fuel
    );
}
