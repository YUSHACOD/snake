use crate::display::Size;
use crate::event_capturer::Input;
use crate::window::make_box_on_screen;
use crossterm::style;
use crossterm::{cursor, style::Stylize, QueueableCommand};
use std::io::Stdout;
use std::io::{self, Write};

// Just for testing
fn _get_in_string(input: &Input) -> String {
    match input {
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

// Game UI Printer
pub fn game_window_display(stdout: &mut Stdout, size: Size) -> io::Result<()> {
    let (x_start, x_end) = (size.x_axis.0, size.x_axis.1 - 1);
    let (y_start, y_end) = (size.y_axis.0, size.y_axis.1 - 1);

    make_box_on_screen(stdout, (x_start, x_end), (y_start, y_end))?;
    Ok(())
}

// Message printer
pub fn display_gen(stdout: &mut Stdout, size: &(u16, u16), gen: usize) -> io::Result<()> {
    stdout
        .queue(cursor::MoveTo(size.0, size.1))?
        .queue(style::PrintStyledContent(
            format!(" Generation : {:5} ", gen).grey(),
        ))?;
    stdout.flush()?;
    Ok(())
}
