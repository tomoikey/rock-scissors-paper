use std::env;
use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

use crate::object::Object;
use crate::position::Position;
use crate::screen::Screen;

mod object;
mod position;
mod screen;
mod velocity;

pub const SCREEN_WIDTH: u32 = 800;
pub const SCREEN_HEIGHT: u32 = 600;

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
    let object = Object::new(
        Position::new(50, 50),
        Color::WHITE,
        100,
        100,
        1f64,
        velocity::Velocity::new(2f64, 2f64),
    );
    let object_id = screen.add_object(object);

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
                } => {
                    let object = screen.get_object_mut(object_id).unwrap();
                    match keycode {
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
                    }
                }
                _ => {}
            }
        }
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        screen.draw(&mut canvas);

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
