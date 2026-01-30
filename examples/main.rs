mod entities;
mod stage;

use crate::stage::level::earth;
use starlib::{ui::menu::Menu, util::terminal};

fn main() {
    terminal::setup_terminal_properties();
    let header_text = "SELECT DESTINATION".to_string();
    let options = vec!["Earth".to_string(), "Exit".to_string()];
    let menu = Menu::new(header_text, options);
    loop {
        match &menu.show() {
            0 => {
                let earth_level = earth::build();
                earth_level.game_loop();
            }
            1 => {
                break;
            }
            _ => {
                panic!("Inexistant menu option selected!");
            }
        }
    }
    terminal::restore_terminal_properties();
}
