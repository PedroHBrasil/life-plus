use crate::util::{Coord, Color};

#[derive(Default, Debug, Copy, Clone)]
pub struct Cell {
  pub lives: bool,
  pub origin: Coord,
  pub color: Color,
  pub size: usize,
}

impl Cell {
  /// Updates the life state of a cell
  pub fn update_life(&mut self, n_alive_neighbors: u8) {
    // This line summarizes the original rules found at https://conwaylife.com/
    self.lives = n_alive_neighbors == 3 || (n_alive_neighbors == 2 && self.lives);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn update_life() {
    let mut cell = Cell::default();

    // Tests for initially dead cell
    for i in 0..=9 {
      cell.update_life(i);
      if i == 3 {
        assert!(cell.lives);
      } else {
        assert!(!cell.lives);
      }
    }

    // Tests for initially live cell
    for i in 0..=9 {
      cell.lives = true;
      cell.update_life(i);
      if i == 2 || i == 3 {
        assert!(cell.lives);
      } else {
        assert!(!cell.lives);
      }
    }
  }
}