use crate::Direction::{Down, Forward, Up};

const INPUT: &str = include_str!("../../input/day_2.txt");

pub enum Direction {
    Forward(i32),
    Down(i32),
    Up(i32)
}

fn main() {
    let directions: Vec<Direction> = INPUT
        .lines()
        .map(|l| {
            if l.starts_with("forward") {
                Direction::Forward(l.trim_start_matches("forward ").parse().ok().unwrap())
            } else if l.starts_with("down") {
                Direction::Down(l.trim_start_matches("down ").parse().ok().unwrap())
            } else {
                Direction::Up(l.trim_start_matches("up ").parse().ok().unwrap())
            }
        })
        .collect();

    let mut horizontal_pos = 0;
    let mut depth = 0;

    for direction in directions.iter() {
        match direction {
            Forward(i) => horizontal_pos += i,
            Down(i) => depth += i,
            Up(i) => depth -= i
        }
    }

    println!("final position: {}", horizontal_pos * depth);

    horizontal_pos = 0;
    depth = 0;
    let mut aim = 0;

    for direction in directions {
        match direction {
            Forward(i) => {
                horizontal_pos += i;
                depth += aim * i;
            },
            Down(i) => aim += i,
            Up(i) => aim -= i
        }
    }

    println!("final position including aim: {}", horizontal_pos * depth);
}
