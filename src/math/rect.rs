use super::pos::Pos;
use super::size::Size;

/// A rectangle with a position and size.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(C, align(8))]
pub struct Rect {
	pub x: i16,
	pub y: i16,
	pub w: u16,
	pub h: u16,
}

impl Rect {
	pub const ZERO: Self = Self::from_xywh(0, 0, 0, 0);

	#[inline]
	pub const fn from_xywh(x: i16, y: i16, w: u16, h: u16) -> Self {
		Self { x, y, w, h }
	}

	#[inline]
	pub const fn from_pos_size(pos: Pos, size: Size) -> Self {
		Self {
			x: pos.x,
			y: pos.y,
			w: size.w,
			h: size.h,
		}
	}

	#[inline]
	pub const fn pos(&self) -> Pos {
		Pos { x: self.x, y: self.y }
	}

	#[inline]
	pub const fn size(&self) -> Size {
		Size { w: self.w, h: self.h }
	}

	pub fn contains(&self, px: f64, py: f64) -> bool {
		((self.x as f64) <= px && px < (self.x as f64 + self.w as f64))
			&& ((self.y as f64) <= py && py < (self.y as f64 + self.h as f64))
	}
}
