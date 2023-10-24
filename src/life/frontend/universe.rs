use dioxus::prelude::*;
use super::util;
mod population;
mod grid;

pub fn UniverseUi(cx: Scope) -> Element {
  let (window_width, window_height) = util::get_window_size();
  cx.render(rsx!(
    // population::PopulationUi{},
    grid::GridUi {
      width: window_width,
      height: window_height,
    }
  ))
}

