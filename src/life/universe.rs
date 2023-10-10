use crate::settings;

mod population;
mod grid;

pub struct Universe {
  population: population::Population,
  grid: grid::Grid,
}

impl Universe {
  pub fn new(settings: settings::Settings) -> Self {
    Self {
      population: population::Population::new(&settings),
      grid: grid::Grid::new(&settings),
    }
  }
}