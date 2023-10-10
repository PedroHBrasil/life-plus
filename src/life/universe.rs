use crate::{settings, util::Color};
mod population;
mod grid;

pub struct Universe {
  population: population::Population,
  grid: grid::Grid,
  color: Color,
}

impl Universe {
  pub fn new(settings: &settings::Settings) -> Self {
    Self {
      population: population::Population::new(settings),
      grid: grid::Grid::new(settings),
      color: settings.universe.color,
    }
  }

  pub fn update_population(&mut self) {
    self.population.update();
  }
}
