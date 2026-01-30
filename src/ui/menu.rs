use crate::{
    model::movement::Movement,
    util::{input::get_menu_input, renderer::render_menu},
};

pub struct Menu {
    header_text: String,
    options: Vec<String>,
}
impl Menu {
    pub fn new(header_text: String, options: Vec<String>) -> Menu {
        Menu {
            header_text,
            options,
        }
    }

    pub fn show(&self) -> usize {
        let n_options = &self.options.len();
        let mut selected = 0;
        loop {
            render_menu(&self.header_text, &self.options, selected);
            //sleep(Duration::new(3,0));
            match get_menu_input() {
                Movement::Up => selected = selected.checked_sub(1).unwrap_or(n_options - 1),
                Movement::Down => selected = (selected + 1).rem_euclid(*n_options),
                Movement::Interact => break,
                _ => (),
            }
        }
        selected
    }
}
