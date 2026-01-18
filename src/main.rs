pub mod collider;
pub mod coords;
pub mod decoration;
pub mod entities;
pub mod event;
pub mod input;
pub mod levels;
pub mod map;
pub mod movement;
pub mod planet;
pub mod planets;
pub mod renderable;
pub mod terminal;

use levels::earth;

fn main() {
    terminal::setup_terminal_properties();
    earth::play();
}
