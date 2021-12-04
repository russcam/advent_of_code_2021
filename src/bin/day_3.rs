const INPUT: &str = include_str!("../../input/day_3.txt");

fn main() {
    let diagnostic_report: Vec<Vec<usize>> = INPUT
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(2).unwrap() as usize).collect())
        .collect();

    let counts = count_zeros_ones(&diagnostic_report);

    let gamma_string: String =
        calculate_binary(&counts, |zeros, ones| if zeros > ones { '0' } else { '1' })
            .iter()
            .collect();

    let gamma = usize_from_binary(&gamma_string);
    let epsilon_rate = usize_from_binary(invert(&gamma_string));

    println!(
        "gamma: {}, epsilon_rate: {}, gamma * epsilon_rate: {}",
        gamma,
        epsilon_rate,
        gamma * epsilon_rate
    );

    let mut oxygen_generator_report = diagnostic_report.clone();
    let mut co2_scrubber_report = diagnostic_report.clone();

    let len = diagnostic_report[0].len();
    for i in 0..len {
        if oxygen_generator_report.len() > 1 {
            let oxygen_counts = count_zeros_ones(&oxygen_generator_report);
            let s = calculate_binary(
                &oxygen_counts[i..i + 1],
                |zeros, ones| {
                    if ones >= zeros {
                        1
                    } else {
                        0
                    }
                },
            );
            oxygen_generator_report.retain(|r| r[i] == s[0]);
        }

        if co2_scrubber_report.len() > 1 {
            let co2_counts = count_zeros_ones(&co2_scrubber_report);
            let s = calculate_binary(
                &co2_counts[i..i + 1],
                |zeros, ones| {
                    if zeros <= ones {
                        0
                    } else {
                        1
                    }
                },
            );
            co2_scrubber_report.retain(|r| r[i] == s[0]);
        }
    }

    let oxygen_generator_rating = usize_from_slice(&oxygen_generator_report[0]);
    let co2_scrubber_rating = usize_from_slice(&co2_scrubber_report[0]);

    println!("oxygen_generator_rating: {}, co2_scrubber_rating: {}, oxygen_generator_rating * co2_scrubber_rating: {}",
             oxygen_generator_rating, co2_scrubber_rating, oxygen_generator_rating * co2_scrubber_rating);
}

fn usize_from_slice(v: &[usize]) -> usize {
    let s = v
        .iter()
        .map(|v| if *v == 0 { '0' } else { '1' })
        .collect::<String>();
    usize_from_binary(s)
}

fn usize_from_binary<S: AsRef<str>>(s: S) -> usize {
    usize::from_str_radix(s.as_ref(), 2).unwrap()
}

fn count_zeros_ones(diagnostic_report: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let len = diagnostic_report[0].len();
    let mut counts = vec![vec![0; 2]; len];
    for i in 0..len {
        let digits: Vec<usize> = diagnostic_report.iter().map(|f| f[i]).collect();
        for d in digits {
            counts[i][d] += 1;
        }
    }
    counts
}

fn calculate_binary<F: Fn(usize, usize) -> T, T>(counts: &[Vec<usize>], f: F) -> Vec<T> {
    counts.iter().map(|t| f(t[0], t[1])).collect()
}

fn invert<S: AsRef<str>>(s: S) -> String {
    s.as_ref()
        .chars()
        .map(|c| if c == '1' { '0' } else { '1' })
        .collect()
}
