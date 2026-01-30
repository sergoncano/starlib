mod entities;
mod stage;

use std::time::Duration;

use crate::stage::level::earth;
use starlib::{ui::{menu::Menu, title::Title}, util::terminal};

fn main() {
    terminal::setup_terminal_properties();
    let header_text = "SELECT DESTINATION".to_string();
    let options = vec!["Earth".to_string(), "Exit".to_string()];
    let menu = Menu::new(header_text, options);
    loop {
        match &menu.show() {
            0 => {
                let earth_title = Title::new("EARTH".to_string(), Option::from("catch the rabbit".to_string()), Duration::new(2,0));
                earth_title.show();
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
