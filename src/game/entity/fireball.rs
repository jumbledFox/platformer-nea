use macroquad::{color::WHITE, math::{Rect, Vec2}};

use crate::game::scene::{GRAVITY, MAX_FALL_SPEED};

use super::{Entity, EntityKind, Id};

pub struct Fireball {
    id: Id,
    pos: Vec2,
    vel: Vec2,
}

impl Fireball {
    pub fn new(pos: Vec2, vel: Vec2, id: Id) -> Self {
        Self { id, pos, vel }
    }
}

impl Entity for Fireball {
    fn id(&self) -> Id {
        self.id
    }
    fn kind(&self) -> EntityKind {
        EntityKind::Fireball
    }
    fn hitbox(&self) -> Rect {
        Rect::new(self.pos.x, self.pos.y, 16.0, 16.0)
    }
    fn hurtbox(&self) -> Option<Rect> {
        Some(self.hitbox())
    }
    fn pos(&self) -> Vec2 {
        self.pos
    }
    fn vel(&self) -> Vec2 {
        self.vel
    }
    fn set_pos(&mut self, pos: Vec2) {
        self.pos = pos;
    }
    fn set_vel(&mut self, vel: Vec2) {
        self.vel = vel;
    }
    fn should_destroy(&self) -> bool {
        false
    }
    fn destroy_offscreen(&self) -> bool {
        true
    }
    fn update_far(&self) -> bool {
        true
    }

    fn can_hurt(&self) -> bool {
        true
    }
    
    fn physics_update(&mut self, _player: &mut crate::game::player::Player, _others: &mut Vec<&mut Box<dyn Entity>>, _entity_spawner: &mut crate::game::scene::entity_spawner::EntitySpawner, _particles: &mut crate::game::scene::particles::Particles, _level: &mut crate::game::level::Level, _camera: &mut crate::game::scene::camera::Camera, _resources: &crate::resources::Resources) {
        self.vel.y = (self.vel.y + GRAVITY * 0.4).min(MAX_FALL_SPEED);
        self.pos += self.vel;
    }
    fn draw(&self, _player: &crate::game::player::Player, camera_pos: Vec2, resources: &crate::resources::Resources) {
        let x_offset = if resources.tile_animation_timer() % 0.1 < 0.05 { 16.0 } else { 0.0 };
        let y_offset = if self.vel.x == 0.0 { 0.0 } else { 16.0 };
        let rect = Rect::new(96.0 + x_offset, 96.0 + y_offset, 16.0, 16.0);

        let flip_x = self.vel.x < 0.0;
        let flip_y = self.vel.x == 0.0 && self.vel.y > 0.0;
        resources.draw_rect(self.pos - camera_pos, rect, flip_x, flip_y, WHITE, resources.entity_atlas());
    }
}