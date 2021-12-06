use std::ops::Index;

const INPUT: &str = include_str!("../../input/day_3.txt");

#[derive(Clone)]
struct BinaryVec(Vec<bool>);

impl From<Vec<bool>> for BinaryVec {
    fn from(b: Vec<bool>) -> Self {
        Self(b)
    }
}

impl From<BinaryVec> for usize {
    fn from(b: BinaryVec) -> Self {
        let s: String = b.0.iter().map(|b| if *b { '1' } else { '0' }).collect();
        usize::from_str_radix(s.as_ref(), 2).unwrap()
    }
}

impl Index<usize> for BinaryVec {
    type Output = bool;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl BinaryVec {
    pub fn invert(&mut self) {
        for b in self.0.iter_mut() {
            *b = !*b;
        }
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

struct DiagnosticReport {
    values: Vec<BinaryVec>,
    gamma: Option<usize>,
    epsilon: Option<usize>,
    oxygen: Option<usize>,
    co2: Option<usize>,
}

impl DiagnosticReport {
    pub fn gamma_rate(&mut self) -> usize {
        if self.gamma.is_none() {
            let gamma = self.gamma();
            self.gamma = Some(gamma.into());
        }

        self.gamma.unwrap()
    }

    pub fn epsilon_rate(&mut self) -> usize {
        if self.epsilon.is_none() {
            let mut gamma = self.gamma();
            gamma.invert();
            self.epsilon = Some(gamma.into());
        }

        self.epsilon.unwrap()
    }

    pub fn oxygen_generator_rating(&mut self) -> usize {
        if self.oxygen.is_none() {
            let mut oxygen_generator_report = self.values.clone();
            let len = oxygen_generator_report[0].len();
            for i in 0..len {
                if oxygen_generator_report.len() == 1 {
                    break;
                }

                let count_zeros_ones =
                    Self::count_zeros_ones(&oxygen_generator_report);
                let s = Self::calculate_binary(
                    &count_zeros_ones[i..i + 1],
                    |zeros, ones| ones >= zeros);
                oxygen_generator_report.retain(|r| r[i] == s[0]);
            }

            self.oxygen = Some(oxygen_generator_report[0].clone().into());
        }

        self.oxygen.unwrap()
    }

    pub fn co2_scrubber_rating(&mut self) -> usize {
        if self.co2.is_none() {
            let mut co2_scrubber_report = self.values.clone();
            let len = co2_scrubber_report[0].len();
            for i in 0..len {
                if co2_scrubber_report.len() > 1 {
                    let count_zeros_ones = Self::count_zeros_ones(&co2_scrubber_report);
                    let s = Self::calculate_binary(
                        &count_zeros_ones[i..i + 1],
                        |zeros, ones| !(zeros <= ones));
                    co2_scrubber_report.retain(|r| r[i] == s[0]);
                }
            }

            self.co2 = Some(co2_scrubber_report[0].clone().into());
        }

        self.co2.unwrap()
    }

    fn calculate_binary<F: Fn(usize, usize) -> bool>(counts: &[Vec<usize>], f: F) -> BinaryVec {
        counts.iter().map(|t| f(t[0], t[1])).collect::<Vec<_>>().into()
    }

    fn gamma(&self) -> BinaryVec {
        let count_zeros_ones = Self::count_zeros_ones(&self.values);
        Self::calculate_binary(&count_zeros_ones, |zeros, ones| ones > zeros)
    }

    fn count_zeros_ones(values: &[BinaryVec]) -> Vec<Vec<usize>> {
        let len = values[0].len();
        let mut counts = vec![vec![0; 2]; len];
        for i in 0..len {
            let bools: Vec<bool> = values.iter().map(|f| f[i]).collect();
            for b in bools {
                counts[i][if b { 1 } else { 0 }] += 1;
            }
        }
        counts
    }
}

impl From<&str> for DiagnosticReport {
    fn from(s: &str) -> Self {
        let values: Vec<BinaryVec> = s
            .lines()
            .map(|l| l.chars().map(|c| c == '1').collect::<Vec<_>>().into())
            .collect();

        Self {
            values,
            gamma: None,
            epsilon: None,
            oxygen: None,
            co2: None
        }
    }
}

fn main() {
    let mut diagnostic_report: DiagnosticReport = INPUT.into();

    let gamma = diagnostic_report.gamma_rate();
    let epsilon_rate = diagnostic_report.epsilon_rate();

    println!(
        "gamma: {}, epsilon_rate: {}, gamma * epsilon_rate: {}",
        gamma,
        epsilon_rate,
        gamma * epsilon_rate
    );

    let oxygen_generator_rating = diagnostic_report.oxygen_generator_rating();
    let co2_scrubber_rating = diagnostic_report.co2_scrubber_rating();

    println!("oxygen_generator_rating: {}, co2_scrubber_rating: {}, oxygen_generator_rating * co2_scrubber_rating: {}",
             oxygen_generator_rating, co2_scrubber_rating, oxygen_generator_rating * co2_scrubber_rating);
}