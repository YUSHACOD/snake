mod display;
mod event_capturer;
mod game;
mod game_display;
mod snake;
mod window;

use std::time::Duration;

use window::clean_up;

use crate::display::Size;

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<String>>();

    if args.len() != 2 {
        eprintln!("Arguments required are: \n<name> (1-30) \t<delay> (80-750 millis)");
    } else {
        let title = args[0].to_string();
        let delay = Duration::from_millis(args[1].parse().unwrap_or(100));

        // Setup /////////////////////////////////////////////////////
        // This should be the only instance
        let mut stdout = std::io::stdout();

        // Getting the terminal dimensions
        let size = crossterm::terminal::size().expect("Failed to get the dimensions of terminal.");

        let buffer_size = display::Size {
            x_axis: (0, size.0 - 1),
            y_axis: (0, size.1 - 1),
        };

        // Setting up the terminal ///////////////////////////////////
        window::setup(title, &mut stdout, &buffer_size);
        //////////////////////////////////////////////////////////////

        // Do stuff /////////////////////////////////////////////////
        let (sdr, rcv) = std::sync::mpsc::channel();

        let game_size = Size {
            x_axis: (buffer_size.x_axis.0 + 1, buffer_size.x_axis.1 - 1),
            y_axis: (buffer_size.y_axis.0 + 1, buffer_size.y_axis.1 - 1),
        };

        let renderer_handle = std::thread::spawn(move || game::start(rcv, game_size, delay));
        event_capturer::start(sdr, &mut stdout)
            .inspect_err(|_| clean_up(&mut stdout))
            .expect("Fucked Input");

        renderer_handle
            .join()
            .inspect_err(|_| clean_up(&mut stdout))
            .expect("renderer_failed");
        //////////////////////////////////////////////////////////////

        // Cleaning up the terminal //////////////////////////////////
        window::clean_up(&mut stdout);
        //////////////////////////////////////////////////////////////
    }
}
