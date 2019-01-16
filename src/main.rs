extern crate piston;

extern crate graphics;
extern crate opengl_graphics;
extern crate glutin_window;

use piston::window::{ WindowSettings };
use piston::input::*;
use piston::event_loop::*;

use graphics::*;

use opengl_graphics::{ GlGraphics, OpenGL };
use glutin_window::GlutinWindow as AppWindow;

// internal imports
mod paddle;
mod constants;

fn main() {
    // Pick a version of OpenGL to use
    let opengl = OpenGL::V3_2;

    let mut window: AppWindow = WindowSettings::new("Pong", [constants::WINDOW_SIZE, constants::WINDOW_SIZE])
        .exit_on_esc(true)
        .resizable(false)
        .opengl(opengl)
        .build()
        .unwrap();
    
    let ref mut gl = GlGraphics::new(opengl);

    let mut events = Events::new(EventSettings::new().lazy(true));
    const MID_Y: f64 = constants::WINDOW_SIZE / 2.0;
    let mut left_paddle = paddle::Paddle{position: [constants::PADDLE_WIDTH, MID_Y]};
    let mut right_paddle = paddle::Paddle{position: [constants::WINDOW_SIZE - constants::PADDLE_WIDTH, MID_Y]};

    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.button_args() {
            if args.scancode == Some(103) {
                left_paddle.move_by(-constants::PADDLE_SPEED);
            } else if args.scancode == Some(108) {
                left_paddle.move_by(constants::PADDLE_SPEED);
            }
        }

        if let Some(r) = e.render_args() {
            let viewport = r.viewport();
            let context = gl.draw_begin(viewport);
            clear(constants::WHITE, gl);
            right_paddle.render(gl, &context.transform);
            left_paddle.render(gl, &context.transform);
            gl.draw_end();
        }
    }
}