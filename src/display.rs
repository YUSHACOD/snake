use crossterm::style;
use crossterm::{cursor, style::Stylize, QueueableCommand};
use std::io::Stdout;
use std::io::{self, Write};

pub struct Size {
    pub x_axis: (u16, u16),
    pub y_axis: (u16, u16),
}

pub struct Buffers {
    pub matrix: Vec<Vec<[char; 2]>>,
    pub size: Size,
    pub frame: usize,
}

pub fn _display(stdout: &mut Stdout, buffers: &mut Buffers) -> io::Result<()> {
    let (x_start, x_end) = buffers.size.x_axis;
    let (y_start, y_end) = buffers.size.y_axis;
    let (current, previous) = (buffers.frame % 2, (buffers.frame + 1) % 2);

    for y in y_start..=y_end {
        for x in x_start..=x_end {
            let (y, x) = (y as usize, x as usize); // Typcasting for indexing

            if buffers.matrix[y][x][current] != buffers.matrix[y][x][previous] {
                stdout
                    .queue(cursor::MoveTo(x as u16, y as u16))?
                    .queue(style::Print(buffers.matrix[y][x][current].green()))?;
            }
        }
    }

    stdout.flush()?;
    Ok(())
}
