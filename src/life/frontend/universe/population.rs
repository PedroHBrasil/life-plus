use crate::life::frontend::util::Coord;

mod cell;

pub struct Population {
  pub cells: Vec<Vec<cell::CellProps>>,
}

impl Population {
  pub fn new(n_rows: usize, n_cols: usize, canvas_height: usize, canvas_width: usize, cell_size: usize) -> Self {
    Self {
      cells: Self::init_cells(n_rows, n_cols, canvas_height, canvas_width, cell_size),
    }
  }

  /// Initializes the cells
  fn init_cells(n_rows: usize, n_cols: usize, canvas_height: usize, canvas_width: usize, cell_size: usize) -> Vec<Vec<cell::CellProps>> {
    let mut cells = Vec::with_capacity(n_rows);
    for i in 0..n_rows {
      let y = (i * canvas_height) / n_rows;
      let mut row_cells = Vec::with_capacity(n_cols);
      for j in 0..n_cols {
        let x = (j * canvas_width) / n_cols;
        let cell = cell::CellProps {
          lives: rand::random(),
          origin: Coord { x, y },
          size: cell_size,
        };
        row_cells.push(cell);
      }
      cells.push(row_cells);
    }

    cells
  }
}

