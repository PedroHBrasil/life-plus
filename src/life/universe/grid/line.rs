use crate::util::{Coord, Color, Drawable};

pub struct Line {
  pub begin: Coord,
  pub end: Coord,
  pub thickness: u8,
  pub color: Color,
}

impl Drawable for Line {
  fn draw(&self, graphics: &mut speedy2d::Graphics2D) {
    graphics.draw_line(
      self.begin.as_speedy2d_vec2(),
      self.end.as_speedy2d_vec2(),
      self.thickness as f32,
      self.color.as_speedy2d_color()
    );
  }
}