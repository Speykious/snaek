// TODO: remove this thing as soon as possible
#![allow(unused)]

use std::error::Error;
use std::time::Duration;

use crate::ui::WidgetFlags;

use self::math::pos::{pos, Pos};
use self::math::rect::Rect;
use self::math::size::size;
use self::render::bitmap::Bitmap;
use self::render::color::{alphacomp, Color};
use image::{ImageFormat, ImageResult};
use math::size::Size;
use minifb::{Key, MouseButton, MouseMode, Scale, ScaleMode, Window, WindowOptions};
use owo_colors::OwoColorize;
use render::{DrawCommand, Renderer, SpritesheetId};
use snake::SnaekSheet;
use ui::{Anchor, FlexDirection, Mouse, UiContext, WidgetDim, WidgetLayout, WidgetPadding, WidgetProps, WidgetSize};

mod math;
mod render;
mod snake;
mod ui;

const WIDTH: u16 = 97;
const HEIGHT: u16 = 124;

fn main() {
	eprintln!("{}", "Snaek!!".yellow());

	match game() {
		Ok(_) => eprintln!("{}", "See you next time :)".green()),
		Err(e) => {
			eprintln!("{}", "The game crashed! D:".red());
			eprintln!("-> {}", e);
		}
	}
}

#[derive(Debug, Clone)]
struct Bounce {
	pub pixel: Color,
	pub rect: Rect,
	pub dpos: Pos,
}

const IMG_ASCII_CHARS: &[u8] = include_bytes!("../assets/ascii-chars.png");
const IMG_SNAEKSHEET: &[u8] = include_bytes!("../assets/snaeksheet.png");

/// Loads a PNG from memory into a raw ARGB8 bitmap.
fn load_png_from_memory(png: &[u8]) -> ImageResult<Bitmap> {
	let img = image::load_from_memory_with_format(png, ImageFormat::Png)?;

	let size = size(img.width() as u16, img.height() as u16);

	let buffer = (img.into_rgba8().pixels())
		.map(|pixel| {
			let [r, g, b, a] = pixel.0;
			u32::from_le_bytes([b, g, r, a])
		})
		.collect::<Vec<u32>>();

	Ok(Bitmap::from_buffer(buffer, size))
}

const VIEWPORT_SIZE: Size = size(WIDTH, HEIGHT);

