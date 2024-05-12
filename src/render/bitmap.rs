use super::color::alphacomp::AlphaCompFn;
use super::color::Color;
use super::{Pos, Rect, Size};
use crate::math::pos::pos;

/// RGBA bitmap.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub struct Bitmap {
	buffer: Vec<u32>,
	size: Size,
}

impl Bitmap {
	pub fn from_buffer(buffer: Vec<u32>, size: Size) -> Self {
		Self { buffer, size }
	}

	#[inline]
	pub fn new(size: Size) -> Self {
		Self {
			buffer: vec![0; size.w as usize * size.h as usize],
			size,
		}
	}

	/// Resizes the bitmap with `size` as the size.
	/// This completely resets the data.
	pub fn resize(&mut self, size: Size) {
		self.buffer = vec![0; size.w as usize * size.h as usize];
		self.size = size;
	}

	#[inline]
	pub fn pixels(&self) -> &[u32] {
		&self.buffer
	}

	fn line_indices(&self, pos: Pos, width: u16) -> (usize, usize) {
		let start_x = (self.size.w as usize).min(pos.x as usize);
		let end_x = (self.size.w as usize).min(width as usize + start_x);
		let width = end_x - start_x;

		let start = self.index(pos);
		(start, start + width)
	}

	/// Gives a full horizontal line of pixels
	pub fn line(&self, pos: Pos, width: u16) -> &[u32] {
		let (start, end) = self.line_indices(pos, width);
		&self.buffer[start..end]
	}

	/// Gives a full horizontal line of pixels (mutable)
	pub fn line_mut(&mut self, pos: Pos, width: u16) -> &mut [u32] {
		let (start, end) = self.line_indices(pos, width);
		&mut self.buffer[start..end]
	}

	/// A function that looks like this:
	///
	/// ```ignore
	/// #[inline]
	/// pub fn size(&self) -> Size {
	///     self.size
	/// }
	/// ```
	#[inline]
	pub fn size(&self) -> Size {
		self.size
	}

	pub fn copy_bitmap(&mut self, other: &Bitmap, acf: AlphaCompFn) {
		for (px, other_px) in self.buffer.iter_mut().zip(other.buffer.iter()) {
			*px = (acf)(Color::from_hex(*other_px), Color::from_hex(*px)).to_u32();
		}
	}

	pub fn copy_bitmap_area(&mut self, other: &Bitmap, this_pos: Pos, other_pos: Pos, size: Size, acf: AlphaCompFn, mask_and: Color, mask_or: Color) {
		let this_cropped_rect = self.crop_rect(Rect::from_pos_size(this_pos, size));
		let this_pos = this_cropped_rect.pos();

		if (this_pos.x as i32) >= (self.size.w as i32) || (this_pos.y as i32) >= (self.size.h as i32) {
			return;
		}

		if (this_pos.x as i32) + (size.w as i32) < 0 || (this_pos.y as i32) + (size.h as i32) < 0 {
			return;
		}

		for y in 0..(size.h as i32).min(self.size.h as i32 - this_pos.y as i32) as i16 {
			let this_line = self.line_mut(pos(this_pos.x, this_pos.y + y), size.w);
			let other_line = other.line(pos(other_pos.x, other_pos.y + y), size.w);
			for (this_px, other_px) in this_line.iter_mut().zip(other_line.iter()) {
				let other_color = (Color::from_hex(*other_px) & mask_and) | mask_or;
				let c = (acf)(other_color, Color::from_hex(*this_px));
				*this_px = c.to_u32();
			}
		}
	}

	pub fn fill(&mut self, color: Color, acf: AlphaCompFn) {
		for px in &mut self.buffer {
			*px = (acf)(color, Color::from_hex(*px)).to_u32();
		}
	}

	pub fn fill_area(&mut self, color: Color, rect: Rect, acf: AlphaCompFn) {
		let rect = self.crop_rect(rect);
		if rect.w == 0 || rect.h == 0 {
			return;
		}

		for y in 0..rect.h as i16 {
			for px in self.line_mut(pos(rect.x, rect.y + y), rect.w) {
				*px = (acf)(color, Color::from_hex(*px)).to_u32();
			}
		}
	}

	fn crop_rect(&self, mut rect: Rect) -> Rect {
		if rect.x < 0 {
			rect.w = rect.w.saturating_add_signed(rect.x);
			rect.x = 0;
		}

		if rect.y < 0 {
			rect.h = rect.h.saturating_add_signed(rect.y);
			rect.y = 0;
		}

		let difference = (rect.x as i32 + rect.w as i32 - self.size.w as i32) as i16;
		if difference > 0 {
			rect.w = rect.w.saturating_add_signed(-difference);
		}

		let difference = (rect.y as i32 + rect.h as i32 - self.size.h as i32) as i16;
		if difference > 0 {
			rect.h = rect.h.saturating_add_signed(-difference);
		}

		rect
	}

	/// Converts a position to the index of a pixel on the bitmap.
	fn index(&self, pos: Pos) -> usize {
		debug_assert!(pos.x >= 0 && pos.y >= 0, "Position has a negative coordinate");
		debug_assert!((pos.x as i32) < (self.size.w as i32), "Position exceeds bitmap width");
		debug_assert!((pos.y as i32) < (self.size.h as i32), "Position exceeds bitmap height");

		pos.y as usize * self.size.w as usize + pos.x as usize
	}
}
