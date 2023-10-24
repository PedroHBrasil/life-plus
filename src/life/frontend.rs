use dioxus::prelude::*;
mod universe;
mod util;
mod settings;

pub fn Ui(cx: Scope) -> Element {
  use_shared_state_provider(cx, || settings::Settings::default());
  cx.render(rsx!(
    universe::UniverseUi{}
  ))
}