fn game() -> Result<(), Box<dyn Error>> {
	let ascii_bitmap = load_png_from_memory(IMG_ASCII_CHARS)?;
	let ascii_sheet = render::ascii_sheet();

	let mut renderer = Renderer::new(Bitmap::new(VIEWPORT_SIZE), ascii_bitmap);
	let mut ui = UiContext::new(VIEWPORT_SIZE);

	let snaek_sheet_id = renderer.register_spritesheet(load_png_from_memory(IMG_SNAEKSHEET)?);
	let snaek_sheet = snake::snaek_sheet();

	let options = WindowOptions {
		borderless: true,
		title: true,
		resize: false,
		scale: Scale::X4,
		scale_mode: ScaleMode::Stretch,
		..Default::default()
	};

	let mut window = Window::new("Snaek", WIDTH as usize, HEIGHT as usize, options)?;
	window.limit_update_rate(Some(Duration::from_micros(1_000_000 / 60)));

	let center = pos(WIDTH as i16 / 2, HEIGHT as i16 / 2);
	let bounce_size = size(30, 20);

	let mut bounces = [
		Bounce {
			pixel: Color::from_hex(0xff801234),
			rect: Rect::from_pos_size(center + pos(-8, -10), bounce_size),
			dpos: pos(-1, -1),
		},
		Bounce {
			pixel: Color::from_hex(0x80128034),
			rect: Rect::from_pos_size(center + pos(9, -13), bounce_size),
			dpos: pos(1, -1),
		},
		Bounce {
			pixel: Color::from_hex(0xff123480),
			rect: Rect::from_pos_size(center + pos(11, 12), bounce_size),
			dpos: pos(1, 1),
		},
	];

	let mut draw_cmds = Vec::new();
	let mut mouse = Mouse::default();
	while window.is_open() {
		// input handling
		if window.is_key_down(Key::Escape) {
			break;
		}

		if let Some(next_pos) = window.get_mouse_pos(MouseMode::Discard) {
			mouse.x = next_pos.0;
			mouse.y = next_pos.1;
		}

		mouse.left_pressed = window.get_mouse_down(MouseButton::Left);
		mouse.right_pressed = window.get_mouse_down(MouseButton::Right);
		mouse.middle_pressed = window.get_mouse_down(MouseButton::Middle);

		// state update
		for bounce in &mut bounces {
			if bounce.rect.x <= 0 {
				bounce.dpos.x = 1;
			} else if bounce.rect.x as i32 + bounce.rect.w as i32 + 1 > WIDTH as i32 {
				bounce.dpos.x = -1;
			}

			if bounce.rect.y <= 0 {
				bounce.dpos.y = 1;
			} else if bounce.rect.y as i32 + bounce.rect.h as i32 + 1 > HEIGHT as i32 {
				bounce.dpos.y = -1;
			}

			bounce.rect.x += bounce.dpos.x;
			bounce.rect.y += bounce.dpos.y;
		}

		draw_cmds.clear();

		draw_cmds.push(DrawCommand::Clear);
		draw_cmds.push(DrawCommand::Fill {
			rect: renderer.rect(),
			color: Color::from_hex(0xff262b44),
			acf: alphacomp::dst,
		});

		draw_cmds.push(DrawCommand::Stroke {
			rect: renderer.rect(),
			stroke_width: 1,
			color: Color::from_hex(0xff181425),
			acf: alphacomp::dst,
		});

		draw_rectangles_bouncing(&renderer, &bounces, snaek_sheet_id, &snaek_sheet, &mut draw_cmds);

		// UI
		{
			let (frame_id, _) = ui.build_widget(WidgetProps {
				key: wk!(),
				anchor: Anchor::CENTER,
				origin: Anchor::CENTER,
				size: WidgetSize {
					w: WidgetDim::Fixed(30),
					h: WidgetDim::Fill,
				},
				padding: WidgetPadding::all(2),
				layout: WidgetLayout::Flex {
					direction: FlexDirection::Vertical,
					gap: 2,
				},
				..WidgetProps::default()
			});
			{
				let (ewe_button_id, ewe_button) = ui.button(
					wk!(),
					renderer.text("ewe"),
					WidgetSize {
						w: WidgetDim::Fill,
						h: WidgetDim::Fill,
					},
					(snaek_sheet_id, snaek_sheet.box_embossed),
					(snaek_sheet_id, snaek_sheet.box_carved),
				);
				ui.add_child(frame_id, ewe_button_id);

				if ewe_button.clicked {
					println!("ewe");
				}

				for i in 0..3 {
					let (uwu_button_id, uwu_button) = ui.button(
						wk!(i),
						renderer.text("UwU"),
						WidgetSize {
							w: WidgetDim::Fill,
							h: WidgetDim::Fixed(9),
						},
						(snaek_sheet_id, snaek_sheet.box_embossed),
						(snaek_sheet_id, snaek_sheet.box_carved),
					);
					ui.add_child(frame_id, uwu_button_id);

					if uwu_button.clicked {
						println!("UwU ({})", i);
					}

					let (owo_button_id, owo_button) = ui.button(
						wk!(i),
						renderer.text("OwO"),
						WidgetSize {
							w: WidgetDim::Fill,
							h: WidgetDim::Fixed(9),
						},
						(snaek_sheet_id, snaek_sheet.box_embossed),
						(snaek_sheet_id, snaek_sheet.box_carved),
					);
					ui.add_child(frame_id, owo_button_id);

					if owo_button.clicked {
						println!("OwO ({})", i);
					}
				}
			}
		}
		ui.solve_layout();
		ui.draw_widgets(&mut draw_cmds);
		ui.free_untouched_widgets();
		ui.react(&mouse);

		renderer.draw(&draw_cmds);

		window
			.update_with_buffer(renderer.first_framebuffer().pixels(), WIDTH as usize, HEIGHT as usize)
			.unwrap();
	}

	Ok(())
}

fn draw_rectangles_bouncing(
	renderer: &Renderer,
	bounces: &[Bounce],
	snaek_sheet_id: SpritesheetId,
	snaek_sheet: &SnaekSheet,
	draw_cmds: &mut Vec<DrawCommand>,
) {
	draw_cmds.push(DrawCommand::BeginComposite);
	{
		draw_cmds.push(DrawCommand::Fill {
			rect: renderer.rect(),
			color: Color::from_hex(0x10000000),
			acf: alphacomp::over,
		});

		for bounce in bounces {
			draw_cmds.push(DrawCommand::Fill {
				rect: bounce.rect,
				color: bounce.pixel,
				acf: alphacomp::add,
			});
		}

		draw_cmds.push(DrawCommand::BeginComposite);
		{
			draw_cmds.push(DrawCommand::Clear);
			draw_cmds.push(DrawCommand::Sprite {
				pos: Pos::ZERO,
				sheet_id: snaek_sheet_id,
				sprite: snaek_sheet.snaek_icon,
				acf: alphacomp::over,
			});

			draw_cmds.push(DrawCommand::Text {
				pos: pos(12, 12),
				text: "Hello, world!".into(),
				acf: alphacomp::over,
			});
		}
		draw_cmds.push(DrawCommand::EndComposite(alphacomp::over));
	}
	draw_cmds.push(DrawCommand::EndComposite(alphacomp::over));
}
