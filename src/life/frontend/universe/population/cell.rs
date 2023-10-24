use crate::life::frontend::util::Coord;

#[derive(Default, Debug, Copy, Clone)]
pub struct CellProps {
  pub lives: bool,
  pub origin: Coord,
  pub size: usize,
}
