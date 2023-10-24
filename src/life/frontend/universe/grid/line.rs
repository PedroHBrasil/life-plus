use dioxus::prelude::*;
use crate::life::frontend::util;

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