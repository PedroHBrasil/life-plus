use dioxus::prelude::*;
use crate::life::frontend::{util::Coord, settings};

mod cell;

#[derive(PartialEq, Props)]
pub struct PopulationUiProps {
  width: usize,
  height: usize,
}

pub fn PopulationUi(cx: Scope<PopulationUiProps>) -> Element {
  let settings = use_shared_state::<settings::Settings>(cx).unwrap().read();
  let cell_size = settings.cells.size;

  let n_rows = cx.props.height / cell_size + 1;
  let n_cols = cx.props.width / cell_size + 1;

  let life_state = use_ref(cx, || make_rand_life_state(n_rows, n_cols));
  let cells = make_cells(life_state.read().as_ref(), n_rows, n_cols, cx.props.height, cx.props.width, cell_size);
  let mut alive_neighbors_counts = get_alive_neighbors_counts(n_rows, n_cols);

  cx.render(rsx!(
    svg {
      x: 0 as i64,
      y: 0 as i64,
      width: cx.props.width as i64,
      height: cx.props.height as i64,
      onclick: move |_event| {
        update_life_state(life_state, &mut alive_neighbors_counts);
        life_state.needs_update();
      },
      for i in 0..n_rows {
        for j in 0..n_cols {
          cell::CellUi {
            lives: life_state.read()[i][j],
            origin: cells[i][j].origin,
            size: cells[i][j].size,
          }
        }
      }
    }
  ))
}

fn make_rand_life_state(n_rows: usize, n_cols: usize) -> Vec<Vec<bool>> {
  let mut life_state = Vec::with_capacity(n_rows);
  for _ in 0..n_rows {
    let mut row_life_state = Vec::with_capacity(n_cols);
    for _ in 0..n_cols {
      row_life_state.push(rand::random());
    }
    life_state.push(row_life_state);
  }

  life_state
}

fn make_cells(life_state: &Vec<Vec<bool>>, n_rows: usize, n_cols: usize, canvas_height: usize, canvas_width: usize, cell_size: usize) -> Vec<Vec<cell::CellUiProps>> {
  let mut cells = Vec::with_capacity(n_rows);
  for i in 0..n_rows {
    let y = (i * canvas_height) / n_rows;
    let mut row_cells = Vec::with_capacity(n_cols);
    for j in 0..n_cols {
      let x = (j * canvas_width) / n_cols;
      let cell = cell::CellUiProps {
        lives: life_state[i][j],
        origin: Coord { x, y },
        size: cell_size,
      };
      row_cells.push(cell);
    }
    cells.push(row_cells);
  }

  cells
}

