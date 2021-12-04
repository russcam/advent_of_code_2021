const INPUT: &str = include_str!("../../input/day_1.txt");

fn main() {
    let depths: Vec<i32> = INPUT
        .lines()
        .filter_map(|l| l.trim().parse().ok())
        .collect();

    let increasing = depths
        .windows(2)
        .filter(|w| w[1] > w[0])
        .collect::<Vec<_>>();

    println!("increasing depths: {}", increasing.len());

    let window_sums = depths
        .windows(3)
        .map(|w| w.iter().sum())
        .collect::<Vec<i32>>();

    let windows = window_sums
        .windows(2)
        .filter(|w| w[1] > w[0])
        .collect::<Vec<_>>();

    println!("increasing windows: {}", windows.len());
}
