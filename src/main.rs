#![allow(non_snake_case)]

use dioxus::prelude::*;
use log::LevelFilter;

mod life;

fn main() {
  // Init debug
  dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
  console_error_panic_hook::set_once();

  log::info!("loading settings!");
  let settings = Settings::default();

  log::info!("starting app");
  // dioxus_web::launch_with_props(app, settings, dioxus_web::Config::new());
  dioxus_web::launch(life::run);
}

#[derive(PartialEq, Props)]
struct Settings {
  line_color: &'static str,
}

impl Default for Settings {
    fn default() -> Self {
        Self { line_color: "red" }
    }
}

fn app(cx: Scope<Settings>) -> Element {
  let color_str = use_state(cx, || cx.props.line_color);
  cx.render(rsx! (
    div {
      style: "text-align: center;",
      h1 { "🌗 Dioxus 🚀" }
      h3 { "Frontend that scales." }
      p { "Dioxus is a portable, performant, and ergonomic framework for building cross-platform user interfaces in Rust." }
      svg {
        width: 100,
        height: 100,
        circle {
          cx: 50,
          cy: 50,
          r: 20,
          stroke: color_str.get().to_owned(),
          fill: "black",
        }
      }
      button {
        onclick: |_event| {
          if color_str.get().to_owned() == cx.props.line_color {
            color_str.set("#00ff00");
          } else {
            color_str.set(cx.props.line_color);
          }
        },
        "Switch Color"
      }
    }
  ))
}
