use crate::display::Size;
use crate::event_capturer::Input;
use crate::game_display::*;
use std::io::stdout;
use std::sync::mpsc::Receiver;

pub fn start(rcv: Receiver<Input>, size: Size) {
    let message_pos: (u16, u16) = (size.x_axis.0 + 2, size.y_axis.1 - 2);
    let score_pos: (u16, u16) = (size.x_axis.1 - 16, size.y_axis.1 - 2);
    let mut stdout = stdout();


    let mut default = Input::Bs;
    let mut input: Input;
    let mut score: usize = 0;

    game_display(&mut stdout, size).expect("Unable to display game borders");
    print_score(&mut stdout, &score_pos, 0).expect("Unable to print score.");
    print_message(&mut stdout, &message_pos, &default).expect("Unable to display borders");

    loop {
        input = rcv.recv().expect("Failing to recv.");
        if default != input {
            if let Input::Quit = input {
                break;
            }

            print_score(&mut stdout, &score_pos, score).expect("Unable to print score.");
            score += 1;
            print_message(&mut stdout, &message_pos, &input).expect("Unable to display borders");
            default = input;
        }
    }
}
