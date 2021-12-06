use crate::Direction::{Down, Forward, Up};
use std::str::FromStr;

const INPUT: &str = include_str!("../../input/day_2.txt");

pub enum Direction {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ');
        let direction = parts.next().unwrap();
        let value = parts.next().unwrap().parse().unwrap();
        match direction {
            "forward" => Ok(Self::Forward(value)),
            "down" => Ok(Self::Down(value)),
            "up" => Ok(Self::Up(value)),
            _ => Err(()),
        }
    }
}

impl Direction {
    pub fn part_1(&self, position: &mut Position) {
        match self {
            Forward(i) => position.horizontal += i,
            Down(i) => position.depth += i,
            Up(i) => position.depth -= i,
        }
    }

    pub fn part_2(&self, position: &mut Position) {
        match self {
            Forward(i) => {
                position.horizontal += i;
                position.depth += position.aim * i;
            }
            Down(i) => position.aim += i,
            Up(i) => position.aim -= i,
        }
    }
}

#[derive(Default)]
pub struct Position {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

impl Position {
    pub fn reset(&mut self) {
        self.horizontal = 0;
        self.depth = 0;
        self.aim = 0;
    }

    pub fn value(&self) -> i32 {
        self.horizontal * self.depth
    }
}

fn main() {
    let directions: Vec<Direction> = INPUT
        .lines()
        .filter_map(|l| Direction::from_str(l).ok())
        .collect();

    let mut position = Position::default();

    for direction in directions.iter() {
        direction.part_1(&mut position);
    }

    println!("final position: {}", position.value());

    position.reset();

    for direction in directions {
        direction.part_2(&mut position);
    }

    println!("final position including aim: {}", position.value());
}
