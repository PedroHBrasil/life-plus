use std::{error::Error, path::Path, fs};
use serde::Deserialize;
use crate::util::Color;

#[derive(Deserialize)]
pub struct Settings {
  pub universe: UniverseSettings,
  pub lines: LineSettings,
  pub cells: CellSettings,
}

#[derive(Deserialize)]
pub struct UniverseSettings {
  pub width: usize,
  pub height: usize,
  pub speed: f32,
  pub color: Color,
}

#[derive(Deserialize)]
pub struct LineSettings {
  pub thickness: u8,
  pub color: Color,
}

#[derive(Deserialize)]
pub struct CellSettings {
  pub size: usize,
  pub color: Color,
}


impl Settings {
  pub fn load() -> Result<Self, Box<dyn Error>> {
    let settings_file_path = Path::new("./settings/settings.toml");
    let settings_str = fs::read_to_string(settings_file_path)?;
    let settings: Settings = toml::from_str(&settings_str)?;

    Ok(settings)
  }
}