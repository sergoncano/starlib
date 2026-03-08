use starlib::{
    renderer::{restore_terminal_properties, setup_terminal_properties},
    ui::{menu::Menu},
};

mod earth;
mod util;

fn main() {
    setup_terminal_properties();
    let earth_title = earth::get_title();
    let mut menu = get_menu();
    loop {
        match menu.prompt() {
            0 => {
                earth_title.show();
                match earth::get_level().run() {
                    0 => (),
                    1 => earth::get_victory_title().show(),
                    _ => unreachable!(),
                }
            }
            1 => break,
            _ => panic!("Nonexistant menu option chosen"),
        }
    }
    restore_terminal_properties();
}

fn get_menu() -> Menu {
    let options = vec!["Earth", "Exit"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    Menu::build("Choose destination".to_string(), options)
}
