use dioxus::prelude::Props;

#[derive(Default, Debug, Clone, PartialEq, Props)]
pub struct Settings {
  pub universe: UniverseSettings,
  pub lines: LineSettings,
  pub cells: CellSettings,
}

#[derive(Debug, Clone, PartialEq, Props)]
pub struct UniverseSettings {
  pub width: usize,
  pub height: usize,
  pub color: &'static str,
}

impl Default for UniverseSettings {
  fn default() -> Self {
    Self {
      width: 800,
      height: 450,
      color: "#222222ff",
    }
  }
}

#[derive(Debug, Clone, PartialEq, Props)]
pub struct LineSettings {
  pub thickness: u8,
  pub color: &'static str,
}

impl Default for LineSettings {
  fn default() -> Self {
    Self {
      thickness: 1,
      color: "#444444ff",
    }
  }
}

#[derive(Debug, Clone, PartialEq, Props)]
pub struct CellSettings {
  pub size: usize,
  pub color: &'static str,
}

impl Default for CellSettings {
  fn default() -> Self {
    Self {
      size: 10,
      color: "#6644ffff",
    }
  }
}
