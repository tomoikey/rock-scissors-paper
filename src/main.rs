use std::env;
use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

use crate::position::Position;
use crate::screen::Screen;
use crate::velocity::Velocity;

mod object;
mod paper;
mod position;
mod rock;
mod scissors;
mod screen;
mod screen_object;
mod velocity;

pub const SCREEN_WIDTH: u32 = 500;
pub const SCREEN_HEIGHT: u32 = 500;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Rust-SDL2", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    // init logger
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let mut screen = Screen::new(SCREEN_WIDTH, SCREEN_HEIGHT);

    let objects_count = 50;
    for i in 0..objects_count {
        let size = 500 / objects_count;
        let (position, width, height, mass, velocity) = (
            Position::random(
                (SCREEN_WIDTH - size) as f64,
                (SCREEN_HEIGHT - size) as f64,
                (SCREEN_WIDTH - size) as f64,
                (SCREEN_HEIGHT - size) as f64,
            ),
            size,
            size,
            1f64,
            Velocity::random(1.3, 1.3),
        );
        let object = match i % 3 {
            0 => screen_object::ScreenObject::Paper(paper::Paper::new(
                position,
                width,
                height,
                1.0 * mass,
                velocity,
            )),
            1 => screen_object::ScreenObject::Rock(rock::Rock::new(
                position,
                width,
                height,
                5.0 * mass,
                velocity,
            )),
            2 => screen_object::ScreenObject::Scissors(scissors::Scissors::new(
                position,
                width,
                height,
                25.0 * mass,
                velocity,
            )),
            _ => unreachable!(),
        };

        screen.add_object(object);
    }

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown { .. } => {}
                _ => {}
            }
        }
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        screen.draw(&mut canvas);

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 240));
    }
}
