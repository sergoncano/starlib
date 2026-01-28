use crate::{interfaces::entity::Entity, planet::Planet, util::terminal};

pub fn render_frame(planet: &Planet, entities: &Vec<Box<dyn Entity>>) {
    terminal::clear_screen();
    let (width_u16, height_u16) = crossterm::terminal::size().unwrap_or((0,0));
    let width = width_u16 as usize;
    let mut content = format!("{: ^width$}\n", planet.generate_banner().trim());
    let mut content_height = 0;
    for line in planet.generate_map(entities).trim().split('\n') {
        content.push_str(&format!("{: ^width$}", line)[..]);
        content_height += 1;
    }
    for _ in 0..((height_u16 - content_height)/2)-1 {
        print!("\n");
    }
    print!("{}", content);
}
pub fn render_tip(tip: String) {
    let (width_u16, _height_u16) = crossterm::terminal::size().unwrap_or((0,0));
    let width = width_u16 as usize;
    println!("{: ^width$}", tip);
}
