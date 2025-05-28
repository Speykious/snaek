use std::error::Error;
use std::num::NonZero;
use std::rc::Rc;
use std::time::Instant;

use crate::snake::Direction;

use self::math::pos::pos;
use self::math::size::size;
use self::render::bitmap::Bitmap;
use self::render::color::{alphacomp, Color};
use image::{ImageFormat, ImageResult};
use math::size::Size;
use owo_colors::OwoColorize;
use render::{DrawCommand, Renderer, Rotate, SpritesheetId};
use snake::{Banana, SnaekSheet, SnakeGame};
use ui::{
	Anchor, FlexDirection, Mouse, UiContext, WidgetDim, WidgetFlags, WidgetId, WidgetLayout, WidgetPadding,
	WidgetProps, WidgetSize, WidgetSprite,
};

use winit::application::ApplicationHandler;
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event::{ElementState, KeyEvent, MouseButton, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{CursorIcon, Icon, Theme, Window, WindowAttributes, WindowId};

mod math;
mod render;
mod snake;
mod ui;

const SNAEK_APP_ICON: &[u8] = include_bytes!("../assets/icon.png");

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

const WIDTH: u16 = 97;
const HEIGHT: u16 = 124;

const SNAEK_PIXEL_SIZE: u32 = 4;

const VIEWPORT_SIZE: Size = size(WIDTH, HEIGHT);
const SNAEK_BLACK: Color = Color::from_hex(0xff181425);

fn main() {
	eprintln!("{}", "Snaek!!".yellow());

	let event_loop = EventLoop::new().unwrap();
	event_loop.set_control_flow(ControlFlow::Poll);

	let mut app = match App::new() {
		Ok(app) => app,
		Err(e) => {
			eprintln!("{}", "The game crashed! D:".red());
			eprintln!("-> {}", e);
			std::process::exit(1);
		}
	};

	event_loop.run_app(&mut app).unwrap();

	eprintln!("{}", "See you next time :)".green())
}

struct App {
	window: Option<Rc<Window>>,
	surface: Option<softbuffer::Surface<Rc<Window>, Rc<Window>>>,
	icon: Icon,

	ui: UiContext,
	renderer: Renderer,
	draw_cmds: Vec<DrawCommand>,
	mouse: Mouse,
	last_move: Instant,
	window_size: PhysicalSize<u32>,
	pixel_size: u32,

	snaek_sheet_id: SpritesheetId,
	snaek_sheet: SnaekSheet,
	snake_game: SnakeGame,

	debug: bool,
	show_game_over: bool,
	next_direction: Direction,
}

impl App {
	fn new() -> Result<Self, Box<dyn Error>> {
		let icon = {
			let icon_image = image::load_from_memory_with_format(SNAEK_APP_ICON, ImageFormat::Png)?;
			let (icon_width, icon_height) = (icon_image.width(), icon_image.height());
			Icon::from_rgba(icon_image.into_rgba8().into_vec(), icon_width, icon_height)?
		};

		let ascii_bitmap = load_png_from_memory(IMG_ASCII_CHARS)?;

		let mut renderer = Renderer::new(VIEWPORT_SIZE, ascii_bitmap);
		let snaek_sheet_id = renderer.register_spritesheet(load_png_from_memory(IMG_SNAEKSHEET)?);

		let snake_game = SnakeGame::new(size(11, 11));
		let next_direction = snake_game.direction();

		Ok(Self {
			window: None,
			surface: None,
			icon,

			ui: UiContext::new(VIEWPORT_SIZE),
			renderer,
			draw_cmds: Vec::new(),
			mouse: Mouse::default(),
			last_move: Instant::now(),
			window_size: PhysicalSize::default(),
			pixel_size: SNAEK_PIXEL_SIZE,

			snaek_sheet_id,
			snaek_sheet: snake::snaek_sheet(),
			snake_game,

			debug: false,
			show_game_over: false,
			next_direction,
		})
	}
}

impl ApplicationHandler for App {
	fn can_create_surfaces(&mut self, event_loop: &ActiveEventLoop) {
		let win_attribs = WindowAttributes::default()
			.with_active(true)
			.with_transparent(false)
			.with_decorations(false)
			.with_theme(Some(Theme::Light))
			.with_title("Snaek :3")
			.with_window_icon(Some(self.icon.clone()))
			.with_resizable(true);

		let win = Rc::new(event_loop.create_window(win_attribs).unwrap());

		self.pixel_size = (SNAEK_PIXEL_SIZE as f64 * win.scale_factor()).round() as u32;

		let viewport_size = PhysicalSize::new(WIDTH as u32 * self.pixel_size, HEIGHT as u32 * self.pixel_size);
		let win_size = winit::dpi::Size::Physical(viewport_size);
		let inc_size = winit::dpi::Size::Physical(PhysicalSize::new(self.pixel_size, self.pixel_size));

		let _ = win.request_inner_size(win_size);
		win.set_min_inner_size(Some(win_size));
		win.set_resize_increments(Some(inc_size));

		let context = softbuffer::Context::new(win.clone()).unwrap();
		let mut surface = softbuffer::Surface::new(&context, win.clone()).unwrap();

		if let (Some(width), Some(height)) = (NonZero::new(viewport_size.width), NonZero::new(viewport_size.height)) {
			surface.resize(width, height).unwrap();
		}

		self.surface = Some(surface);
		self.window = Some(win);
	}

	fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
		match event {
			WindowEvent::CloseRequested => {
				event_loop.exit();
			}

			WindowEvent::MouseInput { state, button, .. } => {
				use MouseButton as M;

				self.mouse.l_pressed = (state.is_pressed() && button == M::Left, self.mouse.l_pressed.0);
				self.mouse.r_pressed = (state.is_pressed() && button == M::Right, self.mouse.r_pressed.0);
				self.mouse.m_pressed = (state.is_pressed() && button == M::Middle, self.mouse.m_pressed.0);
			}

			WindowEvent::CursorMoved {
				position: PhysicalPosition { x, y },
				..
			} => {
				self.mouse.x = x / self.pixel_size as f64;
				self.mouse.y = y / self.pixel_size as f64;
			}

			WindowEvent::KeyboardInput {
				event:
					KeyEvent {
						physical_key: PhysicalKey::Code(key_code),
						state: ElementState::Pressed,
						repeat: false,
						..
					},
				..
			} => match key_code {
				KeyCode::ArrowUp | KeyCode::KeyW => self.next_direction = Direction::Up,
				KeyCode::ArrowRight | KeyCode::KeyD => self.next_direction = Direction::Right,
				KeyCode::ArrowDown | KeyCode::KeyS => self.next_direction = Direction::Down,
				KeyCode::ArrowLeft | KeyCode::KeyA => self.next_direction = Direction::Left,
				_ => {}
			},

			WindowEvent::Resized(PhysicalSize { width, height }) => {
				let width = width.max(WIDTH as u32 * self.pixel_size);
				let height = height.max(HEIGHT as u32 * self.pixel_size);

				self.window_size = PhysicalSize { width, height };

				let viewport_size = size((width / self.pixel_size) as u16, (height / self.pixel_size) as u16);

				self.ui.resize(viewport_size);
				self.renderer.resize(viewport_size);

				if let Some(surface) = &mut self.surface {
					if let (Some(width), Some(height)) = (NonZero::new(width), NonZero::new(height)) {
						surface.resize(width, height).unwrap();
					}
				}
			}

			WindowEvent::RedrawRequested => {
				let Some(window) = self.window.as_ref().cloned() else {
					return;
				};

				self.draw_cmds.clear();
				self.draw_cmds.push(DrawCommand::Clear);

				if snaek_ui(self, window.as_ref()) {
					event_loop.exit();
				}

				self.ui.solve_layout();
				self.ui.draw_widgets(&mut self.draw_cmds);
				self.ui.free_untouched_widgets();
				self.ui.react(&self.mouse);

				let now = Instant::now();
				self.snake_game.update_duration();

				if (now - self.last_move).as_secs_f64() >= (1.0 / self.snake_game.speed() as f64) {
					let was_dead = self.snake_game.is_dead();

					self.snake_game.change_direction(self.next_direction);
					self.snake_game.update();
					self.next_direction = self.snake_game.direction();

					if self.snake_game.is_dead() && !was_dead {
						self.show_game_over = true;
					}

					self.last_move = now;
				}

				self.renderer.draw(&self.draw_cmds);

				if let Some(surface) = &mut self.surface {
					let fb = self.renderer.first_framebuffer().pixels();

					let mut buffer = surface.buffer_mut().unwrap();

					if buffer.is_empty() {
						return;
					}

					let (width, height) = (self.window_size.width as usize, self.window_size.height as usize);
					let pxsz = self.pixel_size as usize;
					for y in 0..height {
						for x in 0..width {
							let dst_index = y * width + x;
							let src_index = (y / pxsz) * (width / pxsz) + (x / pxsz);
							if let (Some(dst_pixel), Some(src_pixel)) = (buffer.get_mut(dst_index), fb.get(src_index)) {
								*dst_pixel = *src_pixel;
							};
						}
					}

					buffer.present().unwrap();
				}

				window.request_redraw();
			}
			_ => (),
		}
	}
}

