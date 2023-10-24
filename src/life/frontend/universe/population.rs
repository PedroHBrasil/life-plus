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
  let cell_color = settings.cells.color;
  let cell_size = settings.cells.size;

  let n_rows = cx.props.height / cell_size + 1;
  let n_cols = cx.props.width / cell_size + 1;

  let cells = make_cells(n_rows, n_cols, cx.props.height, cx.props.width, cell_size);
  cx.render(rsx!(
    svg {
      x: 0 as i64,
      y: 0 as i64,
      width: cx.props.width as i64,
      height: cx.props.height as i64,
      style {
        position: "absolute",
        top: 0,
        left: 0,
        z_index: 0,
      },
      for cell_row in cells {
        for cell in cell_row {
          cell::CellUi {
            lives: cell.lives,
            origin: cell.origin,
            size: cell.size,
          }
        }
      }
    }
  ))
}

fn make_cells(n_rows: usize, n_cols: usize, canvas_height: usize, canvas_width: usize, cell_size: usize) -> Vec<Vec<cell::CellUiProps>> {
  let mut cells = Vec::with_capacity(n_rows);
  for i in 0..n_rows {
    let y = (i * canvas_height) / n_rows;
    let mut row_cells = Vec::with_capacity(n_cols);
    for j in 0..n_cols {
      let x = (j * canvas_width) / n_cols;
      let cell = cell::CellUiProps {
        lives: rand::random(),
        origin: Coord { x, y },
        size: cell_size,
      };
      row_cells.push(cell);
    }
    cells.push(row_cells);
  }

  cells
}
