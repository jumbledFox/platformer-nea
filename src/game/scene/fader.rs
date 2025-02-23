use macroquad::{color::Color, math::{Rect, Vec2}};

use crate::{util::draw_rect, VIEW_SIZE};

// The time the screen is fading in or out
const FADE_TIME: f32 = 0.3;
// The time the screen is fully black
const BLACK_TIME: f32 = 0.1;

pub struct Fader {
    fading: bool,
    moved: bool,
    timer: f32,
    dest: Option<Vec2>,
}

impl Default for Fader {
    fn default() -> Self {
        Self { fading: false, moved: false, timer: 0.0, dest: None }
    }
}

impl Fader {
    pub fn begin_fade(&mut self, dest: Option<Vec2>) {
        self.fading = true;
        self.moved = false;
        self.timer = 0.0;
        self.dest = dest;
    }

    pub fn fading(&self) -> bool {
        self.fading
    }

    pub fn move_player(&mut self) -> Option<Vec2> {
        match self.timer >= FADE_TIME + BLACK_TIME * 0.5 {
            true  => {
                self.moved = true;
                self.dest.take()
            },
            false => None,
        }
    }

    pub fn update(&mut self, deltatime: f32) {
        if self.fading {
            self.timer += deltatime;
        }
        if self.timer >= FADE_TIME * 2.0 + BLACK_TIME  {
            self.fading = false;
        }
    }

    pub fn draw(&self) {
        if !self.fading {
            return;
        }
        let alpha = if self.timer < FADE_TIME {
            self.timer / FADE_TIME
        } else if self.timer > FADE_TIME + BLACK_TIME {
            1.0 - (self.timer - FADE_TIME - BLACK_TIME) / FADE_TIME
        } else {
            1.0
        };
        
        draw_rect(Rect::new(0.0, 0.0, VIEW_SIZE.x, VIEW_SIZE.y), Color::new(0.0, 0.0, 0.0, alpha));
    }
}