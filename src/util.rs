use serde::Deserialize;

#[derive(Default, Debug, Copy, Clone)]
pub struct Coord {
  pub x: usize,
  pub y: usize,
}

impl Coord {
  pub fn as_tuple(&self) -> (f32, f32) {
    (self.x as f32, self.y as f32)
  }
}

impl Coord {
  pub fn as_speedy2d_vec2(&self) -> speedy2d::dimen::Vec2 {
    speedy2d::dimen::Vec2 { x: self.x as f32, y: self.y as f32 }
  }
}

#[derive(Default, Debug, Copy, Clone, Deserialize)]
pub struct Color {
  pub red: u8,
  pub green: u8,
  pub blue: u8,
  pub alpha: u8
}

impl Color {
  pub fn as_speedy2d_color(&self) -> speedy2d::color::Color {
    speedy2d::color::Color::from_rgba(
      self.red as f32 / 255.0,
      self.green as f32 / 255.0,
      self.blue as f32 / 255.0,
      self.alpha as f32 / 255.0
    )
  }
}

#[allow(unused_variables)]
pub trait Drawable {
  fn draw(&self, graphics: &mut speedy2d::Graphics2D) { }
}