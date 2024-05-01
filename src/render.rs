use crate::ui::AsciiSheet;

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

	pub fn draw(&mut self, commands: &[DrawCommand], ascii_sheet: &AsciiSheet) {
		draw(commands, &mut self.fb_stack, &self.spritesheets, ascii_sheet);
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
	Text {
		text: String,
		pos: Pos,
		acf: AlphaCompFn,
	},
	BeginComposite,
	EndComposite(AlphaCompFn),
}

fn draw(commands: &[DrawCommand], fb_stack: &mut FramebufferStack, spritesheets: &[Bitmap], ascii_sheet: &AsciiSheet) {
	let ascii_bitmap = &spritesheets[ascii_sheet.space.id.0];

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
					if nssp.rect.w == 0 || nssp.rect.h == 0 {
						break 'top_left;
					}

					let nssp_pos = nssp.rect.pos();
					let nssp_size = nssp.rect.size();
					(fb_stack.fb_mut(fb_id)).copy_bitmap_area(bitmap, fb_pos, nssp_pos, nssp_size, acf);
				}

				'top_center: {
					let nssp = nss.slice(NineSlicePart::TopCenter);
					if nssp.rect.w == 0 || nssp.rect.h == 0 {
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
					if nssp.rect.w == 0 || nssp.rect.h == 0 {
						break 'top_right;
					}

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
					if nssp.rect.w == 0 || nssp.rect.h == 0 {
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
					if nssp.rect.w == 0 || nssp.rect.h == 0 {
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
					if nssp.rect.w == 0 || nssp.rect.h == 0 {
						break 'bottom_left;
					}

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
					if nssp.rect.w == 0 || nssp.rect.h == 0 {
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
					if nssp.rect.w == 0 || nssp.rect.h == 0 {
						break 'bottom_right;
					}

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
			DrawCommand::Text { pos, ref text, acf } => {
				let mut pos = pos;
				let fb = fb_stack.fb_mut(fb_id);

				for &c in text.as_bytes() {
					let c_sprite = match c {
						b' ' => ascii_sheet.space,

						b'A' => ascii_sheet.upper_a,
						b'B' => ascii_sheet.upper_b,
						b'C' => ascii_sheet.upper_c,
						b'D' => ascii_sheet.upper_d,
						b'E' => ascii_sheet.upper_e,
						b'F' => ascii_sheet.upper_f,
						b'G' => ascii_sheet.upper_g,
						b'H' => ascii_sheet.upper_h,
						b'I' => ascii_sheet.upper_i,
						b'J' => ascii_sheet.upper_j,
						b'K' => ascii_sheet.upper_k,
						b'L' => ascii_sheet.upper_l,
						b'M' => ascii_sheet.upper_m,
						b'N' => ascii_sheet.upper_n,
						b'O' => ascii_sheet.upper_o,
						b'P' => ascii_sheet.upper_p,
						b'Q' => ascii_sheet.upper_q,
						b'R' => ascii_sheet.upper_r,
						b'S' => ascii_sheet.upper_s,
						b'T' => ascii_sheet.upper_t,
						b'U' => ascii_sheet.upper_u,
						b'V' => ascii_sheet.upper_v,
						b'W' => ascii_sheet.upper_w,
						b'X' => ascii_sheet.upper_x,
						b'Y' => ascii_sheet.upper_y,
						b'Z' => ascii_sheet.upper_z,

						b'a' => ascii_sheet.lower_a,
						b'b' => ascii_sheet.lower_b,
						b'c' => ascii_sheet.lower_c,
						b'd' => ascii_sheet.lower_d,
						b'e' => ascii_sheet.lower_e,
						b'f' => ascii_sheet.lower_f,
						b'g' => ascii_sheet.lower_g,
						b'h' => ascii_sheet.lower_h,
						b'i' => ascii_sheet.lower_i,
						b'j' => ascii_sheet.lower_j,
						b'k' => ascii_sheet.lower_k,
						b'l' => ascii_sheet.lower_l,
						b'm' => ascii_sheet.lower_m,
						b'n' => ascii_sheet.lower_n,
						b'o' => ascii_sheet.lower_o,
						b'p' => ascii_sheet.lower_p,
						b'q' => ascii_sheet.lower_q,
						b'r' => ascii_sheet.lower_r,
						b's' => ascii_sheet.lower_s,
						b't' => ascii_sheet.lower_t,
						b'u' => ascii_sheet.lower_u,
						b'v' => ascii_sheet.lower_v,
						b'w' => ascii_sheet.lower_w,
						b'x' => ascii_sheet.lower_x,
						b'y' => ascii_sheet.lower_y,
						b'z' => ascii_sheet.lower_z,

						b'0' => ascii_sheet.digit_0,
						b'1' => ascii_sheet.digit_1,
						b'2' => ascii_sheet.digit_2,
						b'3' => ascii_sheet.digit_3,
						b'4' => ascii_sheet.digit_4,
						b'5' => ascii_sheet.digit_5,
						b'6' => ascii_sheet.digit_6,
						b'7' => ascii_sheet.digit_7,
						b'8' => ascii_sheet.digit_8,
						b'9' => ascii_sheet.digit_9,

						b'!' => ascii_sheet.exclamation_mark,
						b'?' => ascii_sheet.question_mark,
						b':' => ascii_sheet.colon,
						b';' => ascii_sheet.semicolon,
						b',' => ascii_sheet.comma,
						b'.' => ascii_sheet.period,
						b'*' => ascii_sheet.star,
						b'#' => ascii_sheet.hashtag,
						b'\'' => ascii_sheet.single_quote,
						b'"' => ascii_sheet.double_quote,
						b'[' => ascii_sheet.bracket_l,
						b']' => ascii_sheet.bracket_r,
						b'(' => ascii_sheet.parens_l,
						b')' => ascii_sheet.parens_r,
						b'{' => ascii_sheet.brace_l,
						b'}' => ascii_sheet.brace_r,
						b'<' => ascii_sheet.less_than,
						b'>' => ascii_sheet.greater_than,
						b'-' => ascii_sheet.minus,
						b'+' => ascii_sheet.plus,
						b'/' => ascii_sheet.slash,
						b'=' => ascii_sheet.equals,
						b'_' => ascii_sheet.underscore,

						_ => ascii_sheet.question_mark,
					};

					fb.copy_bitmap_area(ascii_bitmap, pos, c_sprite.rect.pos(), c_sprite.rect.size(), acf);
					pos.x += c_sprite.rect.w as i16 + 1;
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
