use std::collections::VecDeque;

mod cell;

pub struct Population {
  pub cells: VecDeque<VecDeque<cell::Cell>>
}

impl Population {
  pub fn new(n_rows: usize, n_cols: usize) -> Self {
    Self {
      cells: Self::make_cells(n_rows, n_cols)
    }
  }

  /// Generates a matrix of cells that are randomly alive or dead
  fn make_cells(n_rows: usize, n_cols: usize) -> VecDeque<VecDeque<cell::Cell>> {
    let mut cells = VecDeque::with_capacity(n_rows);
    for _ in 0..n_rows {
      let mut row_cells = VecDeque::with_capacity(n_cols);
      for _ in 0..n_cols {
        let cell = cell::Cell { lives: false };
        row_cells.push_back(cell);
      }
      cells.push_back(row_cells);
    }

    cells
  }

  pub fn update(&mut self) {
    unimplemented!()
  }

  fn count_alive_neighbors(&self) -> VecDeque<VecDeque<u8>> {
    let mut alive_neighbors = VecDeque::with_capacity(self.cells.len());
    for _ in 0..n_rows {
      let mut row_cells = VecDeque::with_capacity(n_cols);
      for _ in 0..n_cols {
        let cell = cell::Cell { lives: false };
        row_cells.push_back(cell);
      }
      cells.push_back(row_cells);
    }
    alive_neighbors
  }

  fn count_alive_neighbors_first_row(&self) -> VecDeque<u8> {
    unimplemented!()
  }

  fn count_alive_neighbors_mid_row(&self, i: usize) -> VecDeque<u8> {
    unimplemented!()
  }

  fn count_alive_neighbors_last_row(&self) -> VecDeque<u8> {
    unimplemented!()
  }

}