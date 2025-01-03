// The player struct.
// Controlled with keyboard, collides with world, etc.

use macroquad::{color::{BLUE, GREEN, ORANGE, RED, WHITE, YELLOW}, math::{vec2, Vec2}, shapes::draw_circle};

// Collision points

const HEAD:   Vec2 = vec2( 8.0,  0.5);
const SIDE_L: Vec2 = vec2( 3.5, 10.0);
const SIDE_R: Vec2 = vec2(12.5, 10.0);
const FOOT_L: Vec2 = vec2( 5.5, 16.0);
const FOOT_R: Vec2 = vec2(10.5, 16.0);

pub struct Player {
    pos: Vec2,
}

impl Default for Player {
    fn default() -> Self {
        Self { pos: Vec2::ZERO }
    }
}

impl Player {
    pub fn draw(&self) {
        for (p, c) in [
            (Vec2::ZERO, WHITE),
            (HEAD, BLUE),
            (SIDE_L, RED),
            (SIDE_R, ORANGE),
            (FOOT_L, YELLOW),
            (FOOT_R, GREEN),
        ] {
            draw_circle(self.pos.x + p.x, self.pos.y + p.y, 2.5, c);
        }
    }
}