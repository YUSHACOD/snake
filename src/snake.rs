use crate::display::Buffers;
use crate::event_capturer::Input;
use std::collections::VecDeque;

const SNAKE_CHAR: char = '#';

#[derive(Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct GameState {
    pub snake: VecDeque<(usize, usize)>,
    pub score: usize,
    pub direction: Direction,
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

fn input_to_direction(input: &Input, prev_direction: &Direction) -> Option<Direction> {
    match input {
        Input::Up => match prev_direction {
            Direction::Down => None,
            _ => Some(Direction::Up),
        },
        Input::Down => match prev_direction {
            Direction::Up => None,
            _ => Some(Direction::Down),
        },
        Input::Left => match prev_direction {
            Direction::Right => None,
            _ => Some(Direction::Left),
        },
        Input::Right => match prev_direction {
            Direction::Left => None,
            _ => Some(Direction::Right),
        },
        _ => None,
    }
}

fn get_new_head(
    head: (usize, usize),
    game_state: &mut GameState,
    input: &Input,
    (width, height): (usize, usize),
) -> (usize, usize) {
    let prev_direction = game_state.direction.clone();
    game_state.direction = input_to_direction(input, &prev_direction).unwrap_or(prev_direction);
    match game_state.direction {
        Direction::Up => (
            head.0,
            ((head.1 as isize) - 1 + height as isize) as usize % height,
        ),
        Direction::Down => (head.0, (head.1 + 1) % height),
        Direction::Left => (
            (head.0 as isize - 1 + width as isize) as usize % width,
            head.1,
        ),
        Direction::Right => ((head.0 + 1) % width, head.1),
    }
}

pub fn create_next_frame(
    screen_buffer: &mut Buffers,
    game_state: &mut GameState,
    input: &Input,
    (width, height): (usize, usize),
) {
    screen_buffer.frame += 1;
    let (current, previous) = (screen_buffer.frame % 2, (screen_buffer.frame + 1) % 2);

    // Setting new snake Position
    let snake_head = game_state.snake[0];
    let trail = game_state.snake.pop_back().unwrap();
    let new_head = get_new_head(snake_head, game_state, input, (width, height));
    game_state.snake.push_front(new_head);

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
