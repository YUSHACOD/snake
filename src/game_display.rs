use crate::display::Size;
use crate::event_capturer::Input;
use crate::window::make_box_on_screen;
use crossterm::style;
use crossterm::{cursor, style::Stylize, QueueableCommand};
use std::io::Stdout;
use std::io::{self, Write};

// Just for testing
fn get_in_string(input: &Input) -> String {
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

// Score Box
fn display_score_box(stdout: &mut Stdout, size: &Size) -> io::Result<()> {
    let (x_start, x_end) = (size.x_axis.1 - 17, size.x_axis.1);
    let (y_start, y_end) = (size.y_axis.1 - 2, size.y_axis.1);
    make_box_on_screen(stdout, (x_start, x_end), (y_start, y_end))
}

// Message Box
fn display_message_box(stdout: &mut Stdout, size: &Size) -> io::Result<()> {
    let (x_start, x_end) = (size.x_axis.0, size.x_axis.1 - 18);
    let (y_start, y_end) = (size.y_axis.1 - 2, size.y_axis.1);

    make_box_on_screen(stdout, (x_start, x_end), (y_start, y_end))
}

// Game UI Printer
pub fn game_display(stdout: &mut Stdout, size: Size) -> io::Result<()> {
    let (x_start, x_end) = (size.x_axis.0, size.x_axis.1);
    let (y_start, y_end) = (size.y_axis.0, size.y_axis.1 - 3);

    make_box_on_screen(stdout, (x_start, x_end), (y_start, y_end))?;
    display_score_box(stdout, &size)?;
    display_message_box(stdout, &size)?;
    Ok(())
}

// Message printer
pub fn print_message(stdout: &mut Stdout, size: &(u16, u16), input: &Input) -> io::Result<()> {
    stdout
        .queue(cursor::MoveTo(size.0, size.1))?
        .queue(style::PrintStyledContent(
            format!(" Message : {:10} ", get_in_string(input)).grey(),
        ))?;
    stdout.flush()?;
    Ok(())
}

// Score printer
pub fn print_score(stdout: &mut Stdout, size: &(u16, u16), score: usize) -> io::Result<()> {
    stdout
        .queue(cursor::MoveTo(size.0, size.1))?
        .queue(style::PrintStyledContent(
            format!(" Score : {:5} ", score).grey(),
        ))?;
    stdout.flush()?;
    Ok(())
}