fn snaek_ui(app: &mut App, window: &Window) -> bool {
	let mut cursor_icon = CursorIcon::Default;

	let App {
		ui,
		renderer,
		snaek_sheet_id,
		snaek_sheet,
		snake_game,
		debug,
		show_game_over,
		next_direction,
		..
	} = app;

	let snaek_sheet_id = *snaek_sheet_id;

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
				.with_flags(WidgetFlags::CAN_HOVER | WidgetFlags::CAN_CLICK)
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

			let filler = ui.build_widget(
				WidgetProps::new(wk!())
					.with_size(WidgetSize::fill())
					.with_padding(WidgetPadding::hv(2, 1)),
			);
			{
				let title = ui.build_widget(
					WidgetProps::text(wk!(), renderer.text("Snaek"))
						.with_anchor_origin(Anchor::BOTTOM_LEFT, Anchor::BOTTOM_LEFT)
						.with_mask_and(Some(SNAEK_BLACK)),
				);
				ui.add_child(filler.id(), title.id());
			}
			ui.add_child(navbar.id(), filler.id());

			let btn_close = ui.btn_icon(
				WidgetProps::new(wk!()).with_size(WidgetSize::fixed(7, 7)),
				WidgetProps::simple_sprite(wk!(), snaek_sheet_id, snaek_sheet.icon_close)
					.with_mask_and(Some(SNAEK_BLACK)),
				Color::from_hex(0xffe43b44),
			);
			ui.add_child(navbar.id(), btn_close.id());

			if btn_close.clicked() {
				return true;
			}
		}
		ui.add_child(window_frame.id(), navbar.id());

		if navbar.hovered() {
			cursor_icon = CursorIcon::Grab;
		}

		if navbar.start_pressed() {
			cursor_icon = CursorIcon::Grabbing;
			window.drag_window().unwrap();
			app.mouse.reset_pressed();
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
					snake_game.bananas_eaten() as usize,
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

					if btn_restart.clicked() {
						snake_game.restart();
						*show_game_over = false;
						*next_direction = snake_game.direction();
					}

					let icon_playpause = {
						let sprite = match debug {
							true => snaek_sheet.icon_play,
							false => snaek_sheet.icon_debug,
						};

						ui.build_widget(
							WidgetProps::simple_sprite(wk!(), snaek_sheet_id, sprite)
								.with_anchor_origin(Anchor::CENTER, Anchor::CENTER)
								.with_acf(Some(alphacomp::xor)),
						)
					};
					let btn_playdebug = ui.btn_box(
						WidgetProps::new(wk!())
							.with_size(WidgetSize::hug())
							.with_padding(WidgetPadding::hv(3, 2)),
						WidgetSprite::NineSlice(snaek_sheet_id, snaek_sheet.box_embossed),
						WidgetSprite::NineSlice(snaek_sheet_id, snaek_sheet.box_carved),
						icon_playpause.id(),
					);
					ui.add_child(middle_frame.id(), btn_playdebug.id());

					if btn_playdebug.clicked() {
						*debug = !*debug;
					}
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
								.with_anchor_origin(Anchor::BOTTOM_RIGHT, Anchor::BOTTOM_RIGHT)
								.with_mask_and(Some(SNAEK_BLACK)),
						);
						ui.add_child(text_holder.id(), text.id());
					}
					ui.add_child(right_frame.id(), text_holder.id());

					let time_display = ui.time_display(
						wk!(),
						snake_game.duration(),
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

			let playfield_frame = ui.build_widget(WidgetProps::new(wk!()).with_size(WidgetSize::fill()));
			{
				let playfield = ui.build_widget(
					WidgetProps::nine_slice_sprite(wk!(), snaek_sheet_id, snaek_sheet.box_playfield)
						.with_anchor_origin(Anchor::CENTER, Anchor::CENTER)
						.with_size(WidgetSize::hug())
						.with_padding(WidgetPadding::all(4)),
				);
				{
					let container_size = snake_game.size() * 7;

					let snake_container = ui.build_widget(
						WidgetProps::new(wk!())
							.with_flags(WidgetFlags::DRAW_BACKGROUND)
							.with_color(Color::from_hex(0xff262b44))
							.with_size(WidgetSize::fixed(container_size.w, container_size.h)),
					);
					{
						snaek_playfield(
							snake_game,
							ui,
							renderer,
							snake_container.id(),
							snaek_sheet_id,
							snaek_sheet,
							*debug,
							show_game_over,
						);
					}
					ui.add_child(playfield.id(), snake_container.id());
				}
				ui.add_child(playfield_frame.id(), playfield.id());
			}
			ui.add_child(game_frame.id(), playfield_frame.id());
		}
		ui.add_child(window_frame.id(), game_frame.id());
	}

	window.set_cursor(cursor_icon);
	false
}

