// TODO: remove this thing as soon as possible
#![allow(unused)]

use std::error::Error;
use std::time::Duration;

use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};
use owo_colors::OwoColorize;
use render::bitmap::Bitmap;
use render::pixel::{alphacomp, Pixel};
use render::{size, Rect};

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

fn game() -> Result<(), Box<dyn Error>> {
    let mut buffer = Bitmap::new(size(WIDTH, HEIGHT));

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

    let mut bounce = Rect::from_xywh(WIDTH as i16 / 2, HEIGHT as i16 / 2, 10, 10);

    let (mut dx, mut dy) = (-1, -1);
    while window.is_open() {
        // input handling
        if window.is_key_down(Key::Escape) {
            break;
        }

        // state update
        if bounce.x <= 0 {
            dx = 1;
        } else if bounce.x as i32 + bounce.w as i32 + 1 > WIDTH as i32 {
            dx = -1;
        }

        if bounce.y <= 0 {
            dy = 1;
        } else if bounce.y as i32 + bounce.h as i32 + 1 > HEIGHT as i32 {
            dy = -1;
        }

        bounce.x += dx;
        bounce.y += dy;

        // drawing
        buffer.fill(Pixel::from_hex(0x262b44), alphacomp::over);
        buffer.fill_area(Pixel::from_hex(0x80ff0051), bounce, alphacomp::over);

        window
            .update_with_buffer(buffer.pixels(), WIDTH as usize, HEIGHT as usize)
            .unwrap();
    }

    Ok(())
}
