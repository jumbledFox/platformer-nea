use macroquad::{color::{Color, BLUE, GREEN, RED, WHITE}, input::{is_key_down, is_key_pressed, KeyCode}, math::{vec2, Rect, Vec2}, shapes::draw_circle};

use crate::{resources::Resources, text_renderer::render_text, util::draw_rect};

use super::{collision::{collision_bottom, collision_left, collision_right, collision_top}, level::Level, scene::PHYSICS_STEP};

// Collision points
const HEAD:    Vec2 = vec2( 8.0,  0.5);
const SIDE_LT: Vec2 = vec2( 4.0,  3.0);
const SIDE_LB: Vec2 = vec2( 4.0, 13.0);
const SIDE_RT: Vec2 = vec2(12.0,  3.0);
const SIDE_RB: Vec2 = vec2(12.0, 13.0);
const FOOT_L:  Vec2 = vec2( 5.0, 16.0);
const FOOT_R:  Vec2 = vec2(10.0, 16.0);

// Control
const KEY_LEFT:  KeyCode = KeyCode::A;
const KEY_RIGHT: KeyCode = KeyCode::D;
const KEY_JUMP:  KeyCode = KeyCode::Space;
const KEY_RUN:   KeyCode = KeyCode::LeftShift;
const KEY_UP:    KeyCode = KeyCode::W;

const MAX_FALL_SPEED: f32 = 2.0;
const GRAVITY: f32 = 0.045;

// Finite state-machine for movement
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum State {
    Standing,
    Moving,
    Jumping,
    Falling,
    Climbing,
}

pub struct Player {
    state: State,
    pos: Vec2,
    vel: Vec2,

    nudging_l: bool,
    nudging_r: bool,
}

impl Player {
    pub fn new(pos: Vec2) -> Self {
        Self {
            state: State::Standing,
            pos,
            vel: Vec2::ZERO,
            nudging_l: false,
            nudging_r: false,
        }
    }

    pub fn update(&mut self) {
        if self.state == State::Standing {
            if is_key_pressed(KEY_JUMP) {
                self.vel.y = -1.8;
            }
        }
    }

    pub fn physics_update(&mut self, level: &mut Level, resources: &Resources) {
        let gravity = match is_key_down(KEY_JUMP) {
            true  => GRAVITY * 0.7,
            false => GRAVITY,
        };

        self.vel.y = (self.vel.y + gravity).min(MAX_FALL_SPEED);
        self.pos += self.vel;

        let mut movement = Vec2::ZERO;
        if is_key_down(KeyCode::A) && !self.nudging_l { movement.x -= 1.0 }
        if is_key_down(KeyCode::D) && !self.nudging_r { movement.x += 1.0 }
        if is_key_down(KeyCode::W) { movement.y -= 1.0 }
        if is_key_down(KeyCode::S) { movement.y += 1.0 }
        self.pos += movement / 120.0 * 16.0 * 5.0;
        
        let moving_up = self.vel.y < 0.0;

        // Handling sides
        // We only want to push the top sides, and we only want to do it if the player is moving up
        self.nudging_l = collision_left(&mut self.pos,  SIDE_LT, moving_up, &level, resources);
        collision_left(&mut self.pos,  SIDE_LB, false, &level, resources);
        self.nudging_r = collision_right(&mut self.pos, SIDE_RT, moving_up, &level, resources);
        collision_right(&mut self.pos, SIDE_RB, false, &level, resources);

        // If we're moving up, handle the head
        if moving_up {
            let t = collision_top(&mut self.pos, HEAD, level, resources);
            if t {
                level.hit_tile_at_pos(self.pos + HEAD - 1.0, super::level::tile::TileHitKind::Hard, resources);
                self.vel.y = 0.0;
            }
        }
        // Otherwise handle the feet
        else {
            let l = collision_bottom(&mut self.pos, FOOT_L, level, resources);
            let r = collision_bottom(&mut self.pos, FOOT_R, level, resources);

            if l || r {
                self.vel.y = 0.0;
            }
        }
    }

    pub fn draw(&self, camera_pos: Vec2, resources: &Resources) {
        render_text(&format!("pos: [{:8.3}, {:8.3}]", self.pos.x, self.pos.y), RED, vec2(10.0, 50.0), Vec2::ONE, crate::text_renderer::Align::End, crate::text_renderer::Font::Small, resources);
        render_text(&format!("vel: [{:8.3}, {:8.3}]", self.vel.x, self.vel.y), RED, vec2(10.0, 60.0), Vec2::ONE, crate::text_renderer::Align::End, crate::text_renderer::Font::Small, resources);

        // draw_rect(Rect::new(self.pos.x, self.pos.y, 16.0, 16.0).offset(-camera_pos), Color::from_rgba(255, 0, 0, 128));

        resources.draw_rect(self.pos - vec2(0.0, 16.0), Rect::new(96.0, 0.0, 16.0, 32.0), WHITE, resources.entity_atlas());

        for (point, col) in [
            (HEAD, BLUE),
            (SIDE_LT, RED),
            (SIDE_LB, BLUE),
            (SIDE_RT, RED),
            (SIDE_RB, BLUE),
            (FOOT_L, GREEN),
            (FOOT_R, GREEN),
        ] {
            let pos = (self.pos + point - camera_pos).floor();
            draw_circle(pos.x, pos.y, 1.0, col);
        }
    }
}