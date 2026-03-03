use std::{collections::HashMap, thread::sleep, time::Duration};

use crate::{Sprite, graphics::map::Map, model::{coords::Coords, tip::Tip}};

use crossterm::{cursor::MoveTo, execute, terminal::{Clear, enable_raw_mode}};

pub(crate) fn render_game(
    map: &Map,
    decorations: &HashMap<Coords, Sprite>,
    entity_sprites: &Vec<(Coords, Sprite)>,
    tip: Option<Tip>
) {
    //this is not optimal efficiency but I think the shorter code is worth it
    let mut decorations = decorations.clone();
    for (coords, sprite) in entity_sprites {
        decorations
            .entry(coords.clone())
            .and_modify(|s| *s = s.clone().overlap(sprite.clone()))
            .or_insert(sprite.clone());
    }

    let mut res = vec![];
    for (y, line) in map.lines.iter().enumerate() {
        let mut res_line = String::from("");
        for (x, char) in line.chars().enumerate() {
            if let Some(alt_char) = decorations.get(&Coords::new(x as i32, y as i32)) && alt_char.get_z_index() > 0 {
                    res_line.push(alt_char.get_character());
            } else {
                res_line.push(char);
            }
        }
        res.push(res_line);
    }
    centered_render(res, tip);
}

fn centered_render(text: Vec<String>, tip: Option<Tip>) {
    let mut width;
    let mut height;
    loop {
        clear_screen();
        let min_height = text.len();
        let (w, h) = crossterm::terminal::size().unwrap_or((0, 0));
        width = w as usize;
        height = h as usize;
        if height < min_height {
            println!("Enlarge your terminal!");
            sleep(Duration::from_secs(1));
        } else {
            break;
        }
    }
    for _ in 0..(height - text.len().div_ceil(2)) {
        println!();
    }
    for line in text {
        println!("{: ^width$}", line);
    }
    if let Some(tip) = tip {
        println!("{: ^width$}", tip.get_text());
    }
}

fn clear_screen() {
    let mut stdout = std::io::stdout();
    let _ = execute!(stdout, Clear(crossterm::terminal::ClearType::All));
    let _ = execute!(stdout, MoveTo(0, 0));
}

pub fn setup_terminal_properties() {
    use crossterm::{cursor, execute, terminal::EnterAlternateScreen};
    let mut stdout = std::io::stdout();
    let _ = execute!(stdout, EnterAlternateScreen);
    let _ = execute!(stdout, cursor::Hide);
    let _ = execute!(stdout, cursor::MoveTo(0, 0));

    //Make input keys invisible
    let _ = execute!(stdout, cursor::Hide);
    let _ = execute!(stdout, EnterAlternateScreen);
    let _ = enable_raw_mode();
}

pub fn restore_terminal_properties() {
    use crossterm::{execute, terminal::LeaveAlternateScreen};
    let mut stdout = std::io::stdout();
    let _ = execute!(stdout, LeaveAlternateScreen);
}
