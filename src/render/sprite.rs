use std::ops::{Deref, DerefMut};

use crate::math::rect::Rect;

use super::SpritesheetId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Sprite {
	/// Describes where the sprite is located in the spritesheet.
	pub rect: Rect,
}

impl Sprite {
	#[inline]
	pub fn new(rect: Rect) -> Self {
		Self { rect }
	}
}

impl Deref for Sprite {
	type Target = Rect;

	fn deref(&self) -> &Self::Target {
		&self.rect
	}
}

impl DerefMut for Sprite {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.rect
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum NineSlicePart {
	TopLeft,
	TopCenter,
	TopRight,
	CenterLeft,
	Center,
	CenterRight,
	BottomLeft,
	BottomCenter,
	BottomRight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NineSlicingSprite {
	pub sprite: Sprite,

	/// position of the left vertical bar (from top left corner)
	pub vl: u16,
	/// position of the right vertical bar (from top left corner)
	pub vr: u16,
	/// position of the top horizontal bar (from top left corner)
	pub ht: u16,
	/// position of the bottom horizontal bar (from top left corner)
	pub hb: u16,
}

impl NineSlicingSprite {
	#[inline]
	pub fn new(rect: Rect, vl: u16, vr: u16, ht: u16, hb: u16) -> Self {
		Self {
			sprite: Sprite::new(rect),
			vl,
			vr,
			ht,
			hb,
		}
	}

	pub fn slice(&self, part: NineSlicePart) -> Sprite {
		match part {
			NineSlicePart::TopLeft => {
				let x = self.sprite.x;
				let y = self.sprite.y;
				let w = self.vl;
				let h = self.ht;
				Sprite::new(Rect::from_xywh(x, y, w, h))
			}
			NineSlicePart::TopCenter => {
				let x = self.sprite.x + self.vl as i16;
				let y = self.sprite.y;
				let w = self.vr - self.vl;
				let h = self.ht;
				Sprite::new(Rect::from_xywh(x, y, w, h))
			}
			NineSlicePart::TopRight => {
				let x = self.sprite.x + self.vr as i16;
				let y = self.sprite.y;
				let w = self.sprite.w - self.vr;
				let h = self.ht;
				Sprite::new(Rect::from_xywh(x, y, w, h))
			}
			NineSlicePart::CenterLeft => {
				let x = self.sprite.x;
				let y = self.sprite.y + self.ht as i16;
				let w = self.vl;
				let h = self.hb - self.ht;
				Sprite::new(Rect::from_xywh(x, y, w, h))
			}
			NineSlicePart::Center => {
				let x = self.sprite.x + self.vl as i16;
				let y = self.sprite.y + self.ht as i16;
				let w = self.vr - self.vl;
				let h = self.hb - self.ht;
				Sprite::new(Rect::from_xywh(x, y, w, h))
			}
			NineSlicePart::CenterRight => {
				let x = self.sprite.x + self.vr as i16;
				let y = self.sprite.y + self.ht as i16;
				let w = self.sprite.w - self.vr;
				let h = self.hb - self.ht;
				Sprite::new(Rect::from_xywh(x, y, w, h))
			}
			NineSlicePart::BottomLeft => {
				let x = self.sprite.x;
				let y = self.sprite.y + self.hb as i16;
				let w = self.vl;
				let h = self.sprite.h - self.hb;
				Sprite::new(Rect::from_xywh(x, y, w, h))
			}
			NineSlicePart::BottomCenter => {
				let x = self.sprite.x + self.vl as i16;
				let y = self.sprite.y + self.hb as i16;
				let w = self.vr - self.vl;
				let h = self.sprite.h - self.hb;
				Sprite::new(Rect::from_xywh(x, y, w, h))
			}
			NineSlicePart::BottomRight => {
				let x = self.sprite.x + self.vr as i16;
				let y = self.sprite.y + self.hb as i16;
				let w = self.sprite.w - self.vr;
				let h = self.sprite.h - self.hb;
				Sprite::new(Rect::from_xywh(x, y, w, h))
			}
		}
	}
}
