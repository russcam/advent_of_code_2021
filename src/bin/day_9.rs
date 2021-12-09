use std::collections::HashSet;

const INPUT: &str = include_str!("../../input/day_9.txt");

struct Map {
    rows: Vec<Vec<Point>>,
}

impl From<Vec<Vec<Point>>> for Map {
    fn from(points: Vec<Vec<Point>>) -> Self {
        Self { rows: points }
    }
}

type Coord = (usize, usize);

#[derive(Debug, Eq, PartialEq, Hash)]
struct Point {
    height: u8,
    coord: Coord,
}

impl Point {
    pub fn new(c: char, x: usize, y: usize) -> Self {
        let height = c.to_digit(10).unwrap() as u8;
        Self {
            height,
            coord: (x, y),
        }
    }

    pub fn is_low_point(&self, map: &Map) -> bool {
        Self::adjacent_points(self, map)
            .iter()
            .flatten()
            .all(|a| self.height < a.height)
    }

    pub fn risk_level(&self) -> usize {
        self.height as usize + 1
    }

    pub fn basin_size(&self, map: &Map) -> usize {
        let mut points = HashSet::new();
        points.insert(self);
        Self::expand(&mut points, self, map);
        points.len()
    }

    fn adjacent_points<'a>(point: &'a Point, map: &'a Map) -> Vec<Option<&'a Point>> {
        vec![
            Self::adjacent_point(map, (point.coord.0, point.coord.1 + 1)),
            if point.coord.1 == 0 {
                None
            } else {
                Self::adjacent_point(map, (point.coord.0, point.coord.1 - 1))
            },
            if point.coord.0 == 0 {
                None
            } else {
                Self::adjacent_point(map, (point.coord.0 - 1, point.coord.1))
            },
            Self::adjacent_point(map, (point.coord.0 + 1, point.coord.1)),
        ]
    }

    fn expand<'a>(points: &mut HashSet<&'a Point>, point: &'a Point, map: &'a Map) {
        let adjacent = Self::adjacent_points(point, map);
        for a in adjacent.iter().flatten() {
            if a.height != 9 && !points.contains(a) {
                points.insert(a);
                Self::expand(points, a, map);
            }
        }
    }

    fn adjacent_point(map: &Map, coord: Coord) -> Option<&Point> {
        match map.rows.get(coord.1) {
            Some(r) => r.get(coord.0),
            None => None,
        }
    }
}

fn main() {
    let map: Map = INPUT
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| Point::new(c, x, y))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
        .into();

    let low_points: Vec<&Point> = map
        .rows
        .iter()
        .map(|r| {
            r.iter()
                .filter(|p| p.is_low_point(&map))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect();

    let sum_risk_levels = low_points.iter().map(|p| p.risk_level()).sum::<usize>();

    println!("sum of risk levels of low points: {}", sum_risk_levels);

    let mut basin_sizes = low_points
        .iter()
        .map(|p| p.basin_size(&map))
        .collect::<Vec<_>>();

    basin_sizes.sort_by(|a, b| b.cmp(a));

    println!(
        "product of 3 largest basin sizes, {:?}",
        &basin_sizes[0..3].iter().product::<usize>()
    );
}
