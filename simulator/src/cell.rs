
/// Defines the possible cell states
#[derive(Clone, Debug, PartialEq)]
pub enum CellState {
  Dead,
  Alive,
}

pub fn next_state(cell_state: &CellState, neighbor_count: u8) -> CellState {
  match (cell_state, neighbor_count) {
    (CellState::Alive, 2) | (_, 3) => CellState::Alive,
    (_, _) => CellState::Dead,
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn next_state_from_alive() {
    let ref_state = CellState::Alive;
    let max_neighbor_count = 9;

    for neighbor_count in 0..max_neighbor_count {
      let expected = match neighbor_count {
        2 | 3 => CellState::Alive,
        _ => CellState::Dead,
      };
      let result = next_state(&ref_state, neighbor_count);

      assert_eq!(result, expected);
    }
  }

  #[test]
  fn next_state_from_dead() {
    let ref_state = CellState::Dead;
    let max_neighbor_count = 9;

    for neighbor_count in 0..max_neighbor_count {
      let expected = match neighbor_count {
        3 => CellState::Alive,
        _ => CellState::Dead,
      };
      let result = next_state(&ref_state, neighbor_count);

      assert_eq!(result, expected);
    }
  }
}