//! Contains a few utility functions related to rendering.
//! [setup_terminal_properties] is a must for (input)[crate::util::input]'s methods to work.
use std::{collections::HashMap, thread::sleep, time::Duration};

use crate::{
    Sprite,
    graphics::map::Map,
    model::{coords::Coords, tip::Tip},
};

use crossterm::{
    cursor::{MoveTo, MoveToNextLine},
    execute,
    terminal::{Clear, enable_raw_mode},
};

pub(crate) fn render_game(
    map: &Map,
    decorations: &HashMap<Coords, Sprite>,
    entity_sprites: &Vec<(Coords, Sprite)>,
    tip: Option<Tip>,
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
            if let Some(alt_char) = decorations.get(&Coords::new(x as i32, y as i32))
                && alt_char.get_z_index() > 0
            {
                res_line.push(alt_char.get_character());
            } else {
                res_line.push(char);
            }
        }
        res.push(res_line);
    }
    centered_render(&res, tip);
}

pub(crate) fn centered_render(text: &Vec<String>, tip: Option<Tip>) {
    let mut stdout = std::io::stdout();
    let mut width;
    let mut height;
    loop {
        clear_screen();
        let min_height = text.len();
        let min_width = text.iter().fold(0, |a, b| if a>b.len() {a} else {b.len()});
        let (w, h) = crossterm::terminal::size().unwrap_or((0, 0));
        width = w as usize;
        height = h as usize;
        if height < min_height || width < min_width {
            println!("Enlarge your terminal!");
            sleep(Duration::from_secs(1));
        } else {
            break;
        }
    }
    for _ in 0..((height - text.len()).div_ceil(2)) {
        let _ = execute!(stdout, MoveToNextLine(1));
    }
    for line in text {
        print!("{: ^width$}", line);
        let _ = execute!(stdout, MoveToNextLine(1));
    }
    if let Some(tip) = tip {
        print!("{: ^width$}", tip.get_text());
        let _ = execute!(stdout, MoveToNextLine(1));
    }
}

/// Clears the screen using crossterm's command and moves the cursor to (0,0)
pub fn clear_screen() {
    let mut stdout = std::io::stdout();
    let _ = execute!(stdout, Clear(crossterm::terminal::ClearType::All));
    let _ = execute!(stdout, MoveTo(0, 0));
}

/// Enters alternate terminal, hides the cursor and enables raw mode.
/// Note that basically nothing in [input](crate::util::input) will work
/// if these properties aren't setup. Raw mode sends input byte-by-byte instead of
/// in buffers, which is crucial for both menus and levels.
pub fn setup_terminal_properties() {
    use crossterm::{cursor, execute, terminal::EnterAlternateScreen};
    let mut stdout = std::io::stdout();
    let _ = execute!(stdout, EnterAlternateScreen);
    let _ = execute!(stdout, cursor::Hide);
    let _ = execute!(stdout, cursor::MoveTo(0, 0));
    let _ = enable_raw_mode();
}

/// Undoes all changes [setup_terminal_properties] does.
pub fn restore_terminal_properties() {
    use crossterm::{execute, terminal::LeaveAlternateScreen};
    let mut stdout = std::io::stdout();
    let _ = execute!(stdout, LeaveAlternateScreen);
}
