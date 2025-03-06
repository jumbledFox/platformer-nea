use macroquad::{color::{Color, WHITE}, math::{vec2, Rect, Vec2}};

use crate::{game::{collision::{collision_bottom, collision_left, collision_right, collision_top}, level::{tile::{LockColor, RAINBOW_LOCK_FRAME_DUR}, Level}, scene::{entity_spawner::EntitySpawner, particles::Particles, GRAVITY, MAX_FALL_SPEED}}, level_pack_data::LevelPosition, resources::Resources};

use super::{Entity, Id};

const TOP:     Vec2 = vec2( 8.0,  0.0);
const SIDE_LT: Vec2 = vec2( 0.0,  2.0);
const SIDE_LB: Vec2 = vec2( 0.0, 12.0);
const SIDE_RT: Vec2 = vec2(16.0,  2.0);
const SIDE_RB: Vec2 = vec2(16.0, 12.0);
const BOT_L:   Vec2 = vec2( 5.0, 14.0);
const BOT_R:   Vec2 = vec2(11.0, 14.0);

pub struct Key {
    id: Id,
    pos: Vec2,
    vel: Vec2,
    color: LockColor,
}

impl Key {
    pub fn new(color: LockColor, pos: Vec2, vel: Vec2, id: Id) -> Self {
        Self { pos, vel, color, id }
    }
    pub fn hitbox() -> Rect {
        Rect::new(0.0, 0.0, 16.0, 14.0)
    }
    pub fn tile_offset() -> Vec2 {
        vec2(0.0, 1.0)
    }

    pub fn draw_editor(key_color: LockColor, pos: Vec2, camera_pos: Vec2, color: Color, resources: &Resources) {
        Self::draw(key_color, pos, camera_pos, color, resources);
    }

    fn draw(key_color: LockColor, pos: Vec2, camera_pos: Vec2, color: Color, resources: &Resources) {
        let sprite = match key_color {
            LockColor::Rainbow => ((resources.tile_animation_timer() % (RAINBOW_LOCK_FRAME_DUR * 4.0)) / RAINBOW_LOCK_FRAME_DUR).floor() as usize,
            c @ _ => c as usize,
        };
        let rect = Rect::new(256.0, sprite as f32 * 14.0, 16.0, 14.0);
        resources.draw_rect(pos - camera_pos, rect, false, color, resources.entity_atlas());
    }
}

impl Entity for Key {
    fn id(&self) -> Id { self.id }
    fn hitbox(&self) -> Rect { Self::hitbox().offset(self.pos) }
    fn hold_offset(&self) -> Option<Vec2> { Some(vec2(0.0, 3.0)) }
    fn throw(&mut self, vel: Vec2) {
        self.vel = vel;
    }
    fn set_pos(&mut self, pos: Vec2) { self.pos = pos; }
    fn set_vel(&mut self, vel: Vec2) { self.vel = vel; }
    fn should_destroy(&self) -> bool { false }

    fn update(&mut self, _resources: &Resources) {

    }

    fn physics_update(&mut self, _entity_spawner: &mut EntitySpawner, _particles: &mut Particles, level: &mut Level, resources: &Resources) {
        self.vel.y = (self.vel.y + GRAVITY).min(MAX_FALL_SPEED);
        self.pos += self.vel;

        let moving_up = self.vel.y < 0.0;
        // Top/bottom
        if moving_up {
            // Top
            if collision_top(&mut self.pos, TOP, level, resources) {
                self.vel.y = 0.0;
            }
        } else {
            // Bottom
            if collision_bottom(&mut self.pos, BOT_L, level, resources)
            || collision_bottom(&mut self.pos, BOT_R, level, resources) {
                self.vel.y = 0.0;
                self.vel.x = 0.0;
            }
        }
        // Sides
        let sl = collision_left(&mut self.pos, SIDE_LT, true, level, resources)
        ||       collision_left(&mut self.pos, SIDE_LB, true, level, resources);
        let sr = collision_right(&mut self.pos, SIDE_RT, true, level, resources)
        ||       collision_right(&mut self.pos, SIDE_RB, true, level, resources);
        if sl && self.vel.x < 0.0 || sr && self.vel.x > 0.0 {
            self.vel.x = 0.0;
        }
    }

    fn draw(&self, camera_pos: Vec2, resources: &Resources) {
        Self::draw(self.color, self.pos, camera_pos, WHITE, resources);
    }
}