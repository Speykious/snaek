pub mod snaeksheet;

use std::collections::VecDeque;

use rand::rngs::ThreadRng;
use rand::Rng;
pub use snaeksheet::{snaek_sheet, SnaekSheet};

use crate::math::pos::{self, pos, Pos};
use crate::math::size::Size;

fn rand_pos(rng: &mut ThreadRng, size: Size) -> Pos {
	let x = rng.gen_range(0..size.w as i16);
	let y = rng.gen_range(0..size.h as i16);
	pos(x, y)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
	Up,
	Right,
	Down,
	Left,
}

pub struct SnakeGame {
	thread_rng: ThreadRng,
	playfield_size: Size,
	snake: VecDeque<Pos>,
	bananas_eaten: u32,
	curr_banana_pos: Pos,
	direction: Direction,
	is_dead: bool,
}

impl SnakeGame {
	pub fn new(playfield_size: Size) -> Self {
		let mut thread_rng = rand::thread_rng();
		let curr_banana_pos = rand_pos(&mut thread_rng, playfield_size);

		let snake_head = pos(playfield_size.w as i16 / 2, playfield_size.h as i16 / 2);
		let snake_tail = pos(snake_head.x - 1, snake_head.y);

		Self {
			thread_rng,
			playfield_size,
			snake: [snake_head, snake_tail].into_iter().collect(),
			bananas_eaten: 0,
			curr_banana_pos,
			direction: Direction::Right,
			is_dead: false,
		}
	}

	pub fn change_direction(&mut self, direction: Direction) {
		let opposite = match self.direction {
			Direction::Up => Direction::Down,
			Direction::Right => Direction::Left,
			Direction::Down => Direction::Up,
			Direction::Left => Direction::Right,
		};

		if direction == opposite {
			return;
		}

		self.direction = direction;
	}

	pub fn update(&mut self) {
		if self.is_dead {
			return;
		}

		let width = self.playfield_size.w as i16;
		let height = self.playfield_size.h as i16;

		let next_pos = {
			let curr_pos = self.snake[0];

			let pos_offset = match self.direction {
				Direction::Up => pos(0, -1),
				Direction::Right => pos(1, 0),
				Direction::Down => pos(0, 1),
				Direction::Left => pos(-1, 0),
			};

			let next_x = (curr_pos.x + pos_offset.x) % width;
			let next_y = (curr_pos.y + pos_offset.y) % height;

			let next_x = if next_x < 0 { next_x + width } else { next_x };
			let next_y = if next_y < 0 { next_y + height } else { next_y };

			pos(next_x, next_y)
		};

		// snake collision!
		if self.snake.iter().any(|&part| part == next_pos) {
			self.is_dead = true;
			return;
		}

		if next_pos == self.curr_banana_pos {
			// banana eating logic
			self.curr_banana_pos = rand_pos(&mut self.thread_rng, self.playfield_size);
			self.bananas_eaten += 1;

			self.snake.push_front(next_pos);
		} else {
			// snake be snakin
			self.snake.push_front(next_pos);
			self.snake.pop_back();
		}
	}

	pub fn restart(&mut self) {
		let snake_head = pos(self.playfield_size.w as i16 / 2, self.playfield_size.h as i16 / 2);
		let snake_tail = pos(snake_head.x - 1, snake_head.y);

		self.snake = [snake_head, snake_tail].into_iter().collect();
		self.bananas_eaten = 0;
		self.curr_banana_pos = rand_pos(&mut self.thread_rng, self.playfield_size);
		self.direction = Direction::Right;
		self.is_dead = false;
	}

	pub fn is_dead(&self) -> bool {
		self.is_dead
	}

	pub fn bananas_eaten(&self) -> u32 {
		self.bananas_eaten
	}

	pub fn curr_banana_pos(&self) -> Pos {
		self.curr_banana_pos
	}

	pub fn snake_len(&self) -> usize {
		self.snake.len()
	}

	pub fn snake_iter(&self) -> impl Iterator<Item = Pos> + '_ {
		self.snake.iter().copied()
	}

	pub fn direction(&self) -> Direction {
		self.direction
	}
}
