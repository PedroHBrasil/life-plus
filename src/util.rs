use serde::Deserialize;

#[derive(Default, Debug, Copy, Clone)]
pub struct Coord {
  pub x: usize,
  pub y: usize,
}

#[derive(Default, Debug, Copy, Clone, Deserialize)]
pub struct Color {
  pub red: u8,
  pub green: u8,
  pub blue: u8,
  pub alpha: u8
}
