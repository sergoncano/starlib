use crate::{entities::Entity, planet::Planet, terminal};

pub fn render_frame(planet: &Planet, entities: &Vec<Box<dyn Entity>>) {
    terminal::clear_screen();
    print!("{}", planet.generate_banner());
    print!("{}", planet.generate_map(&entities));
}
pub fn render_tip(tip: String) {
    println!("{}", tip);
}
