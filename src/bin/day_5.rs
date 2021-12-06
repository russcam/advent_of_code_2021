use crate::LineType::{Diagonal, Horizontal, Vertical};
use crate::Point::{Marked, Unmarked};
use std::cmp::Ordering;
use std::fmt::Formatter;
use std::str::FromStr;

const INPUT: &str = include_str!("../../input/day_5.txt");

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum LineType {
    Horizontal,
    Vertical,
    Diagonal,
}

type Coord = (usize, usize);

#[derive(Debug, Clone)]
struct LineSegment {
    start: Coord,
    end: Coord,
    curr: Option<Coord>,
    line_type: LineType,
}

impl LineSegment {
    fn parse_coords(s: &str) -> Coord {
        let mut parts = s.split(',').map(|p| p.parse().unwrap());
        (parts.next().unwrap(), parts.next().unwrap())
    }

    pub fn max_coords(&self) -> Coord {
        let max_x = std::cmp::max(self.start.0, self.end.0);
        let max_y = std::cmp::max(self.start.1, self.end.1);
        (max_x, max_y)
    }
}

impl FromStr for LineSegment {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" -> ");
        let start = Self::parse_coords(parts.next().unwrap());
        let end = Self::parse_coords(parts.next().unwrap());
        let x_diff = start.0 as i32 - end.0 as i32;
        let y_diff = start.1 as i32 - end.1 as i32;
        let line_type = match (x_diff, y_diff) {
            (0, _) => Vertical,
            (_, 0) => Horizontal,
            _ => Diagonal,
        };

        Ok(Self {
            start,
            end,
            curr: None,
            line_type,
        })
    }
}

impl Iterator for LineSegment {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr.is_none() {
            self.curr = Some(self.start);
            self.curr
        } else if self.curr == Some(self.end) {
            None
        } else {
            self.curr = self.curr.map(|c| {
                let x = match c.0.cmp(&self.end.0) {
                    Ordering::Less => c.0 + 1,
                    Ordering::Equal => c.0,
                    Ordering::Greater => c.0 - 1,
                };

                let y = match c.1.cmp(&self.end.1) {
                    Ordering::Less => c.1 + 1,
                    Ordering::Equal => c.1,
                    Ordering::Greater => c.1 - 1,
                };

                (x, y)
            });
            self.curr
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Point {
    Unmarked,
    Marked(usize),
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Unmarked => write!(f, "."),
            Marked(n) => write!(f, "{}", n),
        }
    }
}

struct Plot {
    coords: Vec<Vec<Point>>,
}

impl Plot {
    pub fn new(line_segments: &mut [LineSegment]) -> Self {
        let (max_x, max_y): (Vec<_>, Vec<_>) = line_segments.iter().map(|l| l.max_coords()).unzip();

        let max_x = max_x.iter().max().unwrap();
        let max_y = max_y.iter().max().unwrap();
        let mut coords = vec![vec![Unmarked; *max_x]; *max_y];
        for line_segment in line_segments {
            for (x, y) in line_segment {
                coords[y - 1][x - 1] = match coords[y - 1][x - 1] {
                    Unmarked => Marked(1),
                    Marked(n) => Marked(n + 1),
                }
            }
        }

        Self { coords }
    }

    pub fn overlaps(&self) -> usize {
        self.coords
            .iter()
            .map(|r| {
                r.iter().filter(|p| matches!(p, Marked(n) if *n > 1))
            })
            .flatten()
            .count()
    }
}

impl std::fmt::Display for Plot {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.coords.iter().rev() {
            for (i, point) in row.iter().enumerate() {
                if i == row.len() - 1 {
                    writeln!(f, "{}", point).unwrap();
                } else {
                    write!(f, "{}", point).unwrap();
                }
            }
        }
        Ok(())
    }
}

fn main() {
    let mut line_segments: Vec<LineSegment> = INPUT
        .lines()
        .map(|l| LineSegment::from_str(l).unwrap())
        .collect();

    let mut vertical_horizontal_line_segments: Vec<_> = line_segments
        .iter()
        .filter(|&l| l.line_type != Diagonal)
        .cloned()
        .collect();

    let mut plot = Plot::new(&mut vertical_horizontal_line_segments);

    println!("horizontal and vertical overlaps: {}", plot.overlaps());

    plot = Plot::new(&mut line_segments);

    println!("all overlaps: {}", plot.overlaps());

    // visualize the plot
    //print!("{}", plot);
}
