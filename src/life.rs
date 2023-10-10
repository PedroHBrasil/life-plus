use dioxus::prelude::*;

use crate::settings;

mod universe;

pub struct Life {
  universe: universe::Universe,
}

impl Life {
  pub fn new() -> Self {
    let settings = settings::Settings::load().unwrap();
    Self {
      universe: universe::Universe::new(settings)
    }
  }

  pub fn run(cx: Scope) -> Element {
    cx.render(rsx! {
      div {
        "Hello World"
      }
    })
  }
}
