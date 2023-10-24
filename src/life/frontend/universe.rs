use dioxus::prelude::*;
use super::{util, settings};
mod population;

pub fn UniverseUi(cx: Scope) -> Element {
  let settings = use_shared_state::<settings::Settings>(cx).unwrap().read();
  let universe_color = settings.universe.color;
  let style_tag = format!("body {{ background-color: {}}}", universe_color);

  let (window_width, window_height) = util::get_window_size();
  cx.render(rsx!(
    style { style_tag }
    population::PopulationUi {
      width: window_width,
      height: window_height,
    }
  ))
}