fn get_alive_neighbors_counts(n_rows: usize, n_cols: usize) -> Vec<Vec<u8>> {
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

fn update_life_state(life_state: &UseRef<Vec<Vec<bool>>>, alive_neighbors_counts: &mut Vec<Vec<u8>>) {
  let n_rows = life_state.read().len();
  let n_cols = life_state.read()[0].len();

  update_alive_neighbors_counts(life_state, alive_neighbors_counts, n_rows, n_cols);

  let mut life_state_mut = life_state.write_silent();
  for i in 0..n_rows {
    for j in 0..n_cols {
      life_state_mut[i][j] = cell::update_cell_life(life_state_mut[i][j], alive_neighbors_counts[i][j]);
    }
  }

}

fn update_alive_neighbors_counts(life_state: &UseRef<Vec<Vec<bool>>>, alive_neighbors_counts: &mut Vec<Vec<u8>>, n_rows: usize, n_cols: usize) {
  // First row
  update_alive_neighbors_counts_top_row(life_state, alive_neighbors_counts, n_rows, n_cols);
  // Mid rows
  for i in 1..n_rows-1 {
    update_alive_neighbors_counts_mid_row(life_state, alive_neighbors_counts, n_cols, i);
  }
  // Last row
  update_alive_neighbors_counts_bottom_row(life_state, alive_neighbors_counts, n_rows, n_cols);
}

fn update_alive_neighbors_counts_top_row(life_state: &UseRef<Vec<Vec<bool>>>, alive_neighbors_counts: &mut Vec<Vec<u8>>, n_rows: usize, n_cols: usize) {
  // Top left cell
  alive_neighbors_counts[0][0] = count_top_left_cell_alive_neighbors(life_state, n_rows, n_cols);
  // Top most cells
  for j in 1..n_cols-1 {
    alive_neighbors_counts[0][j] = count_top_cell_alive_neighbors(life_state, n_rows, j);
  }
  // Top right cell
  alive_neighbors_counts[0][n_cols - 1] = count_top_right_cell_alive_neighbors(life_state, n_rows, n_cols);
}

fn update_alive_neighbors_counts_mid_row(life_state: &UseRef<Vec<Vec<bool>>>, alive_neighbors_counts: &mut Vec<Vec<u8>>, n_cols: usize, i: usize) {
  // Left most cell
  alive_neighbors_counts[i][0] = count_left_cell_alive_neighbors(life_state, n_cols, i);
  // Mid cells
  for j in 1..n_cols-1 {
    alive_neighbors_counts[i][j] = count_mid_cell_alive_neighbors(life_state, i, j);
  }
  // Right most cell
  alive_neighbors_counts[i][n_cols - 1] = count_right_cell_alive_neighbors(life_state, n_cols, i);
}

fn update_alive_neighbors_counts_bottom_row(life_state: &UseRef<Vec<Vec<bool>>>, alive_neighbors_counts: &mut Vec<Vec<u8>>, n_rows: usize, n_cols: usize) {
  // Bottom left cell
  alive_neighbors_counts[n_rows - 1][0] = count_bottom_left_cell_alive_neighbors(life_state, n_rows, n_cols);
  // Bottom most cells
  for j in 1..n_cols-1 {
    alive_neighbors_counts[n_rows - 1][j] = count_bottom_cell_alive_neighbors(life_state, n_rows, j);
  }
  // Bottom right cell
  alive_neighbors_counts[n_rows - 1][n_cols - 1] = count_bottom_right_cell_alive_neighbors(life_state, n_rows, n_cols);
}

fn count_top_left_cell_alive_neighbors(life_state: &UseRef<Vec<Vec<bool>>>, n_rows: usize, n_cols: usize) -> u8 {
  let ij_set = [
    (n_rows - 1, n_cols - 1),  // Relative Top Left
    (n_rows - 1, 0),                // Relative Top
    (n_rows - 1, 1),                // Relative Top Right
    (0, n_cols - 1),                // Relative Left
    (0, 1),                              // Relative Right
    (1, n_cols - 1),                // Relative Bottom Left
    (1, 0),                              // Relative Bottom
    (1, 1),                              // Relative Bottom Right
  ];
  let n_alive_neighbors = count_cell_alive_neighbors(life_state, &ij_set);

  n_alive_neighbors
}

fn count_top_cell_alive_neighbors(life_state: &UseRef<Vec<Vec<bool>>>, n_rows: usize, j: usize) -> u8 {
  let ij_set = [
    (n_rows - 1, j - 1),  // Relative Top Left
    (n_rows - 1, j),      // Relative Top
    (n_rows - 1, j + 1),  // Relative Top Right
    (0, j - 1),                // Relative Left
    (0, j + 1),                // Relative Right
    (1, j - 1),                // Relative Bottom Left
    (1, j),                    // Relative Bottom
    (1, j + 1),                // Relative Bottom Right
  ];
  let n_alive_neighbors = count_cell_alive_neighbors(life_state, &ij_set);

  n_alive_neighbors
}

fn count_top_right_cell_alive_neighbors(life_state: &UseRef<Vec<Vec<bool>>>, n_rows: usize, n_cols: usize) -> u8 {
  let ij_set = [
    (n_rows - 1, n_cols - 2),  // Relative Top Left
    (n_rows - 1, n_cols - 1),  // Relative Top
    (n_rows - 1, 0),                // Relative Top Right
    (0, n_cols - 2),                // Relative Left
    (0, 0),                              // Relative Right
    (1, n_cols - 2),                // Relative Bottom Left
    (1, n_cols - 1),                // Relative Bottom
    (1, 0),                              // Relative Bottom Right
  ];
  let n_alive_neighbors = count_cell_alive_neighbors(life_state, &ij_set);

  n_alive_neighbors
}

fn count_left_cell_alive_neighbors(life_state: &UseRef<Vec<Vec<bool>>>, n_cols: usize, i: usize) -> u8 {
  let ij_set = [
    (i - 1, n_cols - 1),  // Relative Top Left
    (i - 1, 0),                // Relative Top
    (i - 1, 1),                // Relative Top Right
    (i, n_cols - 1),      // Relative Left
    (i, 1),                    // Relative Right
    (i + 1, n_cols - 1),  // Relative Bottom Left
    (i + 1, 0),                // Relative Bottom
    (i + 1, 1),                // Relative Bottom Right
  ];
  let n_alive_neighbors = count_cell_alive_neighbors(life_state, &ij_set);

  n_alive_neighbors
}

fn count_mid_cell_alive_neighbors(life_state: &UseRef<Vec<Vec<bool>>>, i: usize, j: usize) -> u8 {
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
  let n_alive_neighbors = count_cell_alive_neighbors(life_state, &ij_set);

  n_alive_neighbors
}

fn count_right_cell_alive_neighbors(life_state: &UseRef<Vec<Vec<bool>>>, n_cols: usize, i: usize) -> u8 {
  let ij_set = [
    (i - 1, n_cols - 2),  // Relative Top Left
    (i - 1, n_cols - 1),  // Relative Top
    (i - 1, 0),                // Relative Top Right
    (i, n_cols - 2),      // Relative Left
    (i, 0),                    // Relative Right
    (i + 1, n_cols - 2),  // Relative Bottom Left
    (i + 1, n_cols - 1),  // Relative Bottom
    (i + 1, 0),                // Relative Bottom Right
  ];
  let n_alive_neighbors = count_cell_alive_neighbors(life_state, &ij_set);

  n_alive_neighbors
}

fn count_bottom_left_cell_alive_neighbors(life_state: &UseRef<Vec<Vec<bool>>>, n_rows: usize, n_cols: usize) -> u8 {
  let ij_set = [
    (n_rows - 2, n_cols - 1),  // Relative Top Left
    (n_rows - 2, 0),                // Relative Top
    (n_rows - 2, 1),                // Relative Top Right
    (n_rows - 1, n_cols - 1),  // Relative Left
    (n_rows - 1, 1),                // Relative Right
    (0, n_cols - 1),                // Relative Bottom Left
    (0, 0),                              // Relative Bottom
    (0, 1),                              // Relative Bottom Right
  ];
  let n_alive_neighbors = count_cell_alive_neighbors(life_state, &ij_set);

  n_alive_neighbors
}

fn count_bottom_cell_alive_neighbors(life_state: &UseRef<Vec<Vec<bool>>>, n_rows: usize, j: usize) -> u8 {
  let ij_set = [
    (n_rows - 2, j - 1),  // Relative Top Left
    (n_rows - 2, j),      // Relative Top
    (n_rows - 2, j + 1),  // Relative Top Right
    (n_rows - 1, j - 1),  // Relative Left
    (n_rows - 1, j + 1),  // Relative Right
    (0, j - 1),                // Relative Bottom Left
    (0, j),                    // Relative Bottom
    (0, j + 1),                // Relative Bottom Right
  ];
  let n_alive_neighbors = count_cell_alive_neighbors(life_state, &ij_set);

  n_alive_neighbors
}

fn count_bottom_right_cell_alive_neighbors(life_state: &UseRef<Vec<Vec<bool>>>, n_rows: usize, n_cols: usize) -> u8 {
  let ij_set = [
    (n_rows - 2, n_cols - 2),  // Relative Top Left
    (n_rows - 2, n_cols - 1),  // Relative Top
    (n_rows - 2, 0),           // Relative Top Right
    (n_rows - 1, n_cols - 2),  // Relative Left
    (n_rows - 1, 0),           // Relative Right
    (0, n_cols - 2),           // Relative Bottom Left
    (0, n_cols - 1),           // Relative Bottom
    (0, 0),                    // Relative Bottom Right
  ];
  let n_alive_neighbors = count_cell_alive_neighbors(life_state, &ij_set);

  n_alive_neighbors
}

fn count_cell_alive_neighbors(life_state: &UseRef<Vec<Vec<bool>>>, ij_set: &[(usize, usize)]) -> u8 {
  let mut count = 0;
  for (i, j) in ij_set {
    count += life_state.read()[*i][*j] as u8;
  }

  count
}
