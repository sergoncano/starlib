pub mod entities;
pub mod interfaces;
pub mod level;
pub mod model;
pub mod planet;
pub mod util;

use crate::{level::earth, util::terminal};

fn main() {
    terminal::setup_terminal_properties();
    let earth_level = earth::build();
    earth_level.game_loop();
}
