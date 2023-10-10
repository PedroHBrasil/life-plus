use dioxus_fullstack::prelude::*;

mod life;
mod util;
mod settings;

fn main() {
    LaunchBuilder::new(life::Life::run).launch();
}
