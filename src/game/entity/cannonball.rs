use macroquad::{color::WHITE, math::{Rect, Vec2}};

use crate::game::scene::{GRAVITY, MAX_FALL_SPEED};

use super::{Entity, EntityKind, Id};

pub struct Cannonball {
    id: Id,
    pos: Vec2,
    vel: Vec2,
    stomped: Option<f32>,
}

impl Cannonball {
    pub fn new(pos: Vec2, vel: Vec2, id: Id) -> Self {
        Self { id, pos, vel, stomped: None }
    }

    fn kill(&mut self) -> bool {
        if self.stomped.is_none() {
            self.stomped = Some(0.0);
            self.vel = Vec2::ZERO;
            return true;
        }
        false
    }
}

impl Entity for Cannonball {
    fn id(&self) -> Id {
        self.id
    }
    fn kind(&self) -> EntityKind {
        EntityKind::Cannonball
    }
    fn hitbox(&self) -> macroquad::prelude::Rect {
        Rect::new(self.pos.x, self.pos.y, 14.0, 14.0)
    }
    fn hurtbox(&self) -> Option<Rect> {
        Some(self.hitbox())
    }
    fn stompbox(&self) -> Option<Rect> {
        Some(Rect::new(self.pos.x - 1.0, self.pos.y, 16.0, 7.0))
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
        matches!(self.stomped, Some(t) if t >= 3.0)
    }

    fn can_hurt(&self) -> bool {
        self.stomped.is_none()
    }
    fn can_stomp(&self) -> bool {
        self.stomped.is_none()
    }
    fn can_stomp_when_player_invuln(&self) -> bool {
        true
    }
    fn kill(&mut self) {
        self.kill();
    }
    fn stomp(&mut self, _power: Option<crate::game::player::FeetPowerup>, _dir: crate::game::player::Dir) -> bool {
        self.kill()
    }
    fn hit_with_throwable(&mut self, _vel: Vec2) -> bool {
        self.kill()
    }

    fn physics_update(&mut self, _player: &mut crate::game::player::Player, _others: &mut Vec<&mut Box<dyn Entity>>, _entity_spawner: &mut crate::game::scene::entity_spawner::EntitySpawner, _particles: &mut crate::game::scene::particles::Particles, _level: &mut crate::game::level::Level, _camera: &mut crate::game::scene::camera::Camera, _resources: &crate::resources::Resources) {
        if let Some(t) = &mut self.stomped {
            *t -= 1.0/120.0;
            self.vel.y = (self.vel.y + GRAVITY).min(MAX_FALL_SPEED);
        }
        
        self.pos += self.vel;
    }

    fn draw(&self, _player: &crate::game::player::Player, camera_pos: Vec2, resources: &crate::resources::Resources) {
        resources.draw_rect(self.pos - camera_pos, Rect::new(128.0, 96.0, 14.0, 14.0), false, false, WHITE, resources.entity_atlas());
    }
}