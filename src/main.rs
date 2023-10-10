use dioxus::prelude::*;

mod life;
mod util;
mod settings;

fn main() {
  dioxus_web::launch(life::run);
}
