pub mod collider;
pub mod coords;
pub mod decoration;
pub mod input;
pub mod movement;
pub mod planet;
pub mod planets;
pub mod renderable;
pub mod terminal;

use movement::Movement;
use planet::Planet;
use planets::earth;

fn main() {
    terminal::setup_terminal_properties();
    let mut earth = earth::generate();
    game_loop(&mut earth);
}

fn game_loop(
    planet: &mut Planet,
) {
    print!("{}", planet.generate_banner());
    print!("{}", planet.generate_map());
    loop {
        let movement_input = input::get_input();
        let movement = planet.check_movement_collision(planet.get_player_coords(), movement_input);
        if movement == Movement::Quit {
            terminal::restore_terminal_properties();
            std::process::exit(0);
        }
        if movement != Movement::Invalid {
            planet.move_player(movement);
            terminal::clear_screen();
            print!("{}", planet.generate_banner());
            print!("{}", planet.generate_map());
        }
    }
}
