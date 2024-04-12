use std::cell::RefCell;
use std::marker::PhantomData;

use self::bitmap::Bitmap;
use self::pixel::alphacomp::{self, AlphaCompFn};
use self::pixel::Pixel;
use self::sprite::{NineSlicePart, NineSlicingSprite, Sprite};
use super::math::pos::{pos, Pos};
use super::math::rect::Rect;
use super::math::size::{size, Size};

pub mod bitmap;
pub mod pixel;
pub mod sprite;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SpritesheetId(usize);

struct FramebufferStack {
	size: Size,
	fbs: Vec<Bitmap>,
}

impl FramebufferStack {
	pub fn new(framebuffer: Bitmap) -> Self {
		Self {
			size: framebuffer.size(),
			fbs: vec![framebuffer],
		}
	}

	pub fn blit_fb_down(&mut self, zindex: usize, acf: AlphaCompFn) {
		if zindex >= self.fbs.len() || zindex == 0 {
			return;
		}

		// this is probably a hack but it's a good way to get two mutable elements from a vec
		let [fba, fbb] = &mut self.fbs[(zindex - 1)..=zindex] else {
			return;
		};

		fba.copy_bitmap(fbb, acf);
	}

	fn push_fbs(&mut self, zindex: usize) {
		while zindex >= self.fbs.len() {
			self.fbs.push(Bitmap::new(self.size));
		}
	}

	fn fb_mut(&mut self, zindex: usize) -> &mut Bitmap {
		self.push_fbs(zindex);
		&mut self.fbs[zindex]
	}

	fn fb(&mut self, zindex: usize) -> &Bitmap {
		self.push_fbs(zindex);
		&self.fbs[zindex]
	}
}

pub struct Renderer {
	fb_stack: FramebufferStack,
	spritesheets: Vec<Bitmap>,
}

impl Renderer {
	pub fn new(framebuffer: Bitmap) -> Self {
		Self {
			fb_stack: FramebufferStack::new(framebuffer),
			spritesheets: Vec::new(),
		}
	}

	pub fn register_spritesheet(&mut self, sheet: Bitmap) -> SpritesheetId {
		let id = SpritesheetId(self.spritesheets.len());
		self.spritesheets.push(sheet);
		id
	}

	pub fn first_framebuffer(&mut self) -> &Bitmap {
		self.fb_stack.fb(0)
	}

	pub fn draw(&mut self, commands: &[DrawCommand]) {
		draw(commands, &mut self.fb_stack, &self.spritesheets);
	}

	pub fn size(&self) -> Size {
		self.fb_stack.size
	}

	pub fn rect(&self) -> Rect {
		Rect::from_pos_size(Pos::ZERO, self.size())
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum DrawCommand {
	Clear,
	Fill {
		rect: Rect,
		color: Pixel,
		acf: AlphaCompFn,
	},
	Stroke {
		rect: Rect,
		stroke_width: u16,
		color: Pixel,
		acf: AlphaCompFn,
	},
	Sprite {
		pos: Pos,
		sprite: Sprite,
		acf: AlphaCompFn,
	},
	NineSlicingSprite {
		rect: Rect,
		nss: NineSlicingSprite,
		acf: AlphaCompFn,
	},
	BeginComposite,
	EndComposite(AlphaCompFn),
}

fn draw(commands: &[DrawCommand], fb_stack: &mut FramebufferStack, spritesheets: &[Bitmap]) {
	let mut fb_id = 0;
	for command in commands {
		match *command {
			DrawCommand::Clear => (fb_stack.fb_mut(fb_id)).fill(Pixel::ZERO, alphacomp::dst),
			DrawCommand::Fill { rect, color, acf } => (fb_stack.fb_mut(fb_id)).fill_area(color, rect, acf),
			DrawCommand::Stroke {
				rect,
				stroke_width,
				color,
				acf,
			} => {
				let hsize = size(rect.w, stroke_width);
				let vsize = size(stroke_width, rect.h - 2 * stroke_width);
				let lry = rect.y + stroke_width as i16;

				let top_pos = rect.pos();
				let top_rect = Rect::from_pos_size(top_pos, hsize);
				(fb_stack.fb_mut(fb_id)).fill_area(color, top_rect, acf);

				let left_pos = pos(rect.x, lry);
				let left_rect = Rect::from_pos_size(left_pos, vsize);
				(fb_stack.fb_mut(fb_id)).fill_area(color, left_rect, acf);

				let bottom_pos = pos(rect.x, rect.y + (rect.h - stroke_width) as i16);
				let bottom_rect = Rect::from_pos_size(bottom_pos, hsize);
				(fb_stack.fb_mut(fb_id)).fill_area(color, bottom_rect, acf);

				let right_pos = pos(rect.x + (rect.w - stroke_width) as i16, lry);
				let right_rect = Rect::from_pos_size(right_pos, vsize);
				(fb_stack.fb_mut(fb_id)).fill_area(color, right_rect, acf);
			}
			DrawCommand::Sprite { pos, sprite, acf } => {
				let Some(bitmap) = spritesheets.get(sprite.id.0) else {
					continue;
				};

				(fb_stack.fb_mut(fb_id)).copy_bitmap_area(bitmap, pos, sprite.rect.pos(), sprite.rect.size(), acf);
			}
			DrawCommand::NineSlicingSprite { rect, nss, acf } => {
				if rect.w == 0 || rect.h == 0 {
					continue;
				}

				let Some(bitmap) = spritesheets.get(nss.sprite.id.0) else {
					continue;
				};

				let fb_pos = rect.pos();

				'top_left: {
					let nssp = nss.slice(NineSlicePart::TopLeft);
					let nssp_pos = nssp.rect.pos();
					let nssp_size = nssp.rect.size();
					(fb_stack.fb_mut(fb_id)).copy_bitmap_area(bitmap, fb_pos, nssp_pos, nssp_size, acf);
				}

				'top_center: {
					let nssp = nss.slice(NineSlicePart::TopCenter);
					if nssp.rect.w == 0 {
						break 'top_center;
					}

					let mut x = nss.vl;
					while x < rect.w.saturating_sub(nssp.rect.w) {
						let nssp_pos = nssp.rect.pos();
						let nssp_size = nssp.rect.size();
						(fb_stack.fb_mut(fb_id)).copy_bitmap_area(
							bitmap,
							pos(fb_pos.x + x as i16, fb_pos.y),
							nssp_pos,
							size(nssp_size.w.min((rect.w - nssp.rect.w) - x), nssp_size.h),
							acf,
						);
						x += nssp.rect.w;
					}
				}

				'top_right: {
					let nssp = nss.slice(NineSlicePart::TopRight);
					let nssp_pos = nssp.rect.pos();
					let nssp_size = nssp.rect.size();
					(fb_stack.fb_mut(fb_id)).copy_bitmap_area(
						bitmap,
						pos(fb_pos.x + (rect.w - nssp.rect.w) as i16, fb_pos.y),
						nssp_pos,
						nssp_size,
						acf,
					);
				}

				'center_left: {
					let nssp = nss.slice(NineSlicePart::CenterLeft);
					if nssp.rect.h == 0 {
						break 'center_left;
					}

					let mut y = nss.ht;
					while y < rect.h.saturating_sub(nssp.rect.h) {
						let nssp_pos = nssp.rect.pos();
						let nssp_size = nssp.rect.size();
						(fb_stack.fb_mut(fb_id)).copy_bitmap_area(
							bitmap,
							pos(fb_pos.x, fb_pos.y + y as i16),
							nssp_pos,
							size(nssp_size.w, nssp_size.h.min((rect.h - nssp.rect.h) - y)),
							acf,
						);
						y += nssp.rect.h;
					}
				}

				'center: {
					let nssp = nss.slice(NineSlicePart::Center);
					if nssp.rect.w == 0 || nssp.rect.h == 0 {
						break 'center;
					}

					let mut y = nss.ht;
					while y < rect.h.saturating_sub(nssp.rect.h) {
						let mut x = nss.vl;
						while x < rect.w.saturating_sub(nssp.rect.w) {
							let nssp_pos = nssp.rect.pos();
							let nssp_size = nssp.rect.size();
							(fb_stack.fb_mut(fb_id)).copy_bitmap_area(
								bitmap,
								pos(fb_pos.x + x as i16, fb_pos.y + y as i16),
								nssp_pos,
								size(
									nssp_size.w.min((rect.w - nssp.rect.w) - x),
									nssp_size.h.min((rect.h - nssp.rect.h) - y),
								),
								acf,
							);
							x += nssp.rect.w;
						}
						y += nssp.rect.h;
					}
				}

