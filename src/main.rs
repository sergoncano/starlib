pub mod coords;
pub mod input;
pub mod movement;
pub mod planet;
pub mod terminal;

use coords::Coords;
use movement::Movement;
use planet::Planet;

fn main() {
    terminal::setup_terminal_properties();
    let mut earth: Planet<10, 5> = Planet::new(
        "Earth",
        [
            "..........",
            "..........",
            "..........",
            "..........",
            "..........",
        ],
        Coords::new(4, 2),
        "ඞ",
    );
    game_loop(&mut earth);
}

fn game_loop<const MAP_SIZE_X: usize, const MAP_SIZE_Y: usize>(
    planet: &mut Planet<MAP_SIZE_X, MAP_SIZE_Y>,
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
