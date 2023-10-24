use dioxus::prelude::*;
use crate::life::frontend::{settings, util};
mod line;

#[derive(PartialEq, Props)]
pub struct GridUi {
  width: usize,
  height: usize,
}

pub fn GridUi(cx: Scope<GridUi>) -> Element {
  let settings = use_shared_state::<settings::Settings>(cx).unwrap().read();
  let line_color = settings.lines.color;
  let cell_size = settings.cells.size;
  let lines = make_lines(cell_size, cx.props.height, cx.props.width);

  cx.render(rsx!(
    svg {
      x: 0 as i64,
      y: 0 as i64,
      width: cx.props.width as i64,
      height: cx.props.height as i64,
      stroke: line_color,
      for line in lines.iter() {
        line::LineUi {
          begin: line.begin,
          end: line.end,
        }
      }
    }
  ))
}


fn make_lines(cell_size: usize, canvas_height: usize, canvas_width: usize) -> Vec<line::LineUiProps> {
  let n_rows = canvas_height / cell_size + 1;
  let n_cols = canvas_width / cell_size + 1;
  let mut lines: Vec<line::LineUiProps> = Vec::with_capacity(n_rows + n_cols);
  make_horizontal_lines(&mut lines, n_rows, canvas_height, canvas_width);
  make_vertical_lines(&mut lines, n_cols, canvas_height, canvas_width);

  lines
}

fn make_horizontal_lines(lines: &mut Vec<line::LineUiProps>, n_rows: usize, canvas_height: usize, canvas_width: usize) {
  let n_horizontal_lines = n_rows + 1;
  for i in 0..n_horizontal_lines {
    let y = (i * canvas_height) / n_rows;
    let line = line::LineUiProps {
      begin: util::Coord {x: 0, y},
      end: util::Coord {x: canvas_width, y},
    };
    lines.push(line);
  }
}

fn make_vertical_lines(lines: &mut Vec<line::LineUiProps>, n_cols: usize, canvas_height: usize, canvas_width: usize) {
  let n_vertical_lines = n_cols + 1;
  for i in 0..n_vertical_lines {
    let x = (i * canvas_width) / n_cols;
    let line = line::LineUiProps {
      begin: util::Coord {x, y: 0},
      end: util::Coord {x, y: canvas_height},
    };
    lines.push(line);
  }
}
