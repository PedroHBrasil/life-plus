use dioxus::prelude::*;
mod population;
mod grid;

pub struct Universe {
  population: population::Population,
  grid: grid::Grid,
}

impl Universe {
  pub fn new(n_rows: usize, n_cols: usize, canvas_height: usize, canvas_width: usize, cell_size: usize) -> Self {
    Self {
      population: population::Population::new(n_rows, n_cols, canvas_height, canvas_width, cell_size),
      grid: grid::Grid::new(n_rows, n_cols, canvas_height, canvas_width),
    }
  }
}
