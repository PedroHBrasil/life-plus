use dioxus::prelude::*;

use crate::life::frontend::{util::Coord, settings};

#[derive(PartialEq, Props)]
pub struct CellUiProps {
  pub lives: bool,
  pub origin: Coord,
  pub size: usize,
}

pub fn CellUi(cx: Scope<CellUiProps>) -> Element {
  let settings = use_shared_state::<settings::Settings>(cx).unwrap().read();
  let lives = cx.props.lives;
  let lives_state = use_state(cx, || lives);
  let color = settings.cells.color;
  let color_state = use_state(cx, || color);
  cx.render(rsx!(
    svg {
      x: cx.props.origin.x as i64,
      y: cx.props.origin.y as i64,
      width: cx.props.size as i64,
      height: cx.props.size as i64,
      fill: if cx.props.lives {color_state} else {"#00000000"},
      stroke: settings.lines.color,
      stroke_width: settings.lines.thickness as i64,
      rect {
        x: 0,
        y: 0,
        width: cx.props.size as i64,
        height: cx.props.size as i64,
      }
    }
  ))
}

pub fn update_cell_life(lives: bool, n_alive_neighbors: u8) -> bool {
  // This line summarizes the original rules found at https://conwaylife.com/
  n_alive_neighbors == 3 || (n_alive_neighbors == 2 && lives)
}