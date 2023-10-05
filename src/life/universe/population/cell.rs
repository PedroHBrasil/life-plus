use crate::util::{Coord, Color};

pub struct Cell {
  pub lives: bool,
  origin: Coord,
  size: u16,
  color: Color,
}

impl Cell {
  pub fn new() -> Self {
    unimplemented!()
  }

  pub fn update_life(&mut self, n_alive_neighbors: u8) {
    unimplemented!()
  }
}