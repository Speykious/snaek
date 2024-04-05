// TODO: remove this thing as soon as possible
#![allow(unused)]

use std::error::Error;
use std::time::Duration;

use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};
use owo_colors::OwoColorize;
use render::bitmap::Bitmap;
use render::pixel::{alphacomp, Pixel};
use render::{pos, size, Pos, Rect};

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
    pub pixel: Pixel,
    pub rect: Rect,
    pub dpos: Pos,
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

    window.limit_update_rate(Some(Duration::from_micros(1_000_000 / 30)));

    let center = pos(WIDTH as i16 / 2, HEIGHT as i16 / 2);
    let bounce_size = size(20, 20);

    let mut bounces = [
        Bounce {
            pixel: Pixel::from_hex(0x80fee761),
            rect: Rect::from_pos_size(center + pos(-8, -10), bounce_size),
            dpos: pos(-1, -1),
        },
        Bounce {
            pixel: Pixel::from_hex(0x80ff4e7d),
            rect: Rect::from_pos_size(center + pos(9, -13), bounce_size),
            dpos: pos(1, -1),
        },
        Bounce {
            pixel: Pixel::from_hex(0x802ce8f5),
            rect: Rect::from_pos_size(center + pos(11, 12), bounce_size),
            dpos: pos(1, 1),
        },
    ];

    while window.is_open() {
        // input handling
        if window.is_key_down(Key::Escape) {
            break;
        }

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

        // drawing
        buffer.fill(Pixel::from_hex(0xff262b44), alphacomp::over);
        for bounce in &bounces {
            buffer.fill_area(bounce.pixel, bounce.rect, alphacomp::over);
        }

        window
            .update_with_buffer(buffer.pixels(), WIDTH as usize, HEIGHT as usize)
            .unwrap();
    }

    Ok(())
}
