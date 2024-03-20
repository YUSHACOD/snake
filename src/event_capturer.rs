use std::sync::mpsc::{SendError, Sender};

#[derive(PartialEq, Eq)]
pub enum Input {
    Start,
    Pause,
    Up,
    Down,
    Left,
    Right,
    Quit,
    Bs,
}

pub fn start(sdr: Sender<Input>) -> Result<(), SendError<Input>> {
    let mut input: Input;
    let c_term = console::Term::stdout();

    loop {
        // I love this match
        input = match c_term.read_key().expect("somethings wrong") {
            console::Key::Char('q') => Input::Quit,
            console::Key::Char('h') | console::Key::ArrowLeft => Input::Left,
            console::Key::Char('l') | console::Key::ArrowRight => Input::Right,
            console::Key::Char('j') | console::Key::ArrowDown => Input::Down,
            console::Key::Char('k') | console::Key::ArrowUp => Input::Up,
            console::Key::Char(' ') => Input::Pause,
            console::Key::Enter => Input::Start,
            _ => Input::Bs,
        };

        if let Input::Quit = input {
            sdr.send(input)?;
            break;
        }
        sdr.send(input)?;
    }
    Ok(())
}
