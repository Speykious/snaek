use super::pos::Pos;
use super::size::Size;

/// A rectangle with a position and size.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C, align(8))]
pub struct Rect {
	pub x: i16,
	pub y: i16,
	pub w: u16,
	pub h: u16,
}

impl Rect {
	#[inline]
	pub fn from_xywh(x: i16, y: i16, w: u16, h: u16) -> Self {
		Self { x, y, w, h }
	}

	#[inline]
	pub fn from_ab(pos_a: Pos, pos_b: Pos) -> Self {
		let xa = pos_a.x.min(pos_b.x);
		let ya = pos_a.y.min(pos_b.y);
		let xb = pos_a.x.max(pos_b.x);
		let yb = pos_a.y.max(pos_b.y);
		Self::from_xywh(xa, ya, (xb - xa) as u16, (yb - ya) as u16)
	}

	#[inline]
	pub fn from_pos_size(pos: Pos, size: Size) -> Self {
		Self {
			x: pos.x,
			y: pos.y,
			w: size.w,
			h: size.h,
		}
	}

	#[inline]
	pub fn pos(&self) -> Pos {
		Pos { x: self.x, y: self.y }
	}

	#[inline]
	pub fn size(&self) -> Size {
		Size { w: self.w, h: self.h }
	}
}
