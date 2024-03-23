use crossterm::{
    style::Stylize,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
    *,
};
use std::io::{self, Stdout, Write};

use crate::display;

pub fn make_box_on_screen(
    stdout: &mut Stdout,
    (x_start, x_end): (u16, u16),
    (y_start, y_end): (u16, u16),
) -> io::Result<()> {
    for x in x_start..x_end {
        stdout
            .queue(cursor::MoveTo(x, y_start))?
            .queue(style::PrintStyledContent("─".grey()))?
            .queue(cursor::MoveTo(x, y_end))?
            .queue(style::PrintStyledContent("─".grey()))?;
    }

    for y in y_start..y_end {
        stdout
            .queue(cursor::MoveTo(x_start, y))?
            .queue(style::PrintStyledContent("│".grey()))?
            .queue(cursor::MoveTo(x_end, y))?
            .queue(style::PrintStyledContent("│".grey()))?;
    }

    stdout
        .queue(cursor::MoveTo(x_start, y_start))?
        .queue(style::PrintStyledContent("┌".grey()))?
        .queue(cursor::MoveTo(x_end, y_start))?
        .queue(style::PrintStyledContent("┐".grey()))?
        .queue(cursor::MoveTo(x_start, y_end))?
        .queue(style::PrintStyledContent("└".grey()))?
        .queue(cursor::MoveTo(x_end, y_end))?
        .queue(style::PrintStyledContent("┘".grey()))?;

    stdout.flush()?;
    Ok(())
}

fn setup_borders(title: String, stdout: &mut Stdout, (xmax, ymax): (u16, u16)) -> io::Result<()> {
    make_box_on_screen(stdout, (0, xmax), (0, ymax))?;

    let title = format!(" {} ", title);
    let l = title.len() as u16 / 2;
    let offset = xmax / 2 - l;

    stdout
        .queue(cursor::MoveTo(offset as u16, 0))?
        .queue(style::PrintStyledContent(title.grey()))?;

    Ok(())
}

pub fn setup(title: String, stdout: &mut Stdout, buff_size: display::Size) {
    // Terminal Setup
    execute!(stdout, EnterAlternateScreen).expect("Cannot enter alternate screen.");
    execute!(stdout, cursor::Hide).expect("Failed to Hide the cursor");
    terminal::enable_raw_mode()
        .inspect_err(|_| clean_up(stdout))
        .expect("Unable to enter raw mode.");

    // Borders
    setup_borders(title, stdout, (buff_size.x_axis.1, buff_size.y_axis.1))
        .inspect_err(|_| clean_up(stdout))
        .expect("Something went wrong setting up Borders.");
}

pub fn clean_up(stdout: &mut Stdout) {
    // Terminal Clean Up
    terminal::disable_raw_mode().expect("Unable to leave raw mode.");
    execute!(stdout, cursor::Show).expect("Failed to Show the cursor.");
    execute!(stdout, LeaveAlternateScreen).expect("Cannot enter alternate screen.");
}
