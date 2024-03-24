use crate::cell_automata::*;
use crate::display::*;
use crate::event_capturer::Input;
use crate::window::clean_up;
use crate::{game_display::*, window};
use std::io::{stdout, Stdout};
use std::sync::mpsc::Receiver;
use std::time::Duration;
use std::{u16, usize};

fn _message_score_box_testing(
    rcv: Receiver<Input>,
    stdout: &mut Stdout,
    size: Size,
    gen_pos: (u16, u16),
) {
    let mut default = Input::Bs;
    let mut input: Input;

    game_window_display(stdout, size)
        .inspect_err(|_| window::clean_up(stdout))
        .expect("Unable to display game borders");
    display_gen(stdout, &gen_pos, 0)
        .inspect_err(|_| window::clean_up(stdout))
        .expect("Error displaying gen.");

    loop {
        input = rcv.recv().unwrap_or(default.clone());

        if default != input {
            if let Input::Quit = input {
                break;
            }

            display_gen(stdout, &gen_pos, 0)
                .inspect_err(|_| window::clean_up(stdout))
                .expect("Error displaying gen.");
            default = input;
        }
    }
}

pub fn start(rcv: Receiver<Input>, size: Size, alive_cells: usize, delay: Duration) {
    let mut stdout = stdout();
    let gen_pos: (u16, u16) = (size.x_axis.0 + 3, size.y_axis.1 - 1);

    // Input handling
    let default = Input::Bs;
    let mut input: Input;

    // Cells
    let width = (size.x_axis.1 - size.x_axis.0 + 1) as usize;
    let height = (size.y_axis.1 - size.y_axis.0 + 1) as usize;

    let cell_buffer_size = Size {
        x_axis: (size.x_axis.0 + 1, size.x_axis.1 - 1),
        y_axis: (size.y_axis.0 + 1, size.y_axis.1 - 1),
    };
    let mut cell_buffer = Buffers {
        matrix: vec![vec![[' '; 2]; width]; height],
        size: cell_buffer_size,
        frame: 1,
    };

    // Setting the first gen
    first_gen(&mut cell_buffer.matrix, alive_cells, (width, height));

    // Setting game window
    display_gen(&mut stdout, &gen_pos, cell_buffer.frame)
        .inspect_err(|_| clean_up(&mut stdout))
        .expect("Failed to display gen");
    game_window_display(&mut stdout, size)
        .inspect_err(|_| window::clean_up(&mut stdout))
        .expect("Unable to display game borders");

    // Main Automata Loop
    while cell_buffer.frame < usize::MAX {
        input = rcv.try_recv().unwrap_or(default.clone());
        match input {
            Input::Quit => break,
            Input::Pause => {
                if block(&rcv).is_none() {
                    break;
                }
            }
            _ => (),
        };

        next_generation(&mut cell_buffer.matrix, cell_buffer.frame, (width, height));
        std::thread::sleep(delay);

        display_gen(&mut stdout, &gen_pos, cell_buffer.frame)
            .inspect_err(|_| clean_up(&mut stdout))
            .expect("Failed to display gen");
        display(&mut stdout, &cell_buffer)
            .inspect_err(|_| clean_up(&mut stdout))
            .expect("Failed to display automata");

        cell_buffer.frame += 1;
    }
}

fn block(rcv: &Receiver<Input>) -> Option<()> {
    let mut result = Some(());
    loop {
        match rcv.recv().unwrap_or(Input::Bs) {
            Input::Start | Input::Pause => {
                break;
            }
            Input::Quit => {
                result = None;
                break;
            }
            _ => {}
        }
    }
    result
}
