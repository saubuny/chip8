use chip8_core::emu::*;
use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas, sys::KeyCode,
    video::Window,
};
use std::{env, fs};

const SCALE: u32 = 16;
const WINDOW_WIDTH: u32 = (SCREEN_WIDTH as u32) * SCALE;
const WINDOW_HEIGHT: u32 = (SCREEN_HEIGHT as u32) * SCALE;

const TICKS_PER_FRAME: usize = 10;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: cargo run [path/to/game]");
        return;
    }

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Chip8", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut chip8 = Emu::new();

    let rom = fs::read(&args[1]).expect("Unable to open file");
    chip8.load(&rom);

    'gameloop: loop {
        for evt in event_pump.poll_iter() {
            match evt {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'gameloop;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Backspace),
                    ..
                } => {
                    chip8.reset();
                    chip8.load(&rom);
                }
                Event::KeyDown {
                    keycode: Some(key), ..
                } => {
                    if let Some(k) = get_key(key) {
                        chip8.keypress(k, true);
                    }
                }

                Event::KeyUp {
                    keycode: Some(key), ..
                } => {
                    if let Some(k) = get_key(key) {
                        chip8.keypress(k, false);
                    }
                }
                _ => (),
            }
        }

        for _ in 0..TICKS_PER_FRAME {
            chip8.tick();
        }
        if chip8.tick_timers() {
            // TODO: Play a beeping noise
        };

        draw_screen(&chip8, &mut canvas);
    }
}

fn get_key(key: Keycode) -> Option<usize> {
    match key {
        Keycode::Num1 => Some(0x1),
        Keycode::Num2 => Some(0x2),
        Keycode::Num3 => Some(0x3),
        Keycode::Num4 => Some(0xC),
        Keycode::Q => Some(0x4),
        Keycode::W => Some(0x5),
        Keycode::E => Some(0x6),
        Keycode::R => Some(0xD),
        Keycode::A => Some(0x7),
        Keycode::S => Some(0x8),
        Keycode::D => Some(0x9),
        Keycode::F => Some(0xE),
        Keycode::Z => Some(0xA),
        Keycode::X => Some(0x0),
        Keycode::C => Some(0xB),
        Keycode::V => Some(0xF),
        _ => None,
    }
}

fn draw_screen(emu: &Emu, canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    let screen_buf = emu.get_display();
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    for (i, pixel) in screen_buf.iter().enumerate() {
        if *pixel {
            // Convert index to 2d coords
            let x = (i % SCREEN_WIDTH) as u32;
            let y = (i / SCREEN_WIDTH) as u32;

            let rect = Rect::new((x * SCALE) as i32, (y * SCALE) as i32, SCALE, SCALE);
            canvas.fill_rect(rect).unwrap();
        }
    }
    canvas.present();
}
