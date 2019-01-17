use graphics::*;
use graphics::math::Matrix2d;
use opengl_graphics::{ GlGraphics };

use crate::constants;

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const BOTTOM_POSITION: f64 = constants::WINDOW_SIZE - (constants::PADDLE_HEIGHT);
const TOP_POSITION: f64 = 0.0 + (constants::PADDLE_HEIGHT);

impl Paddle {
    pub fn move_by(&mut self, y: f64) {
        let position = self.y() + y;
        if position >= BOTTOM_POSITION {
            self.position[1] = BOTTOM_POSITION;
        } else if position <= TOP_POSITION {
            self.position[1] = TOP_POSITION;
        } else {
            self.position[1] = position;
        }
    }

    pub fn x(&mut self) -> f64 {
        self.position[0]
    }

    pub fn y(&mut self) -> f64 {
        self.position[1]
    }

    pub fn render(&mut self, gl: &mut GlGraphics, transform: &Matrix2d) {
        let (x, y) = (self.x(), self.y());
        let rect = rectangle::centered([x, y, 5.0, constants::PADDLE_HEIGHT]);
        rectangle(BLACK, rect, *transform, gl);
    }
}

pub struct Paddle {
    pub position: [f64; 2]
}