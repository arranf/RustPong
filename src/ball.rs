use graphics::*;
use graphics::math::Matrix2d;
use opengl_graphics::{ GlGraphics };

use crate::constants;


pub struct Ball {
    pub position: [f64; 2],
    pub velocity: [f64; 2]
}

impl Ball{
    pub fn x(&mut self) -> f64 {
        self.position[0]
    }

    pub fn y(&mut self) -> f64 {
        self.position[1]
    }

    pub fn render(&mut self, gl: &mut GlGraphics, transform: &Matrix2d) {
        let (x, y) = (self.x(), self.y());
        let rect = rectangle::square(x, y, constants::PADDLE_WIDTH);
        ellipse(constants::BLACK, rect, *transform, gl);
    }

    pub fn move_frame(&mut self) {
        self.position = [self.x() + self.velocity[0], self.y() + self.velocity[1]];
    }
}