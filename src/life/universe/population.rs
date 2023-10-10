use crate::{settings, util::{self, Drawable}};

mod cell;

pub struct Population {
  pub cells: Vec<Vec<cell::Cell>>,
  n_rows: usize,
  n_cols: usize,
  alive_neighbors_counts: Vec<Vec<u8>>,
}

impl Population {
  pub fn new(settings: &settings::Settings) -> Self {
    let n_rows = settings.universe.height / settings.cells.size;
    let n_cols = settings.universe.width / settings.cells.size;
    Self {
      cells: Self::init_cells(settings, n_rows, n_cols),
      n_rows,
      n_cols,
      alive_neighbors_counts: Self::init_alive_neighbors_counts(n_rows, n_cols),
    }
  }

  /// Initializes the cells
  fn init_cells(settings: &settings::Settings, n_rows: usize, n_cols: usize) -> Vec<Vec<cell::Cell>> {
    let mut cells = Vec::with_capacity(n_rows);
    for i in 0..n_rows {
      let y = (i * settings.universe.height) / n_rows;
      let mut row_cells = Vec::with_capacity(n_cols);
      for j in 0..n_cols {
        let x = (j * settings.universe.width) / n_cols;
        let cell = cell::Cell {
          lives: rand::random(),
          origin: util::Coord { x, y },
          color: settings.cells.color,
          size: settings.cells.size,
        };
        row_cells.push(cell);
      }
      cells.push(row_cells);
    }

    cells
  }


  fn init_alive_neighbors_counts(n_rows: usize, n_cols: usize) -> Vec<Vec<u8>> {
    let mut alive_neighbors_counts = Vec::with_capacity(n_rows);
    for _ in 0..n_rows {
      let mut row_alive_neighbors_counts = Vec::with_capacity(n_cols);
      for _ in 0..n_cols {
        row_alive_neighbors_counts.push(0);
      }
      alive_neighbors_counts.push(row_alive_neighbors_counts);
    }

    alive_neighbors_counts
  }

  pub fn update(&mut self) {
    self.update_alive_neighbors_counts();
    for i in 0..self.n_rows {
      for j in 0..self.n_cols {
        let n_alive_neighbors = self.alive_neighbors_counts[i][j];
        self.cells[i][j].update_life(n_alive_neighbors);
      }
    }
  }

  fn update_alive_neighbors_counts(&mut self) {
    // First row
    self.update_alive_neighbors_counts_top_row();
    // Mid rows
    for i in 1..self.n_rows-1 {
      self.count_cells_alive_neighbors_mid_row(i);
    }
    // Last row
    self.count_cells_alive_neighbors_bottom_row();
  }

  fn update_alive_neighbors_counts_top_row(&mut self) {
    // Top left cell
    self.alive_neighbors_counts[0][0] = self.count_top_left_cell_alive_neighbors();
    // Top most cells
    for j in 1..self.n_cols-1 {
      self.alive_neighbors_counts[0][j] = self.count_top_cell_alive_neighbors(j);
    }
    // Top right cell
    self.alive_neighbors_counts[0][self.n_cols - 1] = self.count_top_right_cell_alive_neighbors();
  }

  fn count_cells_alive_neighbors_mid_row(&mut self, i: usize) {
    // Left most cell
    self.alive_neighbors_counts[i][0] = self.count_left_cell_alive_neighbors(i);
    // Mid cells
    for j in 1..self.n_cols-1 {
      self.alive_neighbors_counts[i][j] = self.count_mid_cell_alive_neighbors(i, j);
    }
    // Right most cell
    self.alive_neighbors_counts[i][self.n_cols - 1] = self.count_right_cell_alive_neighbors(i);
  }

  fn count_cells_alive_neighbors_bottom_row(&mut self) {
    // Bottom left cell
    self.alive_neighbors_counts[self.n_rows - 1][0] = self.count_bottom_left_cell_alive_neighbors();
    // Bottom most cells
    for j in 1..self.n_cols-1 {
      self.alive_neighbors_counts[self.n_rows - 1][j] = self.count_bottom_cell_alive_neighbors(j);
    }
    // Bottom right cell
    self.alive_neighbors_counts[self.n_rows - 1][self.n_cols - 1] = self.count_bottom_right_cell_alive_neighbors();
  }

  fn count_top_left_cell_alive_neighbors(&self) -> u8 {
    let ij_set = [
      (self.n_rows - 1, self.n_cols - 1),  // Relative Top Left
      (self.n_rows - 1, 0),                // Relative Top
      (self.n_rows - 1, 1),                // Relative Top Right
      (0, self.n_cols - 1),                // Relative Left
      (0, 1),                              // Relative Right
      (1, self.n_cols - 1),                // Relative Bottom Left
      (1, 0),                              // Relative Bottom
      (1, 1),                              // Relative Bottom Right
    ];
    let n_alive_neighbors = self.count_cell_alive_neighbors(&ij_set);

    n_alive_neighbors
  }

  fn count_top_cell_alive_neighbors(&self, j: usize) -> u8 {
    let ij_set = [
      (self.n_rows - 1, j - 1),  // Relative Top Left
      (self.n_rows - 1, j),      // Relative Top
      (self.n_rows - 1, j + 1),  // Relative Top Right
      (0, j - 1),                // Relative Left
      (0, j + 1),                // Relative Right
      (1, j - 1),                // Relative Bottom Left
      (1, j),                    // Relative Bottom
      (1, j + 1),                // Relative Bottom Right
    ];
    let n_alive_neighbors = self.count_cell_alive_neighbors(&ij_set);

    n_alive_neighbors
  }

