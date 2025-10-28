const MAP_SIZE_X: usize = 5;
const MAP_SIZE_Y: usize = ((MAP_SIZE_X) as f64 /2.0).ceil() as usize;

fn main() {
    let mut earth = Planet {
        map: [
            ".....",
            ".....",
            ".....",
        ],
        player_coords: Coords {x: 2, y: 1},
        player_sprite: "ඞ",
    };
    earth.game_loop();
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
    Invalid
}

struct Planet {
    map: [&'static str; MAP_SIZE_Y],
    player_coords: Coords,
    player_sprite: &'static str,
}

impl Planet {
    fn print_map(&self) {
        clear_screen();
        let mut y: i32 = 0; 
        for line in self.map {
            let mut x: i32 = 0; 
            for character in line.chars() {
                if x==self.player_coords.x && y==self.player_coords.y  {
                    print!("{}", self.player_sprite);
                } else {
                    print!("{}", character);
                }
                x += 1;
            }
            println!("");
            y += 1;
        }
    }

    fn check_input(&self, input: &String) -> Movement {
        let input_data = &input[..];
        match input_data {
            "w"  => if self.is_valid_movement(&self.player_coords, Movement::Up) { Movement::Up } else { Movement::Invalid },
            "a"  => if self.is_valid_movement(&self.player_coords, Movement::Left) { Movement::Left } else { Movement::Invalid },
            "s"  => if self.is_valid_movement(&self.player_coords, Movement::Down) { Movement::Down } else { Movement::Invalid },
            "d"  => if self.is_valid_movement(&self.player_coords, Movement::Right) { Movement::Right } else { Movement::Invalid },
            _other => Movement::Invalid
        }
    }

    fn move_player(&mut self, movement: Movement) {
        match movement {
            Movement::Up => self.player_coords.y = self.player_coords.y - 1,
            Movement::Left => self.player_coords.x = self.player_coords.x - 1,
            Movement::Down => self.player_coords.y = self.player_coords.y + 1,
            Movement::Right => self.player_coords.x = self.player_coords.x + 1,
            Movement::Invalid => panic!("Wrong movement in move_player!"),
        }
    }

    fn is_valid_movement(&self, coords: &Coords, movement: Movement) -> bool {
        match movement {
            Movement::Up => coords.y - 1 >= 0, 
            Movement::Left => coords.x - 1 >= 0,
            Movement::Down => coords.y + 1 < MAP_SIZE_Y as i32,
            Movement::Right => coords.x + 1 < MAP_SIZE_X as i32, 
            Movement::Invalid => panic!("Invalid movement sent to is_valid_movement"), 
        }
    }

    fn game_loop(&mut self) {
        self.print_map();
        loop {
            let input = get_input();
            let movement = self.check_input(&input);
            if movement != Movement::Invalid { 
                self.move_player(movement);
                self.print_map();
            }
        }
    }
}

fn get_input() -> String {
    use std::{io, io::{Read, Write}};
    use termios::{Termios, TCSANOW, ECHO, ICANON, tcsetattr};
    let stdin = 0;
    let termios = Termios::from_fd(stdin).unwrap();
    let mut new_termios = termios.clone(); 
    new_termios.c_lflag &= !(ICANON | ECHO);
    tcsetattr(stdin, TCSANOW, &mut new_termios).unwrap();
    let stdout = io::stdout();
    let mut reader = io::stdin();
    let mut buffer = [0;1]; 
    stdout.lock().flush().unwrap();
    reader.read_exact(&mut buffer).unwrap();
    tcsetattr(stdin, TCSANOW, & termios).unwrap(); 
    let result = String::from_utf8(buffer.to_vec());
    match result {
        Ok(string) => string,
        Err(_error) => String::from(""),
    }
}

fn clear_screen() {
    clearscreen::clear().expect("Failed to clear screen");
}
