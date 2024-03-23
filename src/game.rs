use crate::display::Size;
use crate::event_capturer::Input;
use crate::{game_display::*, window};
use std::io::{stdout, Stdout};
use std::sync::mpsc::Receiver;
use std::u16;

fn message_score_box_testing(
    rcv: Receiver<Input>,
    stdout: &mut Stdout,
    size: Size,
    message_pos: (u16, u16),
    score_pos: (u16, u16),
) {
    let mut default = Input::Bs;
    let mut input: Input;
    let mut score: usize = 0;

    game_display(stdout, size)
        .inspect_err(|_| window::clean_up(stdout))
        .expect("Unable to display game borders");
    print_score(stdout, &score_pos, 0)
        .inspect_err(|_| window::clean_up(stdout))
        .expect("Unable to print score in score box.");
    print_message(stdout, &message_pos, &default)
        .inspect_err(|_| window::clean_up(stdout))
        .expect("Unable to display borders");

    loop {
        input = rcv.recv().unwrap_or(default.clone());
        print_score(stdout, &score_pos, score).expect("Unable to print score.");
        score += 1;

        if default != input {
            if let Input::Quit = input {
                break;
            }

            print_message(stdout, &message_pos, &input)
                .inspect_err(|_| window::clean_up(stdout))
                .expect("Failed to print message in message box");
            default = input;
        }
    }
}

pub fn start(rcv: Receiver<Input>, size: Size) {
    let message_pos: (u16, u16) = (size.x_axis.0 + 2, size.y_axis.1 - 2);
    let score_pos: (u16, u16) = (size.x_axis.1 - 16, size.y_axis.1 - 2);
    let mut stdout = stdout();

    message_score_box_testing(rcv, &mut stdout, size, message_pos, score_pos)
}
