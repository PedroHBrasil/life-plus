
/// Defines the possible cell states
#[derive(Clone, Debug, PartialEq)]
pub enum CellState {
  Dead,
  Alive,
}

pub fn next_state(cell_state: &CellState, neighbor_count: u8) -> CellState {
  match (cell_state, neighbor_count) {
    (CellState::Alive, 2 | 3) => CellState::Alive,
    (_, _) => CellState::Dead,
  }
}