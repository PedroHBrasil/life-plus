use speedy2d::Window;

mod life;
mod util;
mod settings;

fn main() {
  let window = Window::new_centered("Title", (1000, 1000)).unwrap();
  window.run_loop(life::Life::new());
}