				'center_right: {
					let nssp = nss.slice(NineSlicePart::CenterRight);
					if nssp.rect.h == 0 {
						break 'center_right;
					}

					let mut y = nss.ht;
					while y < rect.h.saturating_sub(nssp.rect.h) {
						let nssp_pos = nssp.rect.pos();
						let nssp_size = nssp.rect.size();
						(fb_stack.fb_mut(fb_id)).copy_bitmap_area(
							bitmap,
							pos(fb_pos.x + (rect.w - nssp.rect.w) as i16, fb_pos.y + y as i16),
							nssp_pos,
							size(nssp_size.w, nssp_size.h.min((rect.h - nssp.rect.h) - y)),
							acf,
						);
						y += nssp.rect.h;
					}
				}

				'bottom_left: {
					let nssp = nss.slice(NineSlicePart::BottomLeft);
					let nssp_pos = nssp.rect.pos();
					let nssp_size = nssp.rect.size();
					(fb_stack.fb_mut(fb_id)).copy_bitmap_area(
						bitmap,
						pos(fb_pos.x, fb_pos.y + (rect.h - nssp.rect.h) as i16),
						nssp_pos,
						nssp_size,
						acf,
					);
				}

				'bottom_center: {
					let nssp = nss.slice(NineSlicePart::BottomCenter);
					if nssp.rect.w == 0 {
						break 'bottom_center;
					}

					let mut x = nss.vl;
					while x < rect.w.saturating_sub(nssp.rect.w) {
						let nssp_pos = nssp.rect.pos();
						let nssp_size = nssp.rect.size();
						(fb_stack.fb_mut(fb_id)).copy_bitmap_area(
							bitmap,
							pos(fb_pos.x + x as i16, fb_pos.y + (rect.h - nssp.rect.h) as i16),
							nssp_pos,
							size(nssp_size.w.min((rect.w - nssp.rect.w) - x), nssp_size.h),
							acf,
						);
						x += nssp.rect.w;
					}
				}

				'bottom_right: {
					let nssp = nss.slice(NineSlicePart::BottomRight);
					let nssp_pos = nssp.rect.pos();
					let nssp_size = nssp.rect.size();
					(fb_stack.fb_mut(fb_id)).copy_bitmap_area(
						bitmap,
						pos(
							fb_pos.x + (rect.w - nssp.rect.w) as i16,
							fb_pos.y + (rect.h - nssp.rect.h) as i16,
						),
						nssp_pos,
						nssp_size,
						acf,
					);
				}
			}
			DrawCommand::BeginComposite => {
				fb_id += 1;
			}
			DrawCommand::EndComposite(acf) => {
				fb_stack.blit_fb_down(fb_id, acf);
				fb_id -= 1;
			}
		}
	}
}
