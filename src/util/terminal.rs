pub fn setup_terminal_properties() {
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
    tcsetattr(stdin, TCSANOW, &termios).unwrap();
}

pub fn restore_terminal_properties() {
    use crossterm::{execute, terminal::LeaveAlternateScreen};
    let mut stdout = std::io::stdout();
    let _ = execute!(stdout, LeaveAlternateScreen);
}

pub fn clear_screen() {
    use crossterm::{
        cursor::MoveTo,
        execute,
        terminal::{Clear, ClearType::All},
    };
    let mut stdout = std::io::stdout();
    let _ = execute!(stdout, Clear(All));
    let _ = execute!(stdout, MoveTo(0, 0));
}
