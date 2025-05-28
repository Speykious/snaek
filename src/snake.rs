pub mod snaeksheet;

use std::time::{Duration, Instant};

use rand::rngs::ThreadRng;
use rand::Rng;
pub use snaeksheet::{snaek_sheet, SnaekSheet};

use crate::math::pos::{pos, Pos};
use crate::math::size::Size;

fn rand_pos(rng: &mut ThreadRng, size: Size) -> Pos {
	let x = rng.gen_range(0..size.w as i16);
	let y = rng.gen_range(0..size.h as i16);
	pos(x, y)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
	Up = 0,
	Right = 1,
	Down = 2,
	Left = 3,
}

impl Direction {
	pub const fn pos_offset(&self) -> Pos {
		match self {
			Direction::Up => pos(0, -1),
			Direction::Right => pos(1, 0),
			Direction::Down => pos(0, 1),
			Direction::Left => pos(-1, 0),
		}
	}

	pub const fn opposite(&self) -> Self {
		match self {
			Direction::Up => Direction::Down,
			Direction::Right => Direction::Left,
			Direction::Down => Direction::Up,
			Direction::Left => Direction::Right,
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Banana {
	Yellow = 1,
	Red = 2,
	Cyan = 3,
}

pub struct SnakeGame {
	thread_rng: ThreadRng,
	size: Size,
	playfield: Box<[Slot]>,
	snake_head: Pos,
	snake_tail: Pos,
	bananas_eaten: u32,
	speed: f32,
	direction: Direction,
	ate_banana: bool,
	is_dead: bool,

	start: Instant,
	duration: Duration,
}

impl SnakeGame {
	pub fn new(playfield_size: Size) -> Self {
		let thread_rng = rand::thread_rng();

		let playfield = vec![Slot::default(); playfield_size.w as usize * playfield_size.h as usize].into_boxed_slice();
		let snake_head = pos((playfield_size.w / 2) as i16, (playfield_size.h / 2) as i16);
		let snake_tail = snake_head - pos(-1, 0);

		let mut game = Self {
			thread_rng,
			size: playfield_size,
			playfield,
			snake_head,
			snake_tail,
			bananas_eaten: 0,
			direction: Direction::Right,
			ate_banana: false,
			is_dead: false,
			speed: 3.0,

			start: Instant::now(),
			duration: Duration::default(),
		};

		game.restart();
		game
	}

	pub fn change_direction(&mut self, direction: Direction) {
		let opposite = self.direction.opposite();

		if direction == opposite {
			return;
		}

		self.direction = direction;
	}

	pub fn update_duration(&mut self) {
		if self.is_dead {
			return;
		}

		self.duration = self.start.elapsed();
	}

	pub fn update(&mut self) {
		if self.is_dead {
			return;
		}

		self.ate_banana = false;
		self.playfield[self.slot_index(self.snake_head)].set_direction_next(self.direction);

		let next_head = self.next_at(self.snake_head);

		let next_slot = self.playfield[self.slot_index(next_head)];
		if next_slot.banana().is_some() {
			// banana eating logic
			self.ate_banana = true;

			// snake collision!
			// Since the tail stays in place, any snake part will make the snake die.
			if next_slot.has_snake() {
				self.is_dead = true;
				return;
			}

			// push head
			let curr_slot = &mut self.playfield[self.slot_index(self.snake_head)];
			curr_slot.set_direction_next(self.direction);
			curr_slot.set_snake_tail();

			self.snake_head = self.wrap_pos(next_head);
			let next_slot = &mut self.playfield[self.slot_index(next_head)];
			next_slot.set_direction_prev(self.direction.opposite());
			next_slot.set_snake_head();

			// eat banana
			next_slot.set_banana(None);
			self.bananas_eaten += 1;
			self.speed += 0.1;

			self.place_banana();
		} else {
			// snake be snakin

			// snake collision!
			// Here it's fine if it's just the tail, since we're popping it right after.
			if next_slot.has_snake_head() {
				self.is_dead = true;
				return;
			}

			// pop tail
			let next_tail = self.next_at(self.snake_tail);

			let curr_slot = &mut self.playfield[self.slot_index(self.snake_tail)];
			curr_slot.remove_snake();

			self.snake_tail = self.wrap_pos(next_tail);
			let next_slot = &mut self.playfield[self.slot_index(next_tail)];
			next_slot.remove_snake();
			next_slot.set_snake_tail();

			// push head
			let curr_slot = &mut self.playfield[self.slot_index(self.snake_head)];
			curr_slot.set_direction_next(self.direction);
			curr_slot.set_snake_tail();

			self.snake_head = self.wrap_pos(next_head);
			let next_slot = &mut self.playfield[self.slot_index(next_head)];
			next_slot.set_direction_prev(self.direction.opposite());
			next_slot.set_snake_head();
		}
	}

	pub fn restart(&mut self) {
		self.playfield.fill(Slot::default());

		self.snake_head = pos(self.size.w as i16 / 2, self.size.h as i16 / 2);
		self.snake_tail = pos(self.snake_head.x - 1, self.snake_head.y);

		let head_slot = &mut self.playfield[self.slot_index(self.snake_head)];
		head_slot.set_direction_prev(Direction::Left);
		head_slot.set_direction_next(self.direction);
		head_slot.set_snake_head();

		let tail_slot = &mut self.playfield[self.slot_index(self.snake_tail)];
		tail_slot.set_direction_next(Direction::Right);
		tail_slot.set_snake_tail();

		self.bananas_eaten = 0;
		self.speed = 3.0;
		self.direction = Direction::Right;
		self.is_dead = false;

		self.place_banana();
		self.start = Instant::now();
		self.duration = Duration::default();
	}

	fn place_banana(&mut self) {
		loop {
			let banana_pos = rand_pos(&mut self.thread_rng, self.size);
			let slot = &mut self.playfield[self.slot_index(banana_pos)];
			if slot.has_snake() {
				continue;
			}

			let banana = match self.thread_rng.gen_range(0..100) {
				0 => Banana::Cyan,
				1..=9 => Banana::Red,
				_ => Banana::Yellow,
			};
			slot.set_banana(Some(banana));
			break;
		}
	}

	pub fn size(&self) -> Size {
		self.size
	}

	pub fn slot_at(&self, pos: Pos) -> Slot {
		self.playfield[self.slot_index(pos)]
	}

	pub fn snake_head(&self) -> Pos {
		self.snake_head
	}

	pub fn ate_banana(&self) -> bool {
		self.ate_banana
	}

	pub fn is_dead(&self) -> bool {
		self.is_dead
	}

	pub fn bananas_eaten(&self) -> u32 {
		self.bananas_eaten
	}

	pub fn speed(&self) -> f32 {
		self.speed
	}

	pub fn direction(&self) -> Direction {
		self.direction
	}

	pub fn duration(&self) -> Duration {
		self.duration
	}

	#[inline]
	fn wrap_pos(&self, p: Pos) -> Pos {
		let w = self.size.w as i16;
		let h = self.size.h as i16;

		let x = if p.x < 0 { -(-p.x % w) + w } else { p.x % w };
		let y = if p.y < 0 { -(-p.y % h) + h } else { p.y % h };

		pos(x, y)
	}

	#[inline]
	fn slot_index(&self, pos: Pos) -> usize {
		let pos = self.wrap_pos(pos);
		pos.y as usize * self.size.w as usize + pos.x as usize
	}

	#[inline]
	fn next_at(&self, pos: Pos) -> Pos {
		pos + self.playfield[self.slot_index(pos)].direction_next().pos_offset()
	}
}

/// A slot on the playfield.
///
/// # Anatomy of a slot type
///
/// ```ignore
/// xx xx   xx xx
/// || ||   || ||
/// || ||   || ++---- direction enum (prev)
/// || ||   ++------- direction enum (next)
/// || ||
/// || ++------------ snake enum
/// ++--------------- banana enum
/// ```
///
/// Direction enum:
/// ```ignore
/// +------------------------+
/// | up   (00) | right (01) |
/// | down (10) | left  (11) |
/// +------------------------+
/// ```
///
/// Snake enum:
/// ```ignore
/// +-----------------------+
/// | none (00) | head (01) |
/// | tail (10) | body (11) |
/// +-----------------------+
/// ```
///
/// Banana enum:
/// ```ignore
/// +-------------------------+
/// | none (00) | yellow (01) |
/// | red  (10) | cyan   (11) |
/// +-------------------------+
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct Slot(u8);

impl Slot {
	#[inline]
	pub fn direction_prev(&self) -> Direction {
		match self.0 & 0b0000_0011 {
			0b0000_0000 => Direction::Up,
			0b0000_0001 => Direction::Right,
			0b0000_0010 => Direction::Down,
			0b0000_0011 => Direction::Left,
			_ => unreachable!(),
		}
	}

	#[inline]
	pub fn set_direction_prev(&mut self, direction: Direction) {
		self.0 &= 0b1111_1100;
		self.0 |= match direction {
			Direction::Up => 0b0000_0000,
			Direction::Right => 0b0000_0001,
			Direction::Down => 0b0000_0010,
			Direction::Left => 0b0000_0011,
		}
	}

	#[inline]
	pub fn direction_next(&self) -> Direction {
		match self.0 & 0b0000_1100 {
			0b0000_0000 => Direction::Up,
			0b0000_0100 => Direction::Right,
			0b0000_1000 => Direction::Down,
			0b0000_1100 => Direction::Left,
			_ => unreachable!(),
		}
	}

	#[inline]
	pub fn set_direction_next(&mut self, direction: Direction) {
		self.0 &= 0b1111_0011;
		self.0 |= match direction {
			Direction::Up => 0b0000_0000,
			Direction::Right => 0b0000_0100,
			Direction::Down => 0b0000_1000,
			Direction::Left => 0b0000_1100,
		}
	}

	#[inline]
	pub fn has_snake_head(&self) -> bool {
		self.0 & 0b0001_0000 > 0
	}

	#[inline]
	pub fn set_snake_head(&mut self) {
		self.0 |= 0b0001_0000;
	}

	#[inline]
	pub fn has_snake_tail(&self) -> bool {
		self.0 & 0b0010_0000 > 0
	}

	#[inline]
	pub fn set_snake_tail(&mut self) {
		self.0 |= 0b0010_0000;
	}

	#[inline]
	pub fn has_snake(&self) -> bool {
		self.0 & 0b0011_0000 > 0
	}

	pub fn remove_snake(&mut self) {
		self.0 &= 0b1100_1111;
	}

	#[inline]
	pub fn banana(&self) -> Option<Banana> {
		match self.0 & 0b1100_0000 {
			0b0000_0000 => None,
			0b0100_0000 => Some(Banana::Yellow),
			0b1000_0000 => Some(Banana::Red),
			0b1100_0000 => Some(Banana::Cyan),
			_ => unreachable!(),
		}
	}

	#[inline]
	pub fn set_banana(&mut self, banana: Option<Banana>) {
		self.0 &= 0b0011_1111;
		self.0 |= match banana {
			None => 0b0000_0000,
			Some(Banana::Yellow) => 0b0100_0000,
			Some(Banana::Red) => 0b1000_0000,
			Some(Banana::Cyan) => 0b1100_0000,
		}
	}
}
