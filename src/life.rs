use dioxus::prelude::*;
use std::{thread, env};
use std::time::Duration;

mod backend;
mod frontend;

pub fn run(cx: Scope) -> Element {
  // let settings = settings::Settings::default();
  // let life = Life::new(&settings);
  // let n_rows = 100;
  // let n_cols = 100;

  // let simulation = backend::Simulation::new(n_rows, n_cols);

  // let cells_ref = use_ref(cx, || simulation.population.cells);

  cx.render(rsx! {
    style {
      {"html, body {
        margin: 0;
        padding: 0;
        height: 100%;
        width: 100%;
      }"}
    },
    frontend::Ui{}
  })
}
