extern crate piston;

extern crate graphics;
extern crate opengl_graphics;
extern crate glutin_window;

use piston::window::{WindowSettings};
use piston::input::*;
use piston::event_loop::*;
use glutin_window::GlutinWindow as AppWindow;
use graphics::*;

use opengl_graphics::{ GlGraphics, OpenGL, GlyphCache, TextureSettings, Filter };

// internal imports
mod paddle;
mod constants;
mod app;

fn main() {
    // Pick a version of OpenGL to use
    let opengl = OpenGL::V3_2;

    let mut window: AppWindow = WindowSettings::new("Pong", [constants::WINDOW_SIZE, constants::WINDOW_SIZE])
        .exit_on_esc(true)
        .resizable(false)
        .opengl(opengl)
        .build()
        .unwrap();
    
    let mut events = Events::new(EventSettings::new().lazy(true));
    let ref mut gl = GlGraphics::new(opengl);
    // let factory = window.factory.clone();
    // gl.
    let ref mut glyphs = GlyphCache::new("./FiraSans-Regular.ttf", (), TextureSettings::new())
      .expect("Could not load font");
    // let ref mut glyphs = Glyphs::new("../assets/FiraSans-Regular.ttf", gl, TextureSettings::new()).unwrap();

    const MID_Y: f64 = constants::WINDOW_SIZE / 2.0;
    let mut left_paddle = paddle::Paddle{position: [constants::PADDLE_WIDTH, MID_Y]};
    let mut right_paddle = paddle::Paddle{position: [constants::WINDOW_SIZE - constants::PADDLE_WIDTH, MID_Y]};
    let mut app = app::App{player_score: 0, ai_score: 0};

    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.button_args() {
            if args.scancode == Some(constants::SCAN_CODE_FOR_UP_ARROW) {
                left_paddle.move_by(-constants::PADDLE_SPEED);
                right_paddle.move_by(constants::PADDLE_SPEED);
            } else if args.scancode == Some(constants::SCAN_CODE_FOR_DOWN_ARROW) {
                left_paddle.move_by(constants::PADDLE_SPEED);
                right_paddle.move_by(-constants::PADDLE_SPEED);
            }
        }

        if let Some(r) = e.render_args() {
            let viewport = r.viewport();
            let context = gl.draw_begin(viewport);
            clear(constants::WHITE, gl);
            right_paddle.render(gl, &context.transform);
            left_paddle.render(gl, &context.transform);
            app.render(gl, &context.transform, glyphs);
            gl.draw_end();
        }
    }
}