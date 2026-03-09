use starlib::{
    renderer::{restore_terminal_properties, setup_terminal_properties},
    ui::{menu::Menu},
};

mod arrakis;
mod earth;
mod player;
mod user_event;
mod util;

fn main() {
    setup_terminal_properties();
    let earth_title = earth::get_title();
    let arrakis_title = arrakis::get_title();
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
            1 => {
                arrakis_title.show();
                match arrakis::get_level().run() {
                    0 => (),
                    1 => arrakis::get_death_title().show(),
                    2 => arrakis::get_victory_title().show(),
                    _ => unreachable!(),
                }
            }
            2 => break,
            _ => unreachable!()
        }
    }
    restore_terminal_properties();
}

fn get_menu() -> Menu {
    let options = vec!["Earth", "Arrakis", "Exit"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    Menu::build("Choose destination".to_string(), options)
}
