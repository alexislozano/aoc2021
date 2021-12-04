use crate::file;

pub fn run(part: u8) {
    let exo = 4;
    let input = file::read(exo);
    let result = match part {
        1 => run1(&input),
        _ => run2(&input),
    };
    file::write(exo, part, &result.to_string());
}

fn run1(input: &str) -> usize {
    let (numbers, mut boards) = parse(input);

    for number in numbers {
        for board in &mut boards {
            board.mark(number);
            if board.has_won() {
                return board.score() * number;
            }
        }
    }

    0
}

fn run2(input: &str) -> usize {
    let (numbers, mut boards) = parse(input);
    let boards_nb = boards.len();
    let mut win_nb = 0;

    for number in numbers {
        for board in &mut boards {
            if !board.is_won {
                board.mark(number);
                if board.has_won() {
                    win_nb += 1;
                    board.is_won = true;
                }
                if win_nb == boards_nb {
                    return board.score() * number;
                }
            }
        }
    }

    0
}

struct Board {
    rows: Vec<Vec<Cell>>,
    is_won: bool,
}

impl Board {
    fn new(rows: Vec<Vec<Cell>>) -> Self {
        Self {
            rows,
            is_won: false,
        }
    }

    fn mark(&mut self, number: usize) {
        for row in &mut self.rows {
            for cell in row {
                if cell.number == number {
                    cell.marked = true;
                }
            }
        }
    }

    fn cols(&self) -> Vec<Vec<Cell>> {
        let mut cols = vec![];
        for i in 0..self.rows[0].len() {
            let mut col = vec![];
            for j in 0..self.rows.len() {
                col.push(self.rows[j][i].clone());
            }
            cols.push(col);
        }
        return cols;
    }

    fn has_won(&self) -> bool {
        self.rows
            .iter()
            .any(|row| row.iter().all(|cell| cell.marked))
            || self
                .cols()
                .iter()
                .any(|col| col.iter().all(|cell| cell.marked))
    }

    fn score(&self) -> usize {
        self.rows
            .iter()
            .map(|row| {
                row.iter()
                    .filter(|cell| !cell.marked)
                    .map(|cell| cell.number)
                    .sum::<usize>()
            })
            .sum()
    }
}

#[derive(Clone)]
struct Cell {
    number: usize,
    marked: bool,
}

impl Cell {
    fn new(number: usize) -> Self {
        Self {
            number,
            marked: false,
        }
    }
}

fn parse(input: &str) -> (Vec<usize>, Vec<Board>) {
    let mut blocks = input.split("\n\n").collect::<Vec<&str>>();
    let numbers_str = blocks.remove(0);
    let numbers = numbers_str
        .split(",")
        .map(|number| number.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let boards = blocks
        .into_iter()
        .map(|board_str| {
            let board = board_str
                .split("\n")
                .map(|line| {
                    line.split_whitespace()
                        .map(|number| Cell::new(number.parse::<usize>().unwrap()))
                        .collect::<Vec<Cell>>()
                })
                .collect::<Vec<Vec<Cell>>>();
            Board::new(board)
        })
        .collect::<Vec<Board>>();
    (numbers, boards)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run11() {
        assert_eq!(run1("7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1\n\n22 13 17 11  0\n 8  2 23  4 24\n21  9 14 16  7\n 6 10  3 18  5\n 1 12 20 15 19\n\n 3 15  0  2 22\n 9 18 13 17  5\n19  8  7 25 23\n20 11 10 24  4\n14 21 16 12  6\n\n14 21 17 24  4\n10 16 15  9 19\n18  8 23 26 20\n22 11 13  6  5\n 2  0 12  3  7"), 4512);
    }

    #[test]
    fn run21() {
        assert_eq!(run2("7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1\n\n22 13 17 11  0\n 8  2 23  4 24\n21  9 14 16  7\n 6 10  3 18  5\n 1 12 20 15 19\n\n 3 15  0  2 22\n 9 18 13 17  5\n19  8  7 25 23\n20 11 10 24  4\n14 21 16 12  6\n\n14 21 17 24  4\n10 16 15  9 19\n18  8 23 26 20\n22 11 13  6  5\n 2  0 12  3  7"), 1924);
    }
}
