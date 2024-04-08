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
}

pub fn start(sdr: Sender<Input>) -> Result<(), SendError<Input>> {
    let mut prev = Input::Bs;
    let mut input: Input;

    loop {
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
            _ => Input::Bs,
        };

        if let Input::Quit = input {
            sdr.send(input)?;
            break;
        }

        if let Input::Bs = input {
        } else if input != prev {
            sdr.send(input.clone())?;
        }

        prev = input;
    }
    Ok(())
}
