use crossterm::{
    style::Stylize,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
    *,
};
use std::io::{self, Stdout};

fn setup_borders(title: String, stdout: &mut Stdout, (xmax, ymax): (u16, u16)) -> io::Result<()> {
    stdout
        .queue(cursor::MoveTo(0, 0))?
        .queue(style::PrintStyledContent("┌".grey()))?
        .queue(cursor::MoveTo(xmax as u16, 0))?
        .queue(style::PrintStyledContent("┐".grey()))?
        .queue(cursor::MoveTo(0, ymax as u16))?
        .queue(style::PrintStyledContent("└".grey()))?
        .queue(cursor::MoveTo(xmax as u16, ymax as u16))?
        .queue(style::PrintStyledContent("┘".grey()))?;

    for x in 1..xmax - 1 {
        stdout
            .queue(cursor::MoveTo(x as u16, 0))?
            .queue(style::PrintStyledContent("─".grey()))?
            .queue(cursor::MoveTo(x as u16, ymax as u16))?
            .queue(style::PrintStyledContent("─".grey()))?;
    }

    for y in 1..ymax - 1 {
        stdout
            .queue(cursor::MoveTo(0, y as u16))?
            .queue(style::PrintStyledContent("│".grey()))?
            .queue(cursor::MoveTo(xmax as u16, y as u16))?
            .queue(style::PrintStyledContent("│".grey()))?;
    }

    let title = String::from(format!(" {} ", title));
    let l = title.len() as u16 / 2;
    let offset = xmax / 2 - l;

    stdout
        .queue(cursor::MoveTo(offset as u16, 0))?
        .queue(style::PrintStyledContent(title.grey()))?;

    Ok(())
}

pub fn setup(title: String, stdout: &mut Stdout, buff_size: terminal::WindowSize) {
    // Terminal Setup
    execute!(stdout, EnterAlternateScreen).expect("Cannot enter alternate screen.");
    execute!(stdout, cursor::Hide).expect("Failed to Hide the cursor");
    terminal::enable_raw_mode().expect("Unable to enter raw mode.");

    // Borders
    setup_borders(title, stdout, (buff_size.columns, buff_size.rows))
        .inspect_err(|_| clean_up(stdout))
        .expect("Something went wrong setting up Borders.");
}

pub fn clean_up(stdout: &mut Stdout) {
    // Terminal Clean Up
    terminal::disable_raw_mode().expect("Unable to leave raw mode.");
    execute!(stdout, cursor::Show).expect("Failed to Show the cursor.");
    execute!(stdout, LeaveAlternateScreen).expect("Cannot enter alternate screen.");
}
