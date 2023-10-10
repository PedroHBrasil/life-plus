use dioxus::prelude::*;
use crate::util::{Coord, Color};

pub struct Line {
  pub begin: Coord,
  pub end: Coord,
  pub thickness: u8,
  pub color: Color,
}

impl Line {
  pub fn render<'a>(&'a self, cx: Scope<'a>) -> Element {
    cx.render(rsx!(
      line {
        x1: self.begin.x as i64,
        y1: self.begin.y as i64,
        x2: self.end.x as i64,
        y2: self.end.y as i64,
      }
    ))
  }
}