use crate::ui::Anchor;

use self::pos::Pos;
use self::rect::Rect;

pub mod pos;
pub mod rect;
pub mod size;

#[derive(Debug, Clone, Default)]
pub struct LayoutRect {
	pub rect: Rect,
	pub origin: Anchor,
}

impl LayoutRect {
	pub const fn new(rect: Rect, origin: Anchor) -> Self {
		Self { rect, origin }
	}

	pub fn top_left(&self) -> Pos {
		Pos {
			x: (self.rect.x as f32 - self.rect.w as f32 * self.origin.x) as i16,
			y: (self.rect.y as f32 - self.rect.h as f32 * self.origin.y) as i16,
		}
	}

	pub fn center_of_rect(&self) -> Pos {
		let pos = self.top_left();
		Pos {
			x: pos.x + (self.rect.w / 2) as i16,
			y: pos.y + (self.rect.h / 2) as i16,
		}
	}

	pub fn anchor(&self, anchor: Anchor) -> Pos {
		let pos = self.top_left();
		Pos {
			x: pos.x + (self.rect.w as f32 * anchor.x) as i16,
			y: pos.y + (self.rect.h as f32 * anchor.y) as i16,
		}
	}

	pub fn to_rect(&self) -> Rect {
		let tl = self.top_left();

		Rect {
			x: tl.x,
			y: tl.y,
			w: self.rect.w,
			h: self.rect.h,
		}
	}
}
