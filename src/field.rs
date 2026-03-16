use std::fmt::Display;
use {CellValue::*, CellState::*};

/// A `rows` * `cols` field of mines.
pub struct Field {
    cells: Vec<Vec<Cell>>,
    rows: usize,
    cols: usize,
}

/// A cell that may contain a mine or else indicates how many of its neighbors are mines.
#[derive(Debug, Clone)]
pub struct Cell {
    pub value: CellValue,
    pub state: CellState,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum CellValue {
    Mine,
    Value(usize),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum CellState {
    Unflagged,
    Flagged,
    Revealed,
}

impl Default for Cell {
    fn default() -> Self {
        Cell { value: Value(0), state: Unflagged }
    }
}

impl Field {

    /// Initialize an empty field with no mines.
    fn new(rows: usize, cols: usize) -> Self {
        let cells = vec![vec![Default::default(); cols]; rows];
        Self { cells, rows, cols }
    }

    /// Initialize a field with mines at the specified locations.
    pub fn new_with_mines_at(rows: usize, cols: usize, locations: &[(usize, usize)]) -> Self {
        let mut mines_matrix = vec![vec![false; cols]; rows]; 
        for &(row, col) in locations {
            mines_matrix[row][col] = true;
        }

        Self::new_from_bool_matrix(rows, cols, mines_matrix)
    }

    /// Initialize a field with mines where the specified matrix is `true`.
    pub fn new_from_bool_matrix(rows: usize, cols: usize, mines_matrix: Vec<Vec<bool>>) -> Self {
        let mut field = Field::new(rows, cols);

        for row in 0..rows {
            for col in 0..cols {
                if mines_matrix[row][col] {
                    field.cells[row][col].value = Mine;
                }
                else {
                    let mut neighbor_mine_count = 0;
                    for (neighbor_row, neighbor_col) in field.neighbors_of(row, col) {
                        if mines_matrix[neighbor_row][neighbor_col] {
                            neighbor_mine_count += 1;
                        }
                    }
                    field.cells[row][col].value = Value(neighbor_mine_count);
                }
            }
        }

        field
    }

    /// Reveal the cell at the specified location.
    /// If the revealed cell is a `Value(0)`, then reveal the cells around it automatically.
    ///
    /// Panics: if the specified `row` and `col` lie outside the bounds of the array.
    pub fn reveal(&mut self, row: usize, col: usize) {
        let this_cell = &mut self.cells[row][col];
        this_cell.state = Revealed;

        if this_cell.value == Value(0) {
            for (neighbor_row, neighbor_col) in self.neighbors_of(row, col) {
                let neighbor_cell = &self.cells[neighbor_row][neighbor_col];
                if neighbor_cell.state != Revealed {
                    self.reveal(neighbor_row, neighbor_col);
                }
            }
        }
    }

    /// If the cell at the specified location is a `Value(n)` and it already neighbors `n` flagged cells, reveal the neighboring unflagged cells.
    ///
    /// Panics: if the specified `row` and `col` lie outside the bounds of the array.
    pub fn auto_reveal(&mut self, row: usize, col: usize) {
        let this_cell = &self.cells[row][col];

        let flagged_neighbors: Vec<_> = self.neighbors_of(row, col).into_iter()
            .filter(|&(r, c)| self.cells[r][c].state == Flagged)
            .collect();

        let unflagged_neighbors: Vec<_> = self.neighbors_of(row, col).into_iter()
            .filter(|&(r, c)| self.cells[r][c].state == Unflagged)
            .collect();

        if let Value(x) = this_cell.value && x == flagged_neighbors.len() {
            for (neighbor_row, neighbor_col) in unflagged_neighbors {
                self.reveal(neighbor_row, neighbor_col);
            }
        }
    }

    /// Flag the specified cell (if it is not already flagged or revealed).
    ///
    /// Panics: if the specified `row` and `col` lie outside the bounds of the array.
    pub fn flag(&mut self, row: usize, col: usize) {
        let this_cell = &mut self.cells[row][col];
        if this_cell.state == Unflagged {
            this_cell.state = Flagged;
        }
    }

    /// If the specified cell is a `Value(n)` and neighbors only `n` unrevealed cells, flag those cells.
    ///
    /// Panics: if the specified `row` and `col` lie outside the bounds of the array.
    pub fn auto_flag(&mut self, row: usize, col: usize) {
        let this_cell = &self.cells[row][col];
        let hidden_neighbors: Vec<_> = self.neighbors_of(row, col).into_iter()
            .filter(|&(r, c)| self.cells[r][c].state != Revealed)
            .collect();

        let hidden_neighbors_count = hidden_neighbors.len();

        if let Value(x) = this_cell.value && this_cell.state == Revealed && x == hidden_neighbors_count {
            for (neighbor_row, neighbor_col) in hidden_neighbors {
                self.flag(neighbor_row, neighbor_col);
            }
        }
    }

    /// Returns a vec containing the `(row, col)` indices of the neighbors of the specified cell.
    /// This function performs checks wrt the bounds of the array and the coordinates that it returns are therefore always valid.
    ///
    /// Panics: if the specified `row` and `col` lie outside the bounds of the array.
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

    /// Returns an iterator over all the cells in the `Field`.
    fn iter(&self) -> impl Iterator<Item = &Cell> {
        self.cells.iter().flatten()
    }

    /// Returns an iterator over all the cells in the `Field` that have been revealed.
    fn revealed_cells(&self) -> impl Iterator<Item = &Cell> {
        self.iter().filter(|cell| cell.state == Revealed)
    }

    /// Returns an iterator over all the cells in the `Field` that are not revealed.
    fn not_revealed_cells(&self) -> impl Iterator<Item = &Cell> {
        self.iter().filter(|cell| cell.state != Revealed)
    }

    /// Returns `true` if the game has been lost.
    pub fn lost(&self) -> bool {
        self.revealed_cells().any(|cell| cell.value == Mine)
    }

    /// Returns `true` if the game has been won.
    pub fn won(&self) -> bool {
        !self.lost() && self.not_revealed_cells().all(|cell| cell.value == Mine)
    }

    /// Display the field with the specified function that renders [`Cell`]s as `char`s.
    pub fn display_with_format<F>(&self, format: F) -> String
    where F: Fn(&Cell) -> char {
        let formatted_rows: Vec<String> = self.cells.iter()
            .map(|row| {
                row.iter()
                    .map(|cell| format(cell))
                    .collect::<String>()
            })
            .collect();
        formatted_rows.join("\n")
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let formatted_field = self.display_with_format(|cell| {
            match cell.state {
                Unflagged => '·',
                Flagged => '⚐',
                Revealed => match cell.value {
                    Mine => 'M',
                    Value(n) => [' ', '1', '2', '3', '4', '5', '6', '7', '8'][n]
                }
            }
        });
        write!(f, "{formatted_field}")
    }
}

impl std::fmt::Debug for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Field with {} rows and {} cols:\n", self.rows, self.cols)?;

        let formatted_field = self.display_with_format(|cell| {
            match cell.value {
                Mine => 'M',
                Value(n) => [' ', '1', '2', '3', '4', '5', '6', '7', '8'][n],
            }
        });

        write!(f, "{formatted_field}")
    }
}