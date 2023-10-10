use speedy2d::color::Color;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::Graphics2D;
use crate::settings;
use crate::util::Drawable;
use std::thread;
use std::time::Duration;

mod universe;

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

impl WindowHandler for Life {
  fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D)
    {
        self.universe.draw(graphics);

        thread::sleep(Duration::from_millis(1000 / self.speed));
        helper.request_redraw();

        self.universe.update_population();
    }
}