use std::env;
use std::time::Duration;

use log::debug;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

use crate::position::Position;

mod object;
mod position;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

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

    let mut object = object::Object::new(Position::new(0, 0), 100, 100);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => match keycode {
                    Keycode::Right => {
                        object.move_right(5);
                    }
                    Keycode::Left => {
                        object.move_left(5);
                    }
                    Keycode::Down => {
                        object.move_down(5);
                    }
                    Keycode::Up => {
                        object.move_up(5);
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        if object.hit_wall(SCREEN_WIDTH, SCREEN_HEIGHT) {
            debug!("object (id: {}) hit wall", object.id());
        }
        object.draw(&mut canvas, Color::RGB(255, 0, 0));

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
