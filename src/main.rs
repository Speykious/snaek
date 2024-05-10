#![allow(clippy::too_many_arguments)]
#![allow(unused)] // TODO: remove this thing as soon as possible

use std::error::Error;
use std::time::Duration;

use self::math::pos::pos;
use self::math::size::size;
use self::render::bitmap::Bitmap;
use self::render::color::{alphacomp, Color};
use image::{ImageFormat, ImageResult};
use math::size::Size;
use minifb::{CursorStyle, Key, MouseButton, MouseMode, Scale, ScaleMode, Window, WindowOptions};
use owo_colors::OwoColorize;
use render::{DrawCommand, Renderer};
use ui::{
	Anchor, FlexDirection, Mouse, UiContext, WidgetDim, WidgetFlags, WidgetLayout, WidgetPadding, WidgetProps,
	WidgetSize, WidgetSprite,
};

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

	let mut bananas_count = 23;

	let mut draw_cmds = Vec::new();
	let mut mouse = Mouse::default();
	let mut unscaled_mouse_pos = None;
	'game_loop: while window.is_open() {
		// input handling
		if window.is_key_down(Key::Escape) {
			break;
		}

		if let Some(next_pos) = window.get_mouse_pos(MouseMode::Discard) {
			mouse.x = next_pos.0;
			mouse.y = next_pos.1;
		}

		mouse.l_pressed = (window.get_mouse_down(MouseButton::Left), mouse.l_pressed.0);
		mouse.r_pressed = (window.get_mouse_down(MouseButton::Right), mouse.r_pressed.0);
		mouse.m_pressed = (window.get_mouse_down(MouseButton::Middle), mouse.m_pressed.0);

		draw_cmds.clear();
		draw_cmds.push(DrawCommand::Clear);

		// UI
		let window_frame = ui.build_widget(WidgetProps {
			key: wk!(),
			flags: WidgetFlags::DRAW_BACKGROUND | WidgetFlags::DRAW_BORDER,
			color: Color::from_hex(0xffc0cbdc),
			border_color: Color::from_hex(0xff181425),
			border_width: 1,
			acf: Some(alphacomp::dst),
			size: WidgetSize {
				w: WidgetDim::Fill,
				h: WidgetDim::Fill,
			},
			padding: WidgetPadding::all(1),
			layout: WidgetLayout::Flex {
				direction: FlexDirection::Vertical,
				gap: 0,
			},
			..WidgetProps::default()
		});
		{
			let navbar = ui.build_widget(WidgetProps {
				key: wk!(),
				flags: WidgetFlags::CAN_CLICK,
				size: WidgetSize {
					w: WidgetDim::Fill,
					h: WidgetDim::Fixed(8),
				},
				layout: WidgetLayout::Flex {
					direction: FlexDirection::Horizontal,
					gap: 0,
				},
				..WidgetProps::default()
			});
			{
				let snaek_icon = ui.build_widget(WidgetProps {
					key: wk!(),
					flags: WidgetFlags::DRAW_SPRITE,
					size: WidgetSize::fixed(8, 8),
					sprite: Some(WidgetSprite::Simple(snaek_sheet_id, snaek_sheet.snaek_icon)),
					draw_offset: pos(1, 1),
					layout: WidgetLayout::Flex {
						direction: FlexDirection::Horizontal,
						gap: 0,
					},
					..WidgetProps::default()
				});
				ui.add_child(navbar.id(), snaek_icon.id());

				let menu = ui.build_widget(WidgetProps {
					key: wk!(),
					size: WidgetSize::fill(),
					..WidgetProps::default()
				});
				ui.add_child(navbar.id(), menu.id());

				// // minifb cannot minimize the window so here we are...
				//
				// let btn_minimize = ui.btn_icon(
				// 	wk!(),
				// 	snaek_sheet_id,
				// 	snaek_sheet.icon_minimize,
				// 	WidgetSize::fixed(7, 7),
				// 	Anchor::TOP_LEFT,
				// 	Anchor::TOP_LEFT,
				// 	Color::from_hex(0x40181425),
				// );
				// ui.add_child(navbar.id(), btn_minimize.id());

				let btn_close = ui.btn_icon(
					wk!(),
					snaek_sheet_id,
					snaek_sheet.icon_close,
					WidgetSize::fixed(7, 7),
					Anchor::TOP_LEFT,
					Anchor::TOP_LEFT,
					Color::from_hex(0xffe43b44),
				);
				ui.add_child(navbar.id(), btn_close.id());

				if btn_close.clicked() {
					break 'game_loop;
				}
			}
			ui.add_child(window_frame.id(), navbar.id());

			if navbar.pressed() {
				let (cpx, cpy) = window.get_unscaled_mouse_pos(MouseMode::Pass).unwrap_or_default();
				let (mpx, mpy) = unscaled_mouse_pos.unwrap_or((cpx, cpy));

				let (wpx, wpy) = window.get_position();
				window.set_position(wpx + (cpx - mpx).round() as isize, wpy + (cpy - mpy).round() as isize);

				unscaled_mouse_pos = Some((mpx, mpy));
			} else {
				unscaled_mouse_pos = None;
			}

			let game_frame = ui.build_widget(WidgetProps {
				key: wk!(),
				flags: WidgetFlags::DRAW_SPRITE,
				sprite: Some(WidgetSprite::NineSlice(snaek_sheet_id, snaek_sheet.box_embossed)),
				acf: Some(alphacomp::dst),
				size: WidgetSize::fill(),
				padding: WidgetPadding::hv(4, 3),
				layout: WidgetLayout::Flex {
					direction: FlexDirection::Vertical,
					gap: 2,
				},
				..WidgetProps::default()
			});
			{
				let display_frame = ui.build_widget(WidgetProps {
					key: wk!(),
					size: WidgetSize {
						w: WidgetDim::Fill,
						h: WidgetDim::Hug,
					},
					layout: WidgetLayout::Flex {
						direction: FlexDirection::Horizontal,
						gap: 2,
					},
					..WidgetProps::default()
				});
				{
					let big_display = ui.big_3digits_display(
						wk!(),
						bananas_count,
						snaek_sheet_id,
						snaek_sheet.box_num_display,
						snaek_sheet.bignum_placeholder,
						&snaek_sheet.bignums,
					);
					ui.add_child(display_frame.id(), big_display.id());

					let middle_frame = ui.build_widget(WidgetProps {
						key: wk!(),
						size: WidgetSize::hug(),
						layout: WidgetLayout::Flex {
							direction: FlexDirection::Vertical,
							gap: 2,
						},
						..WidgetProps::default()
					});
					{
						let icon_restart = ui.sprite(
							wk!(),
							snaek_sheet_id,
							snaek_sheet.icon_restart,
							Anchor::CENTER,
							Anchor::CENTER,
							Some(alphacomp::xor),
						);
						let btn_restart = ui.btn_box(
							wk!(),
							WidgetSize::hug(),
							WidgetPadding::hv(3, 2),
							WidgetSprite::NineSlice(snaek_sheet_id, snaek_sheet.box_embossed),
							WidgetSprite::NineSlice(snaek_sheet_id, snaek_sheet.box_carved),
							Anchor::TOP_LEFT,
							Anchor::TOP_LEFT,
							icon_restart.id(),
						);
						ui.add_child(middle_frame.id(), btn_restart.id());

						let icon_playpause = ui.sprite(
							wk!(),
							snaek_sheet_id,
							snaek_sheet.icon_pause,
							Anchor::CENTER,
							Anchor::CENTER,
							Some(alphacomp::xor),
						);
						let btn_playpause = ui.btn_box(
							wk!(),
							WidgetSize::hug(),
							WidgetPadding::hv(3, 2),
							WidgetSprite::NineSlice(snaek_sheet_id, snaek_sheet.box_embossed),
							WidgetSprite::NineSlice(snaek_sheet_id, snaek_sheet.box_carved),
							Anchor::TOP_LEFT,
							Anchor::TOP_LEFT,
							icon_playpause.id(),
						);
						ui.add_child(middle_frame.id(), btn_playpause.id());
					}
					ui.add_child(display_frame.id(), middle_frame.id());
				}
				ui.add_child(game_frame.id(), display_frame.id());
			}
			ui.add_child(window_frame.id(), game_frame.id());
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
