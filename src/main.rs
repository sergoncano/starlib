pub mod collider;
pub mod coords;
pub mod decoration;
pub mod entities;
pub mod event;
pub mod input;
pub mod level;
pub mod map_util;
pub mod movement;
pub mod planet;
pub mod renderable;
pub mod renderer;
pub mod terminal;

use level::earth;

fn main() {
    terminal::setup_terminal_properties();
    let earth_level = earth::build();
    earth_level.game_loop();
}
