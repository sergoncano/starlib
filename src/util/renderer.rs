use std::cmp;

use crate::{interface::entity::Entity, stage::planet::Planet, util::terminal};

pub fn render_frame(planet: &Planet, entities: &Vec<Box<dyn Entity>>) {
    let mut content = vec![planet.generate_banner().trim().to_string()];
    content.extend(planet.generate_map(entities).trim().split('\n').map(|s| s.to_string()));
    centered_render(content, false);
}
pub fn render_tip(tip: String) {
    let (width_u16, _height_u16) = crossterm::terminal::size().unwrap_or((0, 0));
    let width = width_u16 as usize;
    println!("{: ^width$}", tip);
}

pub fn render_menu(header_text: &String, options: &Vec<String>, selected: usize) {
    let mut max_length: usize = 0;
    for option in options {
        max_length = cmp::max(max_length, option.len());
    }
    let formatted_options: Vec<String> = options.iter().enumerate().map(|(i, s)| format!("{}{s: <max_length$}", if i==selected { "·" } else { " " })).collect();
    let mut text: Vec<String> = vec![header_text.clone()];
    text.extend(formatted_options);
    centered_render(text, true);
}

pub fn render_title(title: String, subtitle: Option<String>) {
    let mut content = vec![title];
    if subtitle.is_some() {
        content.push(subtitle.unwrap());
    }
    centered_render(content, true);
}

fn centered_render(text: Vec<String>, separate: bool) {
    terminal::clear_screen();
    let (width_u16, height_u16) = crossterm::terminal::size().unwrap_or((0, 0));
    let width = width_u16 as usize;
    let height = height_u16 as i32;
    for _ in 0..((height - ((text.len() as i32 + 1) * if separate { 2 } else { 1 })) / 2) {
        println!();
    }
    for line in text {
        print!("{: ^width$}\n", line);
        if separate { println!(); }
    }
}
