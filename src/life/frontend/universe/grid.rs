use dioxus::prelude::*;
use crate::life::frontend::util::Coord;

mod line;

pub struct Grid {
  lines: Vec<line::LineProps>,
  width: usize,
  height: usize,
}

impl Grid {
  pub fn new(n_rows: usize, n_cols: usize, canvas_height: usize, canvas_width: usize) -> Self {
    let mut grid = Self {
      lines: Vec::with_capacity(n_rows + n_cols),
      width: canvas_width,
      height: canvas_height,
    };
    grid.make_lines(n_rows, n_cols, canvas_height, canvas_width);

    grid
  }

  fn make_lines(&mut self, n_rows: usize, n_cols: usize, canvas_height: usize, canvas_width: usize) {
    self.make_horizontal_lines(n_rows, canvas_height, canvas_width);
    self.make_vertical_lines(n_cols, canvas_height, canvas_width);
  }

  fn make_horizontal_lines(&mut self, n_rows: usize, canvas_height: usize, canvas_width: usize) {
    let n_horizontal_lines = n_rows + 1;
    let lines_spacing = canvas_height / n_rows;
    for i in 0..n_horizontal_lines {
      let y = i * lines_spacing;
      let line = line::LineProps {
        begin: Coord {x: 0, y},
        end: Coord {x: canvas_width, y},
      };
      self.lines.push(line);
    }
  }

  fn make_vertical_lines(&mut self, n_cols: usize, canvas_height: usize, canvas_width: usize) {
    let n_vertical_lines = n_cols + 1;
    let lines_spacing = canvas_width / n_cols;
    for i in 0..n_vertical_lines {
      let x = i * lines_spacing;
      let line = line::LineProps {
        begin: Coord {x, y: 0},
        end: Coord {x, y: canvas_height},
      };
      self.lines.push(line);
    }
  }
}
