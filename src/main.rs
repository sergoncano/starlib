fn main() {
    setup_terminal_properties();
    let mut earth = Planet::<10, 5> {
        name: "Earth",
        map: [
            "..........",
            "..........",
            "..........",
            "..........",
            "..........",
        ],
        player_coords: Coords { x: 4, y: 2 },
        player_sprite: "ඞ",
    };
    game_loop(&mut earth);
}

fn setup_terminal_properties() {
    use crossterm::{cursor, execute, terminal::EnterAlternateScreen};
    let mut stdout = std::io::stdout();
    let _ = execute!(stdout, EnterAlternateScreen);
    let _ = execute!(stdout, cursor::Hide);
    let _ = execute!(stdout, cursor::MoveTo(0, 0));

    //Make input keys invisible
    use termios::{ECHO, ICANON, TCSANOW, Termios, tcsetattr};
    let stdin = 0;
    let mut termios = Termios::from_fd(stdin).unwrap();
    termios.c_lflag &= !(ICANON | ECHO);
    tcsetattr(stdin, TCSANOW, &mut termios).unwrap();
}

fn restore_terminal_properties() {
    use crossterm::{execute, terminal::LeaveAlternateScreen};
    let mut stdout = std::io::stdout();
    let _ = execute!(stdout, LeaveAlternateScreen);
}

fn game_loop<const MAP_SIZE_X: usize, const MAP_SIZE_Y: usize>(
    planet: &mut Planet<MAP_SIZE_X, MAP_SIZE_Y>,
) {
    print!("{}", planet.generate_banner());
    print!("{}", planet.generate_map());
    loop {
        let movement_input = get_input();
        let movement = planet.check_movement_collision(planet.get_player_coords(), movement_input);
        if movement == Movement::Quit {
            restore_terminal_properties();
            std::process::exit(0);
        }
        if movement != Movement::Invalid {
            planet.move_player(movement);
            clear_screen();
            print!("{}", planet.generate_banner());
            print!("{}", planet.generate_map());
        }
    }
}

struct Coords {
    x: i32,
    y: i32,
}

#[derive(PartialEq)]
enum Movement {
    Up,
    Down,
    Left,
    Right,
    Quit,
    Wait,
    Invalid,
}

struct Planet<const MAP_SIZE_X: usize, const MAP_SIZE_Y: usize> {
    name: &'static str,
    map: [&'static str; MAP_SIZE_Y],
    player_coords: Coords,
    player_sprite: &'static str,
}

impl<const MAP_SIZE_X: usize, const MAP_SIZE_Y: usize> Planet<MAP_SIZE_X, MAP_SIZE_Y> {
    fn generate_map(&self) -> String {
        let mut map_str = String::from("");
        let mut y: i32 = 0;
        for line in self.map {
            let mut x: i32 = 0;
            for character in line.chars() {
                if x == self.player_coords.x && y == self.player_coords.y {
                    map_str.push_str(self.player_sprite);
                } else {
                    map_str.push(character);
                }
                x += 1;
            }
            map_str.push_str("\n");
            y += 1;
        }
        return map_str;
    }

    fn generate_banner(&self) -> String {
        let mut banner = String::from("");
        if self.name.len() > MAP_SIZE_X {
            return String::from("");
        }
        let dashes: f64 = (MAP_SIZE_X - self.name.len()) as f64 / 2.0;
        for _i in 0..(dashes.floor() as i32) {
            banner.push_str("-");
        }
        banner.push_str(self.name);
        for _i in 0..(dashes.ceil() as i32) {
            banner.push_str("-");
        }
        banner.push_str("\n");
        return banner;
    }

    fn move_player(&mut self, movement: Movement) {
        match movement {
            Movement::Up => self.player_coords.y = self.player_coords.y - 1,
            Movement::Left => self.player_coords.x = self.player_coords.x - 1,
            Movement::Down => self.player_coords.y = self.player_coords.y + 1,
            Movement::Right => self.player_coords.x = self.player_coords.x + 1,
            Movement::Wait => (),
            _other => panic!("Unsupported struct Movement passed to move_player!"),
        }
    }

    //Takes coordinates and a movement and returns the same movement if it is valid. If invalid
    //returns Movement::Invalid.
    fn check_movement_collision(&self, coords: &Coords, movement: Movement) -> Movement {
        match movement {
            Movement::Up => { if coords.y - 1 >= 0 { Movement::Up } else { Movement::Invalid } },
            Movement::Left => { if coords.x - 1 >= 0 { Movement::Left } else { Movement::Invalid } },
            Movement::Down => { if coords.y + 1 < MAP_SIZE_Y as i32 { Movement::Down } else { Movement::Invalid } },
            Movement::Right => { if coords.x + 1 < MAP_SIZE_X as i32 { Movement::Right } else { Movement::Invalid } },
            Movement::Wait => Movement::Wait,
            Movement::Invalid => Movement::Invalid,
            Movement::Quit => Movement::Quit,
        }
    }

    fn get_player_coords(&self) -> &Coords {
        &self.player_coords
    }
}

fn get_input() -> Movement {
    use std::{
        io,
        io::{Read, Write},
    };
    use timeout_readwrite::TimeoutReader;
    use std::time::Duration;
    let stdout = io::stdout();
    let reader = io::stdin();
    let mut timeout_reader = TimeoutReader::new(reader, Duration::new(3,0));
    let mut buffer = [0; 1];
    stdout.lock().flush().unwrap();
    let result = timeout_reader.read_exact(&mut buffer);
    let string = match result {
        Ok(_) => {
            let key = String::from_utf8(buffer.to_vec());
            match key {
                Ok(string) => string,
                Err(_error) => String::from(""),
            }
        },
        Err(_) => String::from("timeout")
    };
    parse_input_text(string)
}

fn parse_input_text(input: String) -> Movement {
    let input_data = &input[..];
    match input_data {
        "w" => Movement::Up,
        "a" => Movement::Left,
        "s" => Movement::Down,
        "d" => Movement::Right,
        "q" => Movement::Quit,
        "timeout" => Movement::Wait,
        _other => Movement::Invalid,
    }
}

fn clear_screen() {
    use crossterm::{
        cursor::MoveTo,
        execute,
        terminal::{Clear, ClearType::All},
    };
    let mut stdout = std::io::stdout();
    let _ = execute!(stdout, Clear(All));
    let _ = execute!(stdout, MoveTo(0, 0));
}
