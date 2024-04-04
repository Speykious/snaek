use std::error::Error;
use std::time::Duration;

use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};
use owo_colors::OwoColorize;

const WIDTH: usize = 97;
const HEIGHT: usize = 124;

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
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let options = WindowOptions {
        borderless: true,
        title: true,
        resize: false,
        scale: Scale::X4,
        scale_mode: ScaleMode::Stretch,
        ..Default::default()
    };

    let mut window = Window::new("Snaek", WIDTH, HEIGHT, options)?;

    window.limit_update_rate(Some(Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for i in buffer.iter_mut() {
            *i = 0x8b9bb4;
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }

    Ok(())
}
