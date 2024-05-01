use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/// Position of something on the bitmap, in pixels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(C, align(4))]
pub struct Pos {
	pub x: i16,
	pub y: i16,
}

impl Pos {
	pub const ZERO: Self = Self { x: 0, y: 0 };
}

#[inline]
pub const fn pos(x: i16, y: i16) -> Pos {
	Pos { x, y }
}

impl Add for Pos {
	type Output = Pos;

	fn add(mut self, rhs: Self) -> Self::Output {
		self.x += rhs.x;
		self.y += rhs.y;
		self
	}
}

impl AddAssign for Pos {
	fn add_assign(&mut self, rhs: Self) {
		*self = *self + rhs;
	}
}

impl Sub for Pos {
	type Output = Pos;

	fn sub(mut self, rhs: Self) -> Self::Output {
		self.x -= rhs.x;
		self.y -= rhs.y;
		self
	}
}

impl SubAssign for Pos {
	fn sub_assign(&mut self, rhs: Self) {
		*self = *self - rhs;
	}
}

impl Mul<i16> for Pos {
	type Output = Pos;

	fn mul(mut self, rhs: i16) -> Self::Output {
		self.x *= rhs;
		self.y *= rhs;
		self
	}
}

impl Mul<f32> for Pos {
	type Output = Pos;

	fn mul(mut self, rhs: f32) -> Self::Output {
		self.x = (self.x as f32 * rhs) as i16;
		self.y = (self.y as f32 * rhs) as i16;
		self
	}
}

impl MulAssign<i16> for Pos {
	fn mul_assign(&mut self, rhs: i16) {
		*self = *self * rhs;
	}
}

impl Div<i16> for Pos {
	type Output = Pos;

	fn div(mut self, rhs: i16) -> Self::Output {
		self.x /= rhs;
		self.y /= rhs;
		self
	}
}

impl DivAssign<i16> for Pos {
	fn div_assign(&mut self, rhs: i16) {
		*self = *self / rhs;
	}
}
