use crate::display::Buffers;
use crate::event_capturer::Input;
use std::collections::VecDeque;

const SNAKE_CHAR: char = '#';

pub struct GameState {
    pub snake: VecDeque<(usize, usize)>,
    pub score: usize,
}

pub fn get_init_snake(game_state: &mut GameState, screen_buffer: &mut Buffers) {
    // Initial snake at the middle of the frame
    let snake_start = (
        screen_buffer.size.x_axis.1 / 2 - 3,
        screen_buffer.size.y_axis.1 / 2,
    );
    for i in 0..5 {
        // Snake will be of length 5
        game_state
            .snake
            .push_front(((snake_start.0 + i) as usize, (snake_start.1) as usize));
    }

    for pos in game_state.snake.iter() {
        screen_buffer.matrix[pos.1][pos.0][0] = SNAKE_CHAR;
    }
}

pub fn create_next_frame(
    screen_buffer: &mut Buffers,
    game_state: &mut GameState,
    _input: &Input,
    (width, height): (usize, usize),
) {
    screen_buffer.frame += 1;
    let (current, previous) = (screen_buffer.frame % 2, (screen_buffer.frame + 1) % 2);

    // Setting new snake Position
    let snake_head = game_state.snake[0];
    let trail = game_state.snake.pop_back().unwrap();
    game_state
        .snake
        .push_front(((snake_head.0 + 1) % width, (snake_head.1) % height));

    for y in 0..height {
        for x in 0..width {
            screen_buffer.matrix[y][x][current] = screen_buffer.matrix[y][x][previous];
        }
    }
    screen_buffer.matrix[trail.1][trail.0][current] = '.';

    for pos in game_state.snake.iter() {
        screen_buffer.matrix[pos.1][pos.0][current] = SNAKE_CHAR;
    }
}
