use std::ops::{
	Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign, Mul, MulAssign,
	Sub, SubAssign,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(C, align(4))]
pub struct Color {
	pub a: u8,
	pub r: u8,
	pub g: u8,
	pub b: u8,
}

impl Color {
	pub const TRANSPARENT: Self = Self::from_hex(0x00000000);
	pub const WHITE: Self = Self::from_hex(0xffffffff);

	#[inline]
	pub const fn from_hex(hex: u32) -> Self {
		// We're doing math here to not assume big- or little-endian

		Self {
			a: ((hex >> 24) & 0xff) as u8,
			r: ((hex >> 16) & 0xff) as u8,
			g: ((hex >> 8) & 0xff) as u8,
			b: (hex & 0xff) as u8,
		}
	}

	#[inline]
	pub const fn to_u32(self) -> u32 {
		// We're doing math here to not assume big- or little-endian

		let mut val = (self.a as u32) << 24;
		val |= (self.r as u32) << 16;
		val |= (self.g as u32) << 8;
		val |= self.b as u32;
		val
	}
}

impl Add for Color {
	type Output = Self;

	fn add(mut self, rhs: Self) -> Self::Output {
		self.a += rhs.a;
		self.r += rhs.r;
		self.g += rhs.g;
		self.b += rhs.b;
		self
	}
}

impl AddAssign for Color {
	fn add_assign(&mut self, rhs: Self) {
		*self = *self + rhs;
	}
}

impl Sub for Color {
	type Output = Self;

	fn sub(mut self, rhs: Self) -> Self::Output {
		self.a -= rhs.a;
		self.r -= rhs.r;
		self.g -= rhs.g;
		self.b -= rhs.b;
		self
	}
}

impl SubAssign for Color {
	fn sub_assign(&mut self, rhs: Self) {
		*self = *self - rhs;
	}
}

impl Mul<u8> for Color {
	type Output = Self;

	fn mul(mut self, rhs: u8) -> Self::Output {
		self.a *= rhs;
		self.r *= rhs;
		self.g *= rhs;
		self.b *= rhs;
		self
	}
}

impl Mul<f32> for Color {
	type Output = Self;

	fn mul(mut self, rhs: f32) -> Self::Output {
		self.a = (self.a as f32 * rhs) as u8;
		self.r = (self.r as f32 * rhs) as u8;
		self.g = (self.g as f32 * rhs) as u8;
		self.b = (self.b as f32 * rhs) as u8;
		self
	}
}

impl MulAssign<u8> for Color {
	fn mul_assign(&mut self, rhs: u8) {
		*self = *self * rhs;
	}
}

impl Div<u8> for Color {
	type Output = Self;

	fn div(mut self, rhs: u8) -> Self::Output {
		self.a /= rhs;
		self.r /= rhs;
		self.g /= rhs;
		self.b /= rhs;
		self
	}
}

impl DivAssign<u8> for Color {
	fn div_assign(&mut self, rhs: u8) {
		*self = *self / rhs;
	}
}

impl BitAnd for Color {
	type Output = Self;

	fn bitand(mut self, rhs: Self) -> Self::Output {
		self.a &= rhs.a;
		self.r &= rhs.r;
		self.g &= rhs.g;
		self.b &= rhs.b;
		self
	}
}

impl BitAndAssign for Color {
	fn bitand_assign(&mut self, rhs: Self) {
		*self = *self & rhs;
	}
}

impl BitOr for Color {
	type Output = Self;

	fn bitor(mut self, rhs: Self) -> Self::Output {
		self.a |= rhs.a;
		self.r |= rhs.r;
		self.g |= rhs.g;
		self.b |= rhs.b;
		self
	}
}

impl BitOrAssign for Color {
	fn bitor_assign(&mut self, rhs: Self) {
		*self = *self | rhs;
	}
}

impl BitXor for Color {
	type Output = Self;

	fn bitxor(mut self, rhs: Self) -> Self::Output {
		self.a ^= rhs.a;
		self.r ^= rhs.r;
		self.g ^= rhs.g;
		self.b ^= rhs.b;
		self
	}
}

impl BitXorAssign for Color {
	fn bitxor_assign(&mut self, rhs: Self) {
		*self = *self ^ rhs;
	}
}

pub mod alphacomp {
	//! Alpha composition functions.
	//!
	//! ![Alpha compositing](https://upload.wikimedia.org/wikipedia/commons/thumb/2/2a/Alpha_compositing.svg/642px-Alpha_compositing.svg.png)

	use super::Color;

	/// An alpha composition function.
	pub type AlphaCompFn = fn(Color, Color) -> Color;

	/// Computes `A over B`.
	#[inline]
	pub fn over(pixa: Color, pixb: Color) -> Color {
		pixa * (pixa.a as f32 / 255.) + pixb * (1. - pixa.a as f32 / 255.)
	}

	/// Computes `A + B`.
	#[inline]
	pub fn add(pixa: Color, pixb: Color) -> Color {
		Color {
			a: pixa.a.saturating_add(pixb.a),
			r: pixa.r.saturating_add(pixb.r),
			g: pixa.g.saturating_add(pixb.g),
			b: pixa.b.saturating_add(pixb.b),
		}
	}

	/// Computes `A xor B`.
	pub fn xor(pixa: Color, pixb: Color) -> Color {
		pixa ^ pixb
	}

	/// Computes A.
	#[inline]
	pub fn dst(pixa: Color, _pixb: Color) -> Color {
		pixa
	}
}
