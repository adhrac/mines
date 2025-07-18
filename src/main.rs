#![allow(unused)]

use std::fmt::Display;
fn main() {
    let board = Board::new_with_mines_at(5, 5, &[(4,4), (3,3), (3,4)]);
    println!("{:?}", board);
}

struct Board {
    cells: Vec<Vec<Cell>>,
    rows: usize,
    cols: usize,
}

impl std::fmt::Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Board(rows: {}, cols: {}, board state: \n", self.rows, self.cols);
        for row in &self.cells {
            for cell in row {
                let s = match cell.value {
                    CellValue::Mine => "M",
                    CellValue::Value(n) => &format!("{n}"),
                };
                write!(f, "{s}");
            }
            write!(f, "\n");
        }
        Ok(())
    }
}

#[derive(Debug, Copy, Clone)]
struct Cell {
    value: CellValue,
    state: CellState,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum CellValue {
    Mine,
    Value(usize),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum CellState {
    Unflagged,
    Flagged,
    Revealed,
}

impl Default for Cell {
    fn default() -> Self {
        Cell { value: CellValue::Value(0), state: CellState::Unflagged }
    }
}

impl Board {
    fn new(rows: usize, cols: usize) -> Self {
        let cells = vec![vec![Default::default(); cols]; rows];
        Self { cells, rows, cols }
    }

    fn new_with_mines_at(rows: usize, cols: usize, locations: &[(usize, usize)]) -> Self {
        let mut board = Board::new(rows, cols);

        let mut mines_matrix = vec![vec![false; cols]; rows];
        for &(row, col) in locations {
            mines_matrix[row][col] = true;
        }

        for row in 0..rows {
            for col in 0..cols {
                if mines_matrix[row][col] {
                    board.cells[row][col].value = CellValue::Mine;
                }
                else {
                    let mut neighbor_mine_count = 0;
                    for (neighbor_row, neighbor_col) in board.neighbors_of(row, col) {
                        if mines_matrix[neighbor_row][neighbor_col] {
                            neighbor_mine_count += 1;
                        }
                    }
                    board.cells[row][col].value = CellValue::Value(neighbor_mine_count);
                }
            }
        }

        board
    }

    fn reveal(&mut self, row: usize, col: usize) {
        assert!(row < self.rows);
        assert!(col < self.cols);
        self.cells[row][col].state = CellState::Revealed;
    }

    fn neighbors_of(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        assert!(row < self.rows);
        assert!(col < self.cols);
        let mut neighbors: Vec<(isize, isize)> = Vec::new();
        let max_row = self.rows as isize;
        let max_col = self.cols as isize;

        for r in -1..=1 {
            for c in -1..=1 {
                let neighbor_row = row as isize + r;
                let neighbor_col = col as isize + c;
                if neighbor_row >= 0 && neighbor_col >= 0 && neighbor_row < max_row && neighbor_col < max_col {
                    if r != 0 || c != 0 {
                        neighbors.push((neighbor_row, neighbor_col));
                    }
                }
            }
        }

        neighbors.into_iter().map(|(a, b)| (a.try_into().unwrap(), b.try_into().unwrap())).collect()
    }

    fn iter(&self) -> impl Iterator<Item = &Cell> {
        self.cells.iter().flatten()
    }

    fn revealed_cells(&self) -> impl Iterator<Item = &Cell> {
        self.iter().filter(|cell| cell.state == CellState::Revealed)
    }

    fn not_revealed_cells(&self) -> impl Iterator<Item = &Cell> {
        self.iter().filter(|cell| ! (cell.state == CellState::Revealed))
    }

    fn lost(&self) -> bool {
        self.revealed_cells().any(|cell| cell.value == CellValue::Mine)
    }

    fn won(&self) -> bool {
        !self.lost() && self.not_revealed_cells().all(|cell| cell.value == CellValue::Mine)
    }
}