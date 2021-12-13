use std::collections::VecDeque;
use std::fmt::{Display, Formatter};

const INPUT: &str = include_str!("../../input/day_13.txt");

type Coord = (usize, usize);

#[derive(Debug, Clone, Eq, PartialEq)]
enum Point {
    Dot,
    Empty,
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Point::Dot => write!(f, "#"),
            Point::Empty => write!(f, "."),
        }
    }
}

impl Point {
    pub fn merge(&self, other: &Self) -> Self {
        if matches!(self, &Point::Dot) || matches!(other, &Point::Dot) {
            Point::Dot
        } else {
            Point::Empty
        }
    }
}

impl From<&str> for Point {
    fn from(s: &str) -> Self {
        match s {
            "." => Self::Empty,
            "#" => Self::Dot,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
enum Direction {
    X,
    Y,
}

impl From<&str> for Direction {
    fn from(s: &str) -> Self {
        match s {
            "x" => Self::X,
            "y" => Self::Y,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    position: usize,
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        let (direction, position) = {
            let (d, p) = s.trim_start_matches("fold along ").split_once('=').unwrap();
            (Direction::from(d), p.parse().unwrap())
        };

        Self {
            direction,
            position,
        }
    }
}

#[derive(Debug)]
struct Grid {
    rows: Vec<Vec<Point>>,
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.rows {
            for (i, p) in row.iter().enumerate() {
                if i == row.len() - 1 {
                    writeln!(f, "{}", p)?;
                } else {
                    write!(f, "{}", p)?;
                }
            }
        }

        Ok(())
    }
}

impl Grid {
    pub fn new(x: usize, y: usize, coords: Vec<Coord>) -> Self {
        let mut rows = vec![vec![Point::Empty; x]; y];
        for (x, y) in coords {
            rows[y][x] = Point::Dot;
        }

        Self { rows }
    }

    pub fn fold(&mut self, instruction: Instruction) {
        match instruction.direction {
            Direction::X => {
                let mut left: Vec<Vec<Point>> = Vec::with_capacity(self.rows.len());
                for (y, row) in self.rows.iter_mut().enumerate() {
                    left.push(row.drain(..instruction.position).collect::<Vec<_>>());
                    // remove the fold
                    row.remove(0);
                    for (x, point) in row.iter_mut().rev().enumerate() {
                        left[y][x] = (&left[y][x]).merge(point);
                    }
                }

                self.rows = left;
            }
            Direction::Y => {
                let mut top: Vec<_> = self.rows.drain(..instruction.position).collect();
                // remove the fold
                self.rows.remove(0);
                for (y, row) in self.rows.iter_mut().rev().enumerate() {
                    for (x, point) in row.iter().enumerate() {
                        top[y][x] = (&top[y][x]).merge(point);
                    }
                }

                self.rows = top;
            }
        }
    }

    pub fn dot_count(&self) -> usize {
        self.rows
            .iter()
            .map(|r| r.iter().filter(|p| matches!(p, Point::Dot)).count())
            .sum()
    }
}

fn main() {
    let mut coords = vec![];
    let mut lines = INPUT.lines();
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }

        coords.push(
            line.split_once(',')
                .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
                .unwrap(),
        );
    }

    let mut instructions = lines.map(Instruction::from).collect::<VecDeque<_>>();

    let x = {
        let x_instruction = instructions
            .iter()
            .find(|i| matches!(i.direction, Direction::X))
            .unwrap();
        x_instruction.position * 2 + 1
    };
    let y = {
        let y_instruction = instructions
            .iter()
            .find(|i| matches!(i.direction, Direction::Y))
            .unwrap();
        y_instruction.position * 2 + 1
    };

    let mut grid = Grid::new(x, y, coords);
    grid.fold(instructions.pop_front().unwrap());
    println!("dot count after first fold: {}", grid.dot_count());

    while let Some(instruction) = instructions.pop_front() {
        grid.fold(instruction);
    }

    println!("The code is:");
    println!("{}", grid);
}
