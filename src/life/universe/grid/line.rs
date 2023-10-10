use crate::util::{Coord, Color};

pub struct Line {
  pub begin: Coord,
  pub end: Coord,
  pub thickness: u8,
  pub color: Color,
}
