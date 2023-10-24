use std::fmt::Display;
use serde::Deserialize;

pub fn get_window_size() -> (usize, usize) {
  let window = web_sys::window().unwrap();

  let width = window.inner_width().unwrap().as_f64().unwrap() as usize;
  let height = window.inner_height().unwrap().as_f64().unwrap() as usize;

  (width, height)
}

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct Coord {
  pub x: usize,
  pub y: usize,
}

#[derive(Default, Debug, Copy, Clone, Deserialize, PartialEq)]
pub struct Color {
  pub red: u8,
  pub green: u8,
  pub blue: u8,
  pub alpha: u8,
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}{:02x}", self.red, self.green, self.blue, self.alpha)
    }
}