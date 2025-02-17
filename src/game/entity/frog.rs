use macroquad::{color::Color, math::{vec2, Rect, Vec2}};

use crate::resources::Resources;

const SHAKE_TIME: f32 = 1.0;

enum State {
    Waiting(f32), // Time left
    Jumping,
    Falling,
    Dead,
}

pub struct Frog {

}

impl Frog {
    pub fn hitbox() -> Rect {
        Rect::new(4.0, 7.0, 11.0, 8.0)
    }
    pub fn tile_offset() -> Vec2 {
        vec2(-1.0, -1.0)
    }
    
    pub fn draw_editor(pos: Vec2, camera_pos: Vec2, color: Color, resources: &Resources) {
        Self::draw(&State::Waiting(SHAKE_TIME + 1.0), pos, camera_pos, color, resources);
    }
    pub fn object_selector_rect() -> Rect {
        Rect::new(0.0, -6.0, 19.0, 11.0)
    }

    fn draw(state: &State, pos: Vec2, camera_pos: Vec2, color: Color, resources: &Resources) {
        let (x, x_offset) = match state {
            // Normal waiting
            State::Waiting(t) if *t > SHAKE_TIME => (0.0, 0.0),
            // Down and shaking
            // TODO: Make shake work
            State::Waiting(t) => (19.0, *t),
            // Leaping
            State::Jumping |
            State::Falling => (38.0, 0.0),
            // DEAD!!
            State::Dead    => (57.0, 0.0)
        };

        let rect = Rect::new(0.0 + x, 64.0, 19.0, 17.0);
        resources.draw_rect(pos + vec2(x_offset, 0.0) - camera_pos, rect, color, resources.entity_atlas());
    }
}