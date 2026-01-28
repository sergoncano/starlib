use crate::model::movement::Movement;

pub fn get_input() -> Movement {
    use std::time::Duration;
    use std::{
        io,
        io::{Read, Write},
    };
    use timeout_readwrite::TimeoutReader;
    let stdout = io::stdout();
    let reader = io::stdin();
    let mut timeout_reader = TimeoutReader::new(reader, Duration::new(0, 500000000));
    let mut buffer = [0; 1];
    stdout.lock().flush().unwrap();
    let result = timeout_reader.read_exact(&mut buffer);
    let string = match result {
        Ok(_) => {
            let key = String::from_utf8(buffer.to_vec());
            key.unwrap_or_default()
        }
        Err(_) => String::from("timeout"),
    };
    parse_input_text(string)
}

fn parse_input_text(input: String) -> Movement {
    let input_data = &input.to_lowercase()[..];
    match input_data {
        "w" => Movement::Up,
        "a" => Movement::Left,
        "s" => Movement::Down,
        "d" => Movement::Right,
        "e" => Movement::Interact,
        "q" => Movement::Quit,
        "timeout" => Movement::Wait,
        _other => Movement::Invalid,
    }
}
