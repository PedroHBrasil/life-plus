use crate::{util::Coord, settings};

mod line;

pub struct Grid {
  lines: Vec<line::Line>,
}

impl Grid {
  pub fn new(settings: &settings::Settings) -> Self {
    Self {
      lines: Self::make_lines(settings)
    }
  }

  fn make_lines(settings: &settings::Settings) -> Vec<line::Line> {
    let n_rows = settings.universe.height / settings.cells.size;
    let n_cols = settings.universe.width / settings.cells.size;

    let mut lines = Vec::with_capacity(n_rows + n_cols);
    Self::make_horizontal_lines(&mut lines, n_rows, settings);
    Self::make_vertical_lines(&mut lines, n_cols, settings);

    lines
  }

  fn make_horizontal_lines(lines: &mut Vec<line::Line>, n_rows: usize, settings: &settings::Settings) {
    let n_horizontal_lines = n_rows + 1;
    let lines_spacing = settings.universe.height / n_rows;
    for i in 0..n_horizontal_lines {
      let y = i * lines_spacing;
      let line = line::Line {
        begin: Coord {x: 0, y},
        end: Coord {x: settings.universe.width, y},
        thickness: settings.lines.thickness,
        color: settings.lines.color
      };
      lines.push(line);
    }
  }

  fn make_vertical_lines(lines: &mut Vec<line::Line>, n_cols: usize, settings: &settings::Settings) {
    let n_vertical_lines = n_cols + 1;
    let lines_spacing = settings.universe.width / n_cols;
    for i in 0..n_vertical_lines {
      let x = i * lines_spacing;
      let line = line::Line {
        begin: Coord {x, y: 0},
        end: Coord {x, y: settings.universe.height},
        thickness: settings.lines.thickness,
        color: settings.lines.color
      };
      lines.push(line);
    }
  }
}
