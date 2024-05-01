use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/// Size of a rectangle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(C, align(4))]
pub struct Size {
	pub w: u16,
	pub h: u16,
}

impl Size {
	pub const ZERO: Self = Self { w: 0, h: 0 };
}

#[inline]
pub const fn size(w: u16, h: u16) -> Size {
	Size { w, h }
}

impl Add for Size {
	type Output = Size;

	fn add(mut self, rhs: Self) -> Self::Output {
		self.w += rhs.w;
		self.h += rhs.h;
		self
	}
}

impl AddAssign for Size {
	fn add_assign(&mut self, rhs: Self) {
		*self = *self + rhs;
	}
}

impl Sub for Size {
	type Output = Size;

	fn sub(mut self, rhs: Self) -> Self::Output {
		self.w -= rhs.w;
		self.h -= rhs.h;
		self
	}
}

impl SubAssign for Size {
	fn sub_assign(&mut self, rhs: Self) {
		*self = *self - rhs;
	}
}

impl Mul<u16> for Size {
	type Output = Size;

	fn mul(mut self, rhs: u16) -> Self::Output {
		self.w *= rhs;
		self.h *= rhs;
		self
	}
}

impl Mul<f32> for Size {
	type Output = Size;

	fn mul(mut self, rhs: f32) -> Self::Output {
		self.w = (self.w as f32 * rhs) as u16;
		self.h = (self.h as f32 * rhs) as u16;
		self
	}
}

impl MulAssign<u16> for Size {
	fn mul_assign(&mut self, rhs: u16) {
		*self = *self * rhs;
	}
}

impl Div<u16> for Size {
	type Output = Size;

	fn div(mut self, rhs: u16) -> Self::Output {
		self.w /= rhs;
		self.h /= rhs;
		self
	}
}

impl DivAssign<u16> for Size {
	fn div_assign(&mut self, rhs: u16) {
		*self = *self / rhs;
	}
}
