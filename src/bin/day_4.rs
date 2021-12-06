use std::str::FromStr;

const INPUT: &str = include_str!("../../input/day_4.txt");

#[derive(Debug)]
struct Bingo {
    numbers: Vec<usize>,
    last_number_index: usize,
    boards: Vec<Board>,
}

impl Bingo {
    pub fn winning_boards(&self) -> Option<Vec<WinningBoard>> {
        let mut winning_boards = vec![];
        for (i, board) in self.boards.iter().enumerate() {
            if let Some((score, index)) = board.score_and_index() {
                winning_boards.push(WinningBoard {
                    index: i,
                    last_number_index: index,
                    score,
                });
            }
        }

        if !winning_boards.is_empty() {
            winning_boards.sort_by(|a, b| a.last_number_index.cmp(&b.last_number_index));
            Some(winning_boards)
        } else {
            None
        }
    }

    pub fn draw_number(&mut self) -> bool {
        if let Some(n) = self.numbers.get(self.last_number_index) {
            for board in &mut self.boards {
                board.mark(*n, self.last_number_index);
            }

            self.last_number_index += 1;
            true
        } else {
            false
        }
    }
}

impl FromStr for Bingo {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let numbers = lines
            .next()
            .unwrap()
            .split(',')
            .map(|c| c.parse().unwrap())
            .collect();

        let mut boards = vec![];
        let mut board = None;

        for line in lines {
            if line.is_empty() {
                if let Some(b) = board {
                    boards.push(b);
                }
                board = Some(Board::new());
                continue;
            }

            if let Some(b) = board.as_mut() {
                b.add_row(line);
            }
        }

        Ok(Self {
            numbers,
            last_number_index: 0,
            boards,
        })
    }
}

#[derive(Debug)]
struct WinningBoard {
    pub index: usize,
    pub last_number_index: usize,
    pub score: usize,
}

#[derive(Debug)]
enum BoardNumber {
    Unmarked(usize),
    Marked(usize),
}

impl BoardNumber {
    pub fn is_marked(&self) -> bool {
        match self {
            BoardNumber::Marked(_) => true,
            BoardNumber::Unmarked(_) => false,
        }
    }

    pub fn is_unmarked(&self) -> bool {
        !self.is_marked()
    }

    pub fn value(&self) -> usize {
        match self {
            BoardNumber::Marked(n) => *n,
            BoardNumber::Unmarked(n) => *n,
        }
    }
}

#[derive(Debug)]
struct Board {
    rows: Vec<Vec<BoardNumber>>,
    score_and_index: Option<(usize, usize)>,
}

impl Board {
    pub fn new() -> Self {
        Self {
            rows: vec![],
            score_and_index: None,
        }
    }

    pub fn add_row(&mut self, line: &str) {
        let row = line
            .split_whitespace()
            .map(|c| BoardNumber::Unmarked(c.parse().ok().unwrap()))
            .collect();

        self.rows.push(row);
    }

    pub fn mark(&mut self, number: usize, index: usize) {
        if self.score_and_index.is_none() {
            for row in &mut self.rows {
                for board_number in row {
                    match board_number {
                        BoardNumber::Unmarked(n) if *n == number => {
                            *board_number = BoardNumber::Marked(*n);
                        }
                        _ => {}
                    }
                }
            }

            self.calculate_row_win(number, index);
            if self.score_and_index.is_none() {
                self.calculate_column_win(number, index)
            }
        }
    }

    pub fn score_and_index(&self) -> Option<(usize, usize)> {
        self.score_and_index
    }

    fn calculate_row_win(&mut self, number: usize, index: usize) {
        for row in &self.rows {
            if row.iter().all(|b| b.is_marked()) {
                let sum_unmarked = self.sum_unmarked_numbers();
                self.score_and_index = Some((sum_unmarked * number, index));
            }
        }
    }

    fn calculate_column_win(&mut self, number: usize, index: usize) {
        let row_len = self.rows[0].len();
        for i in 0..row_len {
            if self.rows.iter().map(|r| &r[i]).all(|b| b.is_marked()) {
                let sum_unmarked = self.sum_unmarked_numbers();
                self.score_and_index = Some((sum_unmarked * number, index));
            }
        }
    }

    fn sum_unmarked_numbers(&self) -> usize {
        self.rows
            .iter()
            .map(|r| {
                r.iter()
                    .filter(|b| b.is_unmarked())
                    .map(|b| b.value())
                    .collect::<Vec<usize>>()
            })
            .flatten()
            .sum()
    }
}

fn main() {
    let mut bingo = Bingo::from_str(INPUT).unwrap();
    while bingo.winning_boards().is_none() {
        bingo.draw_number();
    }

    let mut winning_boards = bingo.winning_boards().unwrap();
    println!("score of winning board: {}", winning_boards[0].score);

    while bingo.draw_number() {}

    winning_boards = bingo.winning_boards().unwrap();
    println!(
        "score of last winning board: {}",
        winning_boards.last().unwrap().score
    );
}
