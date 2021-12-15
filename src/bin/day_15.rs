use pathfinding::prelude::dijkstra;

const INPUT: &str = include_str!("../../input/day_15.txt");

struct Grid(Vec<Vec<usize>>);

impl From<&str> for Grid {
    fn from(s: &str) -> Self {
        let rows = s
            .lines()
            .map(|l| l.as_bytes().iter().map(|&b| (b - b'0') as usize).collect())
            .collect();
        Self(rows)
    }
}

impl Grid {
    pub fn end(&self) -> Coord {
        let y = self.0.len();
        let x = self.0[y - 1].len();
        Coord(x - 1, y - 1)
    }

    pub fn shortest_path(&self, start: &Coord) -> usize {
        let end = self.end();
        dijkstra(start, |p| p.neighbours(&self.0, &end), |p| *p == end)
            .unwrap()
            .1
    }

    pub fn expand(&mut self) {
        for row in self.0.iter_mut() {
            let values = row.clone();
            for i in 1..=4 {
                row.extend(values.iter().map(|v| {
                    let mut n = v + i;
                    if n > 9 {
                        n -= 9;
                    }
                    n
                }));
            }
        }

        let values = self.0.clone();
        for i in 1..=4 {
            self.0.extend(values.iter().map(|row| {
                row.iter()
                    .map(|v| {
                        let mut n = v + i;
                        if n > 9 {
                            n -= 9;
                        }
                        n
                    })
                    .collect::<Vec<_>>()
            }));
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Coord(usize, usize);

impl Coord {
    fn neighbours(&self, grid: &[Vec<usize>], end: &Coord) -> Vec<(Coord, usize)> {
        let &Coord(x, y) = self;
        let &Coord(max_x, max_y) = end;
        let mut neighbours = Vec::with_capacity(4);
        if x < max_x {
            neighbours.push(Coord(x + 1, y));
        }
        if x > 0 {
            neighbours.push(Coord(x - 1, y));
        }
        if y < max_y {
            neighbours.push(Coord(x, y + 1));
        }
        if y > 0 {
            neighbours.push(Coord(x, y - 1));
        }
        neighbours
            .into_iter()
            .map(|c| (c.clone(), grid[c.1][c.0]))
            .collect()
    }
}

fn main() {
    let mut grid = Grid::from(INPUT);

    let start = Coord(0, 0);
    let shortest_path = grid.shortest_path(&start);
    println!("shortest path: {:?}", shortest_path);

    grid.expand();
    let shortest_path = grid.shortest_path(&start);
    println!("shortest path on expanded grid: {:?}", shortest_path);
}
