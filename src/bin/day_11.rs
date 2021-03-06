use ansi_term::Style;
use std::fmt::{Display, Formatter};
use std::thread;
use std::time::Duration;

const INPUT: &str = include_str!("../../input/day_11.txt");

#[derive(Debug)]
struct Octopus {
    energy_level: u32,
    flashed: bool,
}

impl From<u32> for Octopus {
    fn from(energy_level: u32) -> Self {
        Self {
            energy_level,
            flashed: false,
        }
    }
}

impl Octopus {
    pub fn increment_and_should_flash(&mut self) -> bool {
        self.energy_level += 1;
        self.should_flash()
    }

    pub fn should_flash(&self) -> bool {
        self.energy_level > 9 && !self.flashed
    }

    pub fn flash(&mut self) {
        self.flashed = true;
    }

    pub fn reset(&mut self) {
        if self.flashed {
            self.energy_level = 0;
            self.flashed = false;
        }
    }
}

impl Display for Octopus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.energy_level {
            0 => write!(f, "{}", Style::new().bold().paint("0")),
            n => write!(f, "{}", n),
        }
    }
}

#[derive(Debug)]
struct Grid {
    octopus: Vec<Vec<Octopus>>,
    count: usize,
    total_flashes: usize,
    step_flash: usize,
    y: usize,
    x: usize,
    visualize: bool,
}

impl From<&str> for Grid {
    fn from(s: &str) -> Self {
        let octopus = s
            .lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap().into()).collect())
            .collect::<Vec<Vec<_>>>();

        let y = octopus.len();
        let x = octopus[0].len();
        let count = y * x;
        Self {
            octopus,
            x,
            y,
            total_flashes: 0,
            step_flash: 0,
            count,
            visualize: false,
        }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.octopus {
            for (i, octopus) in row.iter().enumerate() {
                if i == row.len() - 1 {
                    writeln!(f, "{}", octopus)?;
                } else {
                    write!(f, "{}", octopus)?;
                }
            }
        }
        Ok(())
    }
}

impl Grid {
    pub fn octopus_count(&self) -> usize {
        self.count
    }

    pub fn advance_step(&mut self) -> usize {
        let mut flash = self.increment();
        while flash {
            flash = self.increment_adjacent();
        }
        let step_flash = self.step_flash;
        self.total_flashes += step_flash;
        self.reset();
        if self.visualize {
            self.visualize();
        }
        step_flash
    }

    fn visualize(&self) {
        thread::sleep(Duration::from_millis(100));
        print!("\x1B[2J\x1B[1;1H");
        print!("{}", &self);
    }

    fn reset(&mut self) {
        self.step_flash = 0;
        for row in self.octopus.iter_mut() {
            for octopus in row {
                octopus.reset();
            }
        }
    }

    fn increment(&mut self) -> bool {
        let mut flash = false;
        for row in self.octopus.iter_mut() {
            for octopus in row {
                flash |= octopus.increment_and_should_flash();
            }
        }
        flash
    }

    fn increment_adjacent(&mut self) -> bool {
        let mut flash = false;
        for y in 0..self.y {
            for x in 0..self.x {
                let mut octopus = &mut self.octopus[y][x];
                if octopus.should_flash() {
                    self.step_flash += 1;
                    octopus.flash();

                    if y < self.y - 1 {
                        // above
                        octopus = &mut self.octopus[y + 1][x];
                        flash |= octopus.increment_and_should_flash();

                        // above right
                        if x < self.x - 1 {
                            octopus = &mut self.octopus[y + 1][x + 1];
                            flash |= octopus.increment_and_should_flash();
                        }

                        // above left
                        if x > 0 {
                            octopus = &mut self.octopus[y + 1][x - 1];
                            flash |= octopus.increment_and_should_flash();
                        }
                    }

                    if y > 0 {
                        //below
                        octopus = &mut self.octopus[y - 1][x];
                        flash |= octopus.increment_and_should_flash();

                        // below right
                        if x < self.x - 1 {
                            octopus = &mut self.octopus[y - 1][x + 1];
                            flash |= octopus.increment_and_should_flash();
                        }

                        // below left
                        if x > 0 {
                            octopus = &mut self.octopus[y - 1][x - 1];
                            flash |= octopus.increment_and_should_flash();
                        }
                    }

                    // right
                    if x < self.x - 1 {
                        octopus = &mut self.octopus[y][x + 1];
                        flash |= octopus.increment_and_should_flash();
                    }

                    // left
                    if x > 0 {
                        octopus = &mut self.octopus[y][x - 1];
                        flash |= octopus.increment_and_should_flash();
                    }
                }
            }
        }
        flash
    }

    pub fn total_flashes(&self) -> usize {
        self.total_flashes
    }
}

fn main() {
    let mut grid = Grid::from(INPUT);

    // to visualize, uncomment
    // grid.visualize = true;

    let mut all_flash_step = None;
    let mut steps = 100;

    for step in 1..=steps {
        let step_count = grid.advance_step();
        if step_count == grid.octopus_count() && all_flash_step.is_none() {
            all_flash_step = Some(step);
        }
    }

    let total_flashes_after_100 = grid.total_flashes();
    if all_flash_step.is_none() {
        loop {
            steps += 1;
            let step_count = grid.advance_step();
            if step_count == grid.octopus_count() {
                all_flash_step = Some(steps);
                break;
            }
        }
    }

    println!("total flashes after 100 steps: {}", total_flashes_after_100);
    println!(
        "first step in which all octopuses flash: {}",
        all_flash_step.unwrap()
    );
}
