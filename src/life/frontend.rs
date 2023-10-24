use dioxus::prelude::*;
mod universe;
mod util;
mod settings;

pub fn Ui(cx: Scope) -> Element {
  use_shared_state_provider(cx, || settings::Settings::default());

  // let window_size = use_callback(cx, || {
  //   let window = web_sys::window().unwrap();
  //   let width = window.inner_width().unwrap().as_f64().unwrap() as usize;
  //   let height = window.inner_height().unwrap().as_f64().unwrap() as usize;

  //   (width, height)
  // });
  let (window_width, window_height) = util::get_window_size();

  cx.render(rsx!(
    GridUi {
      cell_size: 10,
      width: window_width,
      height: window_height,
    }
  ))
}

#[derive(PartialEq, Props)]
pub struct GridUi {
  cell_size: usize,
  width: usize,
  height: usize,
}

pub fn GridUi(cx: Scope<GridUi>) -> Element {
  let settings = use_shared_state::<settings::Settings>(cx);
  let line_color = settings.unwrap().read().lines.color;
  let lines = make_lines(cx.props.cell_size, cx.props.height, cx.props.width);

  cx.render(rsx!(
    svg {
      x: 0 as i64,
      y: 0 as i64,
      width: cx.props.width as i64,
      height: cx.props.height as i64,
      stroke: line_color,
      for line in lines.iter() {
        LineUi {
          begin: line.begin,
          end: line.end,
        }
      }
    }
  ))
}


fn make_lines(cell_size: usize, canvas_height: usize, canvas_width: usize) -> Vec<LineUiProps> {
  let n_rows = canvas_height / cell_size + 1;
  let n_cols = canvas_width / cell_size + 1;
  let mut lines: Vec<LineUiProps> = Vec::with_capacity(n_rows + n_cols);
  make_horizontal_lines(&mut lines, n_rows, canvas_height, canvas_width);
  make_vertical_lines(&mut lines, n_cols, canvas_height, canvas_width);

  lines
}

fn make_horizontal_lines(lines: &mut Vec<LineUiProps>, n_rows: usize, canvas_height: usize, canvas_width: usize) {
  let n_horizontal_lines = n_rows + 1;
  for i in 0..n_horizontal_lines {
    let y = (i * canvas_height) / n_rows;
    let line = LineUiProps {
      begin: util::Coord {x: 0, y},
      end: util::Coord {x: canvas_width, y},
    };
    lines.push(line);
  }
}

fn make_vertical_lines(lines: &mut Vec<LineUiProps>, n_cols: usize, canvas_height: usize, canvas_width: usize) {
  let n_vertical_lines = n_cols + 1;
  for i in 0..n_vertical_lines {
    let x = (i * canvas_width) / n_cols;
    let line = LineUiProps {
      begin: util::Coord {x, y: 0},
      end: util::Coord {x, y: canvas_height},
    };
    lines.push(line);
  }
}

#[derive(PartialEq, Props)]
pub struct LineUiProps {
  pub begin: util::Coord,
  pub end: util::Coord,
}

pub fn LineUi(cx: Scope<LineUiProps>) -> Element {
  cx.render(rsx!(
    line {
      x1: cx.props.begin.x as i64,
      y1: cx.props.begin.y as i64,
      x2: cx.props.end.x as i64,
      y2: cx.props.end.y as i64,
    }
  ))
}