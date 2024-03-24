mod display;
mod event_capturer;
mod game;
mod game_display;
mod window;
mod cell_automata;

use crate::display::Size;
use std::time::Duration;
use window::clean_up;

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<String>>();

    if args.len() != 2 || args[0] == "help" {
        eprintln!("Arguments required are: \n<cells> (800-2200) \t<delay> (80-750 millis)");
    } else {
        let alive_cells = args[0].parse::<usize>().expect("parsing of cells failed");
        let tdelay = args[1].parse::<u64>().expect("parsing of dealy failed");
        let delay = Duration::from_millis(tdelay);

        // Setup /////////////////////////////////////////////////////
        // This should be the only instance
        let mut stdout = std::io::stdout();

        // Getting the terminal dimensions
        let size = crossterm::terminal::size().expect("Failed to get the dimensions of terminal.");

        let buffer_size = display::Size {
            x_axis: (0, size.0),
            y_axis: (0, size.1),
        };

        // Setting up the terminal ///////////////////////////////////
        window::setup("Game of Life".to_string(), &mut stdout, buffer_size);
        //////////////////////////////////////////////////////////////

        // Do stuff /////////////////////////////////////////////////
        let (sdr, rcv) = std::sync::mpsc::channel();

        let game_size = Size {
            x_axis: (1, size.0 - 1),
            y_axis: (1, size.1 - 1),
        };

        let renderer_handle =
            std::thread::spawn(move || game::start(rcv, game_size, alive_cells, delay));
        let input_channel =
            std::thread::spawn(move || event_capturer::start(sdr, std::io::stdout()));

        let _ = input_channel
            .join()
            .inspect_err(|_| clean_up(&mut stdout))
            .expect("input_channel_failed");
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
