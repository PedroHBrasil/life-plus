use std::iter;

/// This crate contains the app's simulation logic.
mod cell;
mod neighborhood;

pub struct Life {
  pub population: Vec<Vec<cell::CellState>>,
  neighbor_counts: Vec<Vec<u8>>,
  n_rows: usize,
  n_cols: usize,
  i_max: usize,
  j_max: usize,
}

impl Life {
  pub fn build(n_rows: usize, n_cols: usize) -> Result<Self, &'static str> {
    if n_rows == 0 || n_cols == 0 {
      return Err("Numbers of rows and columns must be greater than 0.");
    }

    let life = Self {
      population: Self::make_population(n_rows, n_cols),
      neighbor_counts: Self::make_neighbors_counts(n_rows, n_cols),
      n_rows,
      n_cols,
      i_max: n_rows - 1,
      j_max: n_cols - 1,
    };

    Ok(life)
  }

  /// Initializes the cells
  fn make_population(n_rows: usize, n_cols: usize) -> Vec<Vec<cell::CellState>> {
    let row_iter = iter::repeat(cell::CellState::Dead).take(n_cols);
    let row_templ = Vec::from_iter(row_iter);
    let iter = iter::repeat(row_templ).take(n_rows);
    let cells = Vec::from_iter(iter);

    cells
  }

  /// Initializes neighbors counts
  fn make_neighbors_counts(n_rows: usize, n_cols: usize) -> Vec<Vec<u8>> {
    let row_iter = iter::repeat(0).take(n_cols);
    let row_templ = Vec::from_iter(row_iter);
    let iter = iter::repeat(row_templ).take(n_rows);
    let neighbors_counts = Vec::from_iter(iter);

    neighbors_counts
  }

  /// Randomize the population cells' states
  pub fn randomize(&mut self) {
    for row in self.population.iter_mut() {
      for cell_state in row.iter_mut() {
        let choice = rand::random();
        if choice {
          *cell_state = cell::CellState::Alive;
        } else {
          *cell_state = cell::CellState::Dead;
        }
      }
    }
  }

  /// Updates the simulation state
  pub fn update(&mut self) {
    self.update_neighbors_counts();
    self.update_population();
  }

  /// Updates the counts of neighbors of each cell
  fn update_neighbors_counts(&mut self) {
    self.update_neighbor_counts_top_row();
    for i in 1..self.i_max {
      self.update_neighbor_counts_mid_row(i);
    }
    self.update_neighbor_counts_bot_row();
  }

  /// Updates neighbor counts of the top row
  fn update_neighbor_counts_top_row(&mut self) {
    // Left cell
    let mut neighborhood = neighborhood::top_left(self.i_max, self.j_max);
    self.neighbor_counts[0][0] = self.count_neighbors(neighborhood);
    // Mid cells
    for j in 1..self.j_max {
      neighborhood = neighborhood::top_edge(self.i_max, j);
      self.neighbor_counts[0][j] = self.count_neighbors(neighborhood);
    }
    // Right cell
    neighborhood = neighborhood::top_right(self.i_max, self.j_max);
    self.neighbor_counts[0][self.j_max] = self.count_neighbors(neighborhood);
  }

  /// Updates neighbor counts of a middle row
  fn update_neighbor_counts_mid_row(&mut self, i: usize) {
    // Left cell
    let mut neighborhood = neighborhood::left_edge(i, self.j_max);
    self.neighbor_counts[i][0] = self.count_neighbors(neighborhood);
    // Mid cells
    for j in 1..self.j_max {
      neighborhood = neighborhood::mid(i, j);
      self.neighbor_counts[i][j] = self.count_neighbors(neighborhood);
    }
    // Right cell
    neighborhood = neighborhood::right_edge(i, self.j_max);
    self.neighbor_counts[i][self.j_max] = self.count_neighbors(neighborhood);
  }

  /// Updates neighbor counts of the bottom row
  fn update_neighbor_counts_bot_row(&mut self) {
    // Left cell
    let mut neighborhood = neighborhood::bot_left(self.i_max, self.j_max);
    self.neighbor_counts[self.i_max][0] = self.count_neighbors(neighborhood);
    // Mid cells
    for j in 1..self.j_max {
      neighborhood = neighborhood::bot_edge(self.i_max, j);
      self.neighbor_counts[self.i_max][j] = self.count_neighbors(neighborhood);
    }
    // Right cell
    neighborhood = neighborhood::bot_right(self.i_max, self.j_max);
    self.neighbor_counts[self.i_max][self.j_max] = self.count_neighbors(neighborhood);
  }

  /// Counts number of alive neighbors
  fn count_neighbors(&self, neighborhood: [(usize, usize); 8]) -> u8 {
    let mut count = 0;
    for (i, j) in neighborhood {
      match self.population[i][j] {
        cell::CellState::Alive => count += 1,
        cell::CellState::Dead => (),
      }
    }

    count
  }

  /// Updates the states of the cells according to game of life's rules
  fn update_population(&mut self) {
    for i in 0..self.n_rows {
      for j in 0..self.n_cols {
        self.population[i][j] = cell::next_state(&self.population[i][j], self.neighbor_counts[i][j]);
      }
    }
  }
}
