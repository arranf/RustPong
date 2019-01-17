
use piston_window::character::CharacterCache;
use graphics::*;
use graphics::math::Matrix2d;

use crate::constants;

pub struct App {
 pub player_score: u32,
 pub ai_score: u32   
}

impl App {
    fn increment_player_score(&mut self) {
        self.player_score = self.player_score + 1;
    }

    fn increment_ai_score(&mut self) {
        self.ai_score = self.ai_score + 1;
    }

    pub fn render<C, G>(&mut self, g: &mut G, transform: &Matrix2d, glyph: &mut C) where C: CharacterCache, G: Graphics<Texture=C::Texture> {
        let left_transform = transform.trans(constants::WINDOW_SIZE/3.0, constants::WINDOW_SIZE/8.0);
        text(constants::BLACK, 28, self.player_score.to_string().as_ref(), glyph, left_transform, g);

        let right_transform = transform.trans((constants::WINDOW_SIZE/3.0) * 2.0, constants::WINDOW_SIZE/8.0);
        let result = text(constants::BLACK, 28, self.ai_score.to_string().as_ref(), glyph, right_transform, g);
        if result.is_err() {
            result.unwrap_err();
        }
    }
}