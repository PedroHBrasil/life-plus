use crate::settings;
use dioxus::html::canvas;
use dioxus::prelude::*;
use std::{thread, env};
use std::time::Duration;

mod universe;

pub fn run(cx: Scope) -> Element {

  let mut life = use_state(cx, || Life::new());
  let settings = settings::Settings::default();

  cx.render(rsx! {
    life.universe.render(cx, &settings)
  })
}

pub struct Life {
  universe: universe::Universe,
  speed: u64,
}

impl Life {
  pub fn new() -> Self {
    let settings = settings::Settings::default();
    Self {
      universe: universe::Universe::new(&settings),
      speed: settings.universe.speed,
    }
  }
}
