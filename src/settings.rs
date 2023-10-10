use std::{error::Error, path::Path, fs};
use crate::util::Color;

#[derive(Default)]
pub struct Settings {
  pub universe: UniverseSettings,
  pub lines: LineSettings,
  pub cells: CellSettings,
}

pub struct UniverseSettings {
  pub width: usize,
  pub height: usize,
  pub speed: u64,
  pub color: Color,
}

impl Default for UniverseSettings {
  fn default() -> Self {
    Self {
      width: 800,
      height: 450,
      speed: 1,
      color: Color {
        red: 0x22,
        green: 0x22,
        blue: 0x22,
        alpha: 0xff,
      }
    }
  }
}

pub struct LineSettings {
  pub thickness: u8,
  pub color: Color,
}

impl Default for LineSettings {
  fn default() -> Self {
    Self {
      thickness: 1,
      color: Color {
        red: 0x44,
        green: 0x44,
        blue: 0x44,
        alpha: 0xff,
      }
    }
  }
}

pub struct CellSettings {
  pub size: usize,
  pub color: Color,
}

impl Default for CellSettings {
  fn default() -> Self {
    Self {
      size: 10,
      color: Color {
        red: 0x66,
        green: 0x44,
        blue: 0xff,
        alpha: 0xff,
      }
    }
  }
}