  fn count_top_right_cell_alive_neighbors(&self) -> u8 {
    let ij_set = [
      (self.n_rows - 1, self.n_cols - 2),  // Relative Top Left
      (self.n_rows - 1, self.n_cols - 1),  // Relative Top
      (self.n_rows - 1, 0),                // Relative Top Right
      (0, self.n_cols - 2),                // Relative Left
      (0, 0),                              // Relative Right
      (1, self.n_cols - 2),                // Relative Bottom Left
      (1, self.n_cols - 1),                // Relative Bottom
      (1, 0),                              // Relative Bottom Right
    ];
    let n_alive_neighbors = self.count_cell_alive_neighbors(&ij_set);

    n_alive_neighbors
  }

  fn count_left_cell_alive_neighbors(&self, i: usize) -> u8 {
    let ij_set = [
      (i - 1, self.n_cols - 1),  // Relative Top Left
      (i - 1, 0),                // Relative Top
      (i - 1, 1),                // Relative Top Right
      (i, self.n_cols - 1),      // Relative Left
      (i, 1),                    // Relative Right
      (i + 1, self.n_cols - 1),  // Relative Bottom Left
      (i + 1, 0),                // Relative Bottom
      (i + 1, 1),                // Relative Bottom Right
    ];
    let n_alive_neighbors = self.count_cell_alive_neighbors(&ij_set);

    n_alive_neighbors
  }

  fn count_mid_cell_alive_neighbors(&self, i: usize, j: usize) -> u8 {
    let ij_set = [
      (i - 1, j - 1),  // Relative Top Left
      (i - 1, j),      // Relative Top
      (i - 1, j + 1),  // Relative Top Right
      (i, j - 1),      // Relative Left
      (i, j + 1),      // Relative Right
      (i + 1, j - 1),  // Relative Bottom Left
      (i + 1, j),      // Relative Bottom
      (i + 1, j + 1),  // Relative Bottom Right
    ];
    let n_alive_neighbors = self.count_cell_alive_neighbors(&ij_set);

    n_alive_neighbors
  }

  fn count_right_cell_alive_neighbors(&self, i: usize) -> u8 {
    let ij_set = [
      (i - 1, self.n_cols - 2),  // Relative Top Left
      (i - 1, self.n_cols - 1),  // Relative Top
      (i - 1, 0),                // Relative Top Right
      (i, self.n_cols - 2),      // Relative Left
      (i, 0),                    // Relative Right
      (i + 1, self.n_cols - 2),  // Relative Bottom Left
      (i + 1, self.n_cols - 1),  // Relative Bottom
      (i + 1, 0),                // Relative Bottom Right
    ];
    let n_alive_neighbors = self.count_cell_alive_neighbors(&ij_set);

    n_alive_neighbors
  }

  fn count_bottom_left_cell_alive_neighbors(&self) -> u8 {
    let ij_set = [
      (self.n_rows - 2, self.n_cols - 1),  // Relative Top Left
      (self.n_rows - 2, 0),                // Relative Top
      (self.n_rows - 2, 1),                // Relative Top Right
      (self.n_rows - 1, self.n_cols - 1),  // Relative Left
      (self.n_rows - 1, 1),                // Relative Right
      (0, self.n_cols - 1),                // Relative Bottom Left
      (0, 0),                              // Relative Bottom
      (0, 1),                              // Relative Bottom Right
    ];
    let n_alive_neighbors = self.count_cell_alive_neighbors(&ij_set);

    n_alive_neighbors
  }

  fn count_bottom_cell_alive_neighbors(&self, j: usize) -> u8 {
    let ij_set = [
      (self.n_rows - 2, j - 1),  // Relative Top Left
      (self.n_rows - 2, j),      // Relative Top
      (self.n_rows - 2, j + 1),  // Relative Top Right
      (self.n_rows - 1, j - 1),  // Relative Left
      (self.n_rows - 1, j + 1),  // Relative Right
      (0, j - 1),                // Relative Bottom Left
      (0, j),                    // Relative Bottom
      (0, j + 1),                // Relative Bottom Right
    ];
    let n_alive_neighbors = self.count_cell_alive_neighbors(&ij_set);

    n_alive_neighbors
  }

  fn count_bottom_right_cell_alive_neighbors(&self) -> u8 {
    let ij_set = [
      (self.n_rows - 2, self.n_cols - 2),  // Relative Top Left
      (self.n_rows - 2, self.n_cols - 1),  // Relative Top
      (self.n_rows - 2, 0),                // Relative Top Right
      (self.n_rows - 1, self.n_cols - 2),  // Relative Left
      (self.n_rows - 1, 0),                // Relative Right
      (0, self.n_cols - 2),                // Relative Bottom Left
      (0, self.n_cols - 1),                // Relative Bottom
      (0, 0),                              // Relative Bottom Right
    ];
    let n_alive_neighbors = self.count_cell_alive_neighbors(&ij_set);

    n_alive_neighbors
  }

  fn count_cell_alive_neighbors(&self, ij_set: &[(usize, usize)]) -> u8 {
    let mut count = 0;
    for (i, j) in ij_set {
      count += self.cells[*i][*j].lives as u8;
    }

    count
  }

}

impl Drawable for Population {
  fn draw(&self, graphics: &mut speedy2d::Graphics2D) {
    for cell_row in self.cells.iter() {
      for cell in cell_row {
        cell.draw(graphics);
      }
    }
  }
}
