use dioxus::prelude::*;
use crate::life::frontend::util::Coord;

#[derive(PartialEq, Props)]
pub struct LineProps {
  pub begin: Coord,
  pub end: Coord,
}

pub fn LineUi(cx: Scope<LineProps>) -> Element {
  cx.render(rsx!(
    svg {
      stroke: "black",
      line {
        x1: cx.props.begin.x as i64,
        y1: cx.props.begin.y as i64,
        x2: cx.props.end.x as i64,
        y2: cx.props.end.y as i64,
      }
    }
  ))
}