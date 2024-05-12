#![allow(clippy::too_many_arguments)]
#![allow(unused)] // TODO: remove this thing as soon as possible

use std::error::Error;
use std::time::{Duration, Instant};

use self::math::pos::pos;
use self::math::size::size;
use self::render::bitmap::Bitmap;
use self::render::color::{alphacomp, Color};
use image::{ImageFormat, ImageResult};
use math::size::Size;
use minifb::{CursorStyle, Key, MouseButton, MouseMode, Scale, ScaleMode, Window, WindowOptions};
use owo_colors::OwoColorize;
use render::{DrawCommand, Renderer, Text};
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

	const SNAEK_BLACK: Color = Color::from_hex(0xff181425);

	let options = WindowOptions {
		borderless: true,
		title: true,
		resize: false,
		scale: Scale::X4,
		scale_mode: ScaleMode::Stretch,
		..Default::default()
	};

	let mut window = Window::new("Snaek", WIDTH as usize, HEIGHT as usize, options)?;
	window.limit_update_rate(Some(Duration::from_micros(1_000_000 / 300)));

	let start = Instant::now();
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
		let window_frame = ui.build_widget(
			WidgetProps::new(wk!())
				.with_flags(WidgetFlags::DRAW_BACKGROUND | WidgetFlags::DRAW_BORDER)
				.with_color(Color::from_hex(0xffc0cbdc))
				.with_border_color(Color::from_hex(0xff181425))
				.with_border_width(1)
				.with_acf(Some(alphacomp::dst))
				.with_size(WidgetSize::fill())
				.with_padding(WidgetPadding::all(1))
				.with_layout(WidgetLayout::flex(FlexDirection::Vertical, 0)),
		);
		{
			let navbar = ui.build_widget(
				WidgetProps::new(wk!())
					.with_flags(WidgetFlags::CAN_CLICK)
					.with_size(WidgetSize::new(WidgetDim::Fill, WidgetDim::Fixed(8)))
					.with_layout(WidgetLayout::flex(FlexDirection::Horizontal, 0)),
			);
			{
				let snaek_icon = ui.build_widget(
					WidgetProps::simple_sprite(wk!(), snaek_sheet_id, snaek_sheet.snaek_icon)
						.with_size(WidgetSize::fixed(8, 8))
						.with_draw_offset(pos(1, 1))
						.with_layout(WidgetLayout::flex(FlexDirection::Horizontal, 0)),
				);
				ui.add_child(navbar.id(), snaek_icon.id());

				let menu = ui.build_widget(WidgetProps::new(wk!()).with_size(WidgetSize::fill()));
				ui.add_child(navbar.id(), menu.id());

				let btn_close = ui.btn_icon(
					WidgetProps::new(wk!()).with_size(WidgetSize::fixed(7, 7)),
					WidgetProps::simple_sprite(wk!(), snaek_sheet_id, snaek_sheet.icon_close)
						.with_mask_and(Some(SNAEK_BLACK)),
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

			let game_frame = ui.build_widget(
				WidgetProps::nine_slice_sprite(wk!(), snaek_sheet_id, snaek_sheet.box_embossed)
					.with_acf(Some(alphacomp::dst))
					.with_size(WidgetSize::fill())
					.with_padding(WidgetPadding::trbl(4, 5, 5, 5))
					.with_layout(WidgetLayout::flex(FlexDirection::Vertical, 2)),
			);
			{
				let display_frame = ui.build_widget(
					WidgetProps::new(wk!())
						.with_size(WidgetSize::new(WidgetDim::Fill, WidgetDim::Hug))
						.with_layout(WidgetLayout::flex(FlexDirection::Horizontal, 3)),
				);
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

					let middle_frame = ui.build_widget(
						WidgetProps::new(wk!())
							.with_size(WidgetSize::hug())
							.with_layout(WidgetLayout::flex(FlexDirection::Vertical, 2)),
					);
					{
						let icon_restart = ui.build_widget(
							WidgetProps::simple_sprite(wk!(), snaek_sheet_id, snaek_sheet.icon_restart)
								.with_anchor_origin(Anchor::CENTER, Anchor::CENTER)
								.with_acf(Some(alphacomp::xor)),
						);
						let btn_restart = ui.btn_box(
							WidgetProps::new(wk!())
								.with_size(WidgetSize::hug())
								.with_padding(WidgetPadding::hv(3, 2)),
							WidgetSprite::NineSlice(snaek_sheet_id, snaek_sheet.box_embossed),
							WidgetSprite::NineSlice(snaek_sheet_id, snaek_sheet.box_carved),
							icon_restart.id(),
						);
						ui.add_child(middle_frame.id(), btn_restart.id());

						let icon_playpause = ui.build_widget(
							WidgetProps::simple_sprite(wk!(), snaek_sheet_id, snaek_sheet.icon_pause)
								.with_anchor_origin(Anchor::CENTER, Anchor::CENTER)
								.with_acf(Some(alphacomp::xor)),
						);
						let btn_playpause = ui.btn_box(
							WidgetProps::new(wk!())
								.with_size(WidgetSize::hug())
								.with_padding(WidgetPadding::hv(3, 2)),
							WidgetSprite::NineSlice(snaek_sheet_id, snaek_sheet.box_embossed),
							WidgetSprite::NineSlice(snaek_sheet_id, snaek_sheet.box_carved),
							icon_playpause.id(),
						);
						ui.add_child(middle_frame.id(), btn_playpause.id());
					}
					ui.add_child(display_frame.id(), middle_frame.id());

					let right_frame = ui.build_widget(
						WidgetProps::new(wk!())
							.with_size(WidgetSize::fill())
							.with_layout(WidgetLayout::flex(FlexDirection::Vertical, 2)),
					);
					{
						let text_holder = ui.build_widget(WidgetProps::new(wk!()).with_size(WidgetSize::fill()));
						{
							let text = ui.build_widget(
								WidgetProps::text(wk!(), renderer.text("Speykious"))
									.with_anchor_origin(Anchor::BOTTOM_LEFT, Anchor::BOTTOM_LEFT)
									.with_mask_and(Some(SNAEK_BLACK)),
							);
							ui.add_child(text_holder.id(), text.id());
						}
						ui.add_child(right_frame.id(), text_holder.id());

						let time_display = ui.time_display(
							wk!(),
							start.elapsed(),
							snaek_sheet_id,
							snaek_sheet.box_num_display,
							snaek_sheet.num_colon,
							&snaek_sheet.nums,
						);
						ui.add_child(right_frame.id(), time_display.id());
					}
					ui.add_child(display_frame.id(), right_frame.id());
				}
				ui.add_child(game_frame.id(), display_frame.id());

				let playfield = ui.build_widget(
					WidgetProps::nine_slice_sprite(wk!(), snaek_sheet_id, snaek_sheet.box_playfield)
						.with_size(WidgetSize::fill())
						.with_padding(WidgetPadding::all(4)),
				);
				{
					let playfield_bg = ui.build_widget(
						WidgetProps::new(wk!())
							.with_flags(WidgetFlags::DRAW_BACKGROUND)
							.with_color(Color::from_hex(0xff262b44)),
					);
					ui.add_child(playfield.id(), playfield_bg.id());
				}
				ui.add_child(game_frame.id(), playfield.id());
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
