use starlib::ui::menu::Menu;

pub(crate) fn get_pause_menu() -> Menu {
    let options = vec!["Continue", "Exit"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    Menu::build("Paused".to_string(), options)
}
