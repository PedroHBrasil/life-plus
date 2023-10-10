use crate::settings;
use dioxus::html::canvas;
use dioxus::prelude::*;
use std::thread;
use std::time::Duration;

mod universe;

pub fn run(cx: Scope) -> Element {
  // let life = Life::new();

  cx.render(rsx! {
    div {
      "Hello World"
    }
  })
}

pub struct Life {
  universe: universe::Universe,
  speed: u64,
}

impl Life {
  pub fn new() -> Self {
    let settings = settings::Settings::load().unwrap();
    Self {
      universe: universe::Universe::new(&settings),
      speed: settings.universe.speed,
    }
  }
}