#[allow(clippy::too_many_arguments)]
fn snaek_playfield(
	snake_game: &SnakeGame,
	ui: &mut UiContext,
	renderer: &Renderer,
	container_id: WidgetId,
	snaek_sheet_id: SpritesheetId,
	snaek_sheet: &SnaekSheet,
	debug: bool,
	show_game_over: &mut bool,
) {
	let playfield_size = snake_game.size();
	for y in 0..playfield_size.h as i16 {
		for x in 0..playfield_size.w as i16 {
			let slot_pos = pos(x, y);
			let slot = snake_game.slot_at(slot_pos);

			let (ikey_x, ikey_y) = (slot_pos.x as u64, slot_pos.y as u64);
			let mut holder_props = WidgetProps::new(wk!(ikey_x, ikey_y))
				.with_size(WidgetSize::fixed(7, 7))
				.with_pos(slot_pos * 7);

			if debug {
				holder_props = holder_props
					.with_flags(WidgetFlags::DRAW_BORDER)
					.with_border_color(Color::from_hex(0xff333333))
					.with_border_width(1)
					.with_acf(Some(alphacomp::add));
			}

			let sprite_holder = ui.build_widget(holder_props);
			{
				if let Some(banana) = slot.banana() {
					let banana_sprite = match banana {
						Banana::Yellow => snaek_sheet.banana_yellow,
						Banana::Red => snaek_sheet.banana_red,
						Banana::Cyan => snaek_sheet.banana_cyan,
					};

					let sprite = ui.build_widget(
						WidgetProps::simple_sprite(wk!(), snaek_sheet_id, banana_sprite)
							.with_anchor_origin(Anchor::CENTER, Anchor::CENTER),
					);
					ui.add_child(sprite_holder.id(), sprite.id());
				}

				let is_straight = slot.direction_next() == slot.direction_prev().opposite();

				let snake_sprite = match (slot.has_snake_head(), slot.has_snake_tail()) {
					(true, true) if is_straight => {
						let rotate = match slot.direction_next() {
							Direction::Up => Rotate::R270,
							Direction::Right => Rotate::R0,
							Direction::Down => Rotate::R90,
							Direction::Left => Rotate::R180,
						};
						Some((snaek_sheet.snake_straight, rotate))
					}
					(true, true) => {
						use Direction as D;
						let rotate = match (slot.direction_next(), slot.direction_prev()) {
							(D::Up, D::Right) | (D::Right, D::Up) => Rotate::R270,
							(D::Right, D::Down) | (D::Down, D::Right) => Rotate::R0,
							(D::Down, D::Left) | (D::Left, D::Down) => Rotate::R90,
							(D::Left, D::Up) | (D::Up, D::Left) => Rotate::R180,
							_ => Rotate::R0,
						};
						Some((snaek_sheet.snake_gay, rotate))
					}
					(true, false) => {
						let rotate = match slot.direction_prev() {
							Direction::Up => Rotate::R90,
							Direction::Right => Rotate::R180,
							Direction::Down => Rotate::R270,
							Direction::Left => Rotate::R0,
						};
						Some((snaek_sheet.snake_head, rotate))
					}
					(false, true) => {
						let rotate = match slot.direction_next() {
							Direction::Up => Rotate::R0,
							Direction::Right => Rotate::R90,
							Direction::Down => Rotate::R180,
							Direction::Left => Rotate::R270,
						};
						Some((snaek_sheet.snake_end, rotate))
					}
					(false, false) => None,
				};

				if let Some((snake_sprite, rotate)) = snake_sprite {
					let sprite = ui.build_widget(
						WidgetProps::simple_sprite(wk!(ikey_x, ikey_y), snaek_sheet_id, snake_sprite)
							.with_rotate(rotate)
							.with_anchor_origin(Anchor::CENTER, Anchor::CENTER),
					);
					ui.add_child(sprite_holder.id(), sprite.id());
				}

				// debug sprites
				if debug {
					// direction next
					let (anchor, w, h) = match slot.direction_next() {
						Direction::Up => (Anchor::TOP_CENTER, 1, 2),
						Direction::Right => (Anchor::CENTER_RIGHT, 2, 1),
						Direction::Down => (Anchor::BOTTOM_CENTER, 1, 2),
						Direction::Left => (Anchor::CENTER_LEFT, 2, 1),
					};

					let sprite = ui.build_widget(
						WidgetProps::new(wk!(ikey_x, ikey_y))
							.with_flags(WidgetFlags::DRAW_BACKGROUND)
							.with_color(Color::from_hex(0xff116611))
							.with_size(WidgetSize::fixed(w, h))
							.with_anchor_origin(anchor, anchor)
							.with_acf(Some(alphacomp::add)),
					);
					ui.add_child(sprite_holder.id(), sprite.id());

					// direction prev
					let (anchor, w, h) = match slot.direction_prev() {
						Direction::Up => (Anchor::TOP_CENTER, 1, 3),
						Direction::Right => (Anchor::CENTER_RIGHT, 3, 1),
						Direction::Down => (Anchor::BOTTOM_CENTER, 1, 3),
						Direction::Left => (Anchor::CENTER_LEFT, 3, 1),
					};

					let sprite = ui.build_widget(
						WidgetProps::new(wk!(ikey_x, ikey_y))
							.with_flags(WidgetFlags::DRAW_BACKGROUND)
							.with_color(Color::from_hex(0xff661111))
							.with_size(WidgetSize::fixed(w, h))
							.with_anchor_origin(anchor, anchor)
							.with_acf(Some(alphacomp::add)),
					);
					ui.add_child(sprite_holder.id(), sprite.id());
				}
			}
			ui.add_child(container_id, sprite_holder.id());
		}
	}

	if snake_game.ate_banana() {
		let head_pos = snake_game.snake_head();

		let (rotate, anchor) = match snake_game.slot_at(head_pos).direction_prev() {
			Direction::Up => (Rotate::R90, Anchor::TOP_CENTER),
			Direction::Right => (Rotate::R180, Anchor::CENTER_RIGHT),
			Direction::Down => (Rotate::R270, Anchor::BOTTOM_CENTER),
			Direction::Left => (Rotate::R0, Anchor::CENTER_LEFT),
		};

		let tongue_pos = head_pos + snake_game.direction().pos_offset();
		let tongue_holder = ui.build_widget(
			WidgetProps::new(wk!())
				.with_size(WidgetSize::fixed(7, 7))
				.with_pos(tongue_pos * 7),
		);
		{
			let tongue = ui.build_widget(
				WidgetProps::simple_sprite(wk!(), snaek_sheet_id, snaek_sheet.snake_tongue)
					.with_anchor_origin(anchor, anchor)
					.with_rotate(rotate),
			);
			ui.add_child(tongue_holder.id(), tongue.id());
		}
		ui.add_child(container_id, tongue_holder.id());
	}

	if *show_game_over {
		let game_over_overlay = ui.build_widget(
			WidgetProps::new(wk!())
				.with_flags(WidgetFlags::DRAW_BACKGROUND)
				.with_color(Color::from_hex(0x80ffffff & SNAEK_BLACK.to_u32()))
				.with_size(WidgetSize::fill()),
		);
		{
			let column = ui.build_widget(
				WidgetProps::new(wk!())
					.with_size(WidgetSize::hug())
					.with_anchor_origin(Anchor::CENTER, Anchor::CENTER)
					.with_layout(WidgetLayout::flex(FlexDirection::Vertical, 4)),
			);
			{
				let game_over_text = ui.build_widget(WidgetProps::text(wk!(), renderer.text("Game Over! :(")));
				ui.add_child(column.id(), game_over_text.id());

				let oh_text =
					ui.build_widget(WidgetProps::text(wk!(), renderer.text("Oh")).with_mask_and(Some(SNAEK_BLACK)));

				let oh_btn = ui.btn_box(
					WidgetProps::new(wk!())
						.with_size(WidgetSize::hug())
						.with_anchor_origin(Anchor::TOP_CENTER, Anchor::TOP_CENTER)
						.with_padding(WidgetPadding::hv(4, 2)),
					WidgetSprite::NineSlice(snaek_sheet_id, snaek_sheet.box_embossed),
					WidgetSprite::NineSlice(snaek_sheet_id, snaek_sheet.box_carved),
					oh_text.id(),
				);
				ui.add_child(column.id(), oh_btn.id());

				if oh_btn.clicked() {
					*show_game_over = false;
				}
			}
			ui.add_child(game_over_overlay.id(), column.id());
		}
		ui.add_child(container_id, game_over_overlay.id());
	}
}
