use crate::{settings, util::{Color, Drawable}};
use speedy2d::Graphics2D;

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

impl Drawable for Universe {
  fn draw(&self, graphics: &mut Graphics2D) {
    graphics.clear_screen(self.color.as_speedy2d_color());
    self.population.draw(graphics);
    self.grid.draw(graphics);
  }
}