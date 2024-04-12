// TODO: remove this thing as soon as possible
#![allow(unused)]

use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::time::Duration;

use self::render::bitmap::Bitmap;
use self::render::pixel::{alphacomp, Pixel};
use self::render::pos::{pos, Pos};
use self::render::size::size;
use self::render::Rect;
use image::{ImageFormat, ImageResult};
use minifb::{Key, MouseMode, Scale, ScaleMode, Window, WindowOptions};
use owo_colors::OwoColorize;
use render::{DrawCommand, Renderer};

mod render;
mod snake;
mod spritesheet;
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

fn game() -> Result<(), Box<dyn Error>> {
    let mut renderer = Renderer::new(Bitmap::new(size(WIDTH, HEIGHT)));

    let ascii_chars_id = renderer.register_spritesheet(load_png_from_memory(IMG_ASCII_CHARS)?);
    let snaeksheet_id = renderer.register_spritesheet(load_png_from_memory(IMG_SNAEKSHEET)?);

    let ascii_chars = spritesheet::ascii_chars_spritesheet(ascii_chars_id);
    let snaeksheet = spritesheet::snaeksheet(snaeksheet_id);

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
            pixel: Pixel::from_hex(0xff801234),
            rect: Rect::from_pos_size(center + pos(-8, -10), bounce_size),
            dpos: pos(-1, -1),
        },
        Bounce {
            pixel: Pixel::from_hex(0x80128034),
            rect: Rect::from_pos_size(center + pos(9, -13), bounce_size),
            dpos: pos(1, -1),
        },
        Bounce {
            pixel: Pixel::from_hex(0xff123480),
            rect: Rect::from_pos_size(center + pos(11, 12), bounce_size),
            dpos: pos(1, 1),
        },
    ];

    let mut mouse_pos = Pos::ZERO;
    while window.is_open() {
        // input handling
        if window.is_key_down(Key::Escape) {
            break;
        }

        if let Some(next_pos) = window.get_mouse_pos(MouseMode::Clamp) {
            mouse_pos = pos(next_pos.0.round() as i16, next_pos.1.round() as i16);
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
        let mut draw_cmds = Vec::new();
        {
            draw_cmds.push(DrawCommand::Fill {
                rect: renderer.rect(),
                color: Pixel::from_hex(0xff262b44),
                acf: alphacomp::dst,
            });

            draw_cmds.push(DrawCommand::Stroke {
                rect: renderer.rect(),
                stroke_width: 1,
                color: Pixel::from_hex(0xff181425),
                acf: alphacomp::dst,
            });

            draw_cmds.push(DrawCommand::BeginComposite);
            {
                draw_cmds.push(DrawCommand::Fill {
                    rect: renderer.rect(),
                    color: Pixel::from_hex(0x10000000),
                    acf: alphacomp::over,
                });
                for bounce in &bounces {
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
                        sprite: snaeksheet.snaek_icon,
                        acf: alphacomp::over,
                    });

                    draw_cmds.push(DrawCommand::NineSlicingSprite {
                        rect: Rect::from_ab(pos(10, 10), mouse_pos),
                        nss: snaeksheet.box_embossed,
                        acf: alphacomp::over,
                    });
                }
                draw_cmds.push(DrawCommand::EndComposite(alphacomp::over));
            }
            draw_cmds.push(DrawCommand::EndComposite(alphacomp::over));
        }
        renderer.draw(&draw_cmds);

        window
            .update_with_buffer(
                renderer.first_framebuffer().pixels(),
                WIDTH as usize,
                HEIGHT as usize,
            )
            .unwrap();
    }

    Ok(())
}
