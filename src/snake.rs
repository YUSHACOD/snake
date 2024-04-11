use crate::display::Buffers;
use crate::event_capturer::Input;
use nanorand::{Rng, WyRand};
use std::collections::{HashSet, VecDeque};

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
    pub food: (usize, usize),
}

impl GameState {
    fn set_food(&mut self, food_points: &mut HashSet<(usize, usize)>) {
        let mut rnd = WyRand::new();
        for x in self.snake.iter() {
            food_points.take(x);
        }

        let possible_food_points: Vec<(usize, usize)> = food_points.clone().into_iter().collect();
        let random_index = rnd.generate_range(0..(possible_food_points.len()));

        self.food = possible_food_points[random_index];

        for x in self.snake.iter() {
            food_points.insert(*x);
        }
    }

    pub fn get_init_snake(
        &mut self,
        screen_buffer: &mut Buffers,
        food_points: &mut HashSet<(usize, usize)>,
    ) {
        // Initial snake at the middle of the frame
        let snake_start = (
            screen_buffer.size.x_axis.1 / 2 - 3,
            screen_buffer.size.y_axis.1 / 2,
        );
        for i in 0..5 {
            // Snake will be of length 5
            self.snake
                .push_front(((snake_start.0 + i) as usize, (snake_start.1) as usize));
        }

        for pos in self.snake.iter() {
            screen_buffer.matrix[pos.1][pos.0][0] = SNAKE_CHAR;
        }
        self.set_food(food_points);
    }

    fn get_new_head(
        &mut self,
        head: (usize, usize),
        input: &Input,
        (width, height): (usize, usize),
    ) -> (usize, usize) {
        let prev_direction = self.direction.clone();
        self.direction = input_to_direction(input, &prev_direction).unwrap_or(prev_direction);
        match self.direction {
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

    fn is_head_colliding_with_food(&self, new_head: &(usize, usize)) -> bool {
        new_head.0 == self.food.0 && new_head.1 == self.food.1
    }

    fn is_head_colliding_with_snake(&self, new_head: &(usize, usize)) -> bool {
        match self.snake.binary_search(new_head) {
            Ok(_) => true,
            Err(_) => false,
        }
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

pub fn create_next_frame(
    screen_buffer: &mut Buffers,
    game_state: &mut GameState,
    input: &Input,
    (width, height): (usize, usize),
    food_points: &mut HashSet<(usize, usize)>,
) -> bool {
    screen_buffer.frame += 1;
    let (current, previous) = (screen_buffer.frame % 2, (screen_buffer.frame + 1) % 2);
    for y in 0..height {
        for x in 0..width {
            screen_buffer.matrix[y][x][current] = screen_buffer.matrix[y][x][previous];
        }
    }

    // Setting new snake head position
    let snake_head = game_state.snake[0];
    let new_head = game_state.get_new_head(snake_head, input, (width, height));

    // Checking for Snake to Snake collision
    if game_state.is_head_colliding_with_snake(&new_head) {
        return false;
    }

    // Adding head if no collision with self
    game_state.snake.push_front(new_head);

    // Checking for food collision
    if game_state.is_head_colliding_with_food(&new_head) {
        game_state.set_food(food_points); // new food generation
        game_state.score += 1;
    } else {
        let trail = game_state.snake.pop_back().unwrap();
        screen_buffer.matrix[trail.1][trail.0][current] = '.';
    }

    for pos in game_state.snake.iter() {
        screen_buffer.matrix[pos.1][pos.0][current] = SNAKE_CHAR;
    }
    true
}
