use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
use std::sync::mpsc::{SendError, Sender};

const DEFAULT_EVENT: Event = Event::Key(KeyEvent {
    code: KeyCode::Null,
    modifiers: KeyModifiers::NONE,
    kind: KeyEventKind::Press,
    state: KeyEventState::NONE,
});

#[derive(PartialEq, Eq, Clone)]
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

impl Input {
    fn from(key: KeyCode) -> Input {
        match key {
            KeyCode::Char('q') => Input::Quit,
            KeyCode::Char('h') | KeyCode::Left => Input::Left,
            KeyCode::Char('l') | KeyCode::Right => Input::Right,
            KeyCode::Char('j') | KeyCode::Down => Input::Down,
            KeyCode::Char('k') | KeyCode::Up => Input::Up,
            KeyCode::Char(' ') => Input::Pause,
            KeyCode::Enter => Input::Start,
            _ => Input::Bs,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Input::Quit => "Quit".to_string(),
            Input::Up => "Up".to_string(),
            Input::Down => "Down".to_string(),
            Input::Left => "Left".to_string(),
            Input::Right => "Right".to_string(),
            Input::Start => "Start".to_string(),
            Input::Pause => "Pause".to_string(),
            Input::Bs => "Bs".to_string(),
        }
    }
}

pub fn start(sdr: &Sender<Input>) -> Result<(), SendError<Input>> {
    let mut prev = Input::Bs;
    let mut input: Input;

    loop {
<<<<<<< Updated upstream
        input = match read().unwrap_or(DEFAULT_EVENT) {
            Event::Key(key_event) => match key_event {
                KeyEvent {
                    code: key,
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    state: KeyEventState::NONE,
                } => Input::from(key),
                KeyEvent {
                    code: _,
                    modifiers: KeyModifiers::NONE,
                    kind: _,
                    state: KeyEventState::NONE,
                } => {
                    sdr.send(Input::Quit)?;
                    dbg!(key_event);
                    break;
                }
                _ => Input::Bs,
            },
=======
        input = match c_term
            .read_key()
            .inspect_err(|_| window::clean_up(&mut stdout))
            .expect("somethings wrong")
        {
            console::Key::Char('q') => Input::Quit,
            console::Key::Char('h') | console::Key::ArrowLeft => Input::Left,
            console::Key::Char('l') | console::Key::ArrowRight => Input::Right,
            console::Key::Char('j') | console::Key::ArrowDown => Input::Down,
            console::Key::Char('k') | console::Key::ArrowUp => Input::Up,
            console::Key::Char(' ') => Input::Pause,
            console::Key::Enter => Input::Start,
>>>>>>> Stashed changes
            _ => Input::Bs,
        };

        match input.clone() {
            Input::Quit => {
                sdr.send(input)?;
                break;
            }
            Input::Bs => {}
            x if x == prev => {}
            _ => sdr.send(input.clone())?,
        };

        prev = input;
    }
    Ok(())
}
