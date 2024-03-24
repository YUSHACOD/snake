use crate::display::{self, Buffers, Size};
use crate::event_capturer::Input;
use crate::snake::{create_next_frame, get_init_snake, Direction, GameState};
use crate::window::clean_up;
use crate::{game_display::*, window};
use std::collections::VecDeque;
use std::io::{stdout, Stdout};
use std::sync::mpsc::Receiver;
use std::time::Duration;
use std::usize;

fn _message_score_box_testing(
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

fn block(rcv: &Receiver<Input>) -> Option<()> {
    let mut result = Some(());
    loop {
        match rcv.recv().unwrap_or(Input::Bs) {
            Input::Quit => {
                result = None;
                break;
            }
            Input::Pause | Input::Start => break,
            _ => (),
        };
    }
    result
}

pub fn start(rcv: Receiver<Input>, size: Size, delay: Duration) {
    // Only stdout handle
    let mut stdout = stdout();

    // Input
    let default = Input::Bs;
    let mut input: Input;

    // Message and Score Position
    let message_pos: (u16, u16) = (size.x_axis.0 + 1, size.y_axis.1 - 1);
    let score_pos: (u16, u16) = (size.x_axis.1 - 16, size.y_axis.1 - 1);

    // Game state and Screen Buffer
    let width = (size.x_axis.1 - size.x_axis.0 - 1) as usize;
    let height = (size.y_axis.1 - size.y_axis.0 - 1 - 3) as usize;
    let screen_size = Size {
        x_axis: (size.x_axis.0 + 1, size.x_axis.1 - 1),
        y_axis: (size.y_axis.0 + 1, size.y_axis.1 - 4),
    };

    let mut screen_buffer = Buffers {
        matrix: vec![vec![['.', ' ']; width]; height],
        size: screen_size,
        frame: 0,
    };

    let mut game_state = GameState {
        snake: VecDeque::new(),
        direction: Direction::Right,
        score: 0,
    };

    // Initial Setup
    game_display(&mut stdout, size)
        .inspect_err(|_| window::clean_up(&mut stdout))
        .expect("Unable to display game borders");
    print_score(&mut stdout, &score_pos, 0)
        .inspect_err(|_| window::clean_up(&mut stdout))
        .expect("Unable to print score in score box.");
    print_message(&mut stdout, &message_pos, &default)
        .inspect_err(|_| window::clean_up(&mut stdout))
        .expect("Unable to display borders");

    // Getting initial snake in game_state and screen_buffer
    get_init_snake(&mut game_state, &mut screen_buffer);
    display::display(&mut stdout, &mut screen_buffer, (2, 2))
        .inspect_err(|_| clean_up(&mut stdout))
        .expect("Failed to display screen_buffer");

    // Initialy stopping for user to start
    if block(&rcv).is_none() {
        return;
    };

    // Game Loop
    while screen_buffer.frame < usize::MAX {
        // Check for quit of pause
        input = rcv.try_recv().unwrap_or(default.clone());
        print_message(&mut stdout, &message_pos, &input)
            .inspect_err(|_| window::clean_up(&mut stdout))
            .expect("Unable to display borders");
        match input {
            Input::Quit => break,
            Input::Pause => {
                if block(&rcv).is_none() {
                    break;
                }
            }
            _ => (),
        };

        // New buffer
        create_next_frame(&mut screen_buffer, &mut game_state, &input, (width, height));

        // Displaying the screen_buffer and score
        print_score(&mut stdout, &score_pos, game_state.score)
            .inspect_err(|_| window::clean_up(&mut stdout))
            .expect("Unable to print score in score box.");

        display::display(&mut stdout, &mut screen_buffer, (2, 2))
            .inspect_err(|_| clean_up(&mut stdout))
            .expect("Failed to display screen_buffer");
        std::thread::sleep(delay);
    }
}
