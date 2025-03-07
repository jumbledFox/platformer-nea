use macroquad::{color::{Color, WHITE}, math::{vec2, Rect, Vec2}};

use crate::{game::{level::Level, player::Player, scene::{entity_spawner::EntitySpawner, particles::Particles}}, level_pack_data::LevelPosition, resources::Resources};

use super::{Entity, EntityKind, Id};

pub struct Goat {
    id: Id,
    pos: Vec2,
    vel: Vec2,
    arm: Arm,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Arm {
    Down, Up
}

impl Goat {
    pub fn new(pos: Vec2, vel: Vec2, id: Id) -> Self {
        Self { id, pos, vel, arm: Arm::Down }
    }

    pub fn hitbox() -> Rect {
        Rect::new(3.0, 4.0, 8.0, 28.0)
    }
    pub fn tile_offset() -> Vec2 {
        vec2(1.0, -16.0)
    }

    pub fn draw_editor(pos: Vec2, camera_pos: Vec2, color: Color, resources: &Resources) {
        Self::draw(false, Arm::Down, pos, camera_pos, color, resources);
    }
    pub fn object_selector_rect() -> Rect {
        Rect::new(0.0, 0.0, 14.0, 32.0)
    }

    fn draw(step: bool, arm: Arm, pos: Vec2, camera_pos: Vec2, color: Color, resources: &Resources) {
        let offset = match step {
            false => Vec2::ZERO,
            true => vec2(0.0, -1.0)
        };
        // Draw the body
        let body_x = if step { 15.0 } else { 0.0 };
        resources.draw_rect(pos + offset - camera_pos, Rect::new(body_x, 82.0, 14.0, 32.0), false, color, resources.entity_atlas());
        // Draw the arm
        let arm_x = if arm == Arm::Down { 30.0 } else { 48.0 };
        resources.draw_rect(pos + offset + vec2(0.0, 6.0) - camera_pos, Rect::new(arm_x, 82.0, 17.0, 20.0), false, color, resources.entity_atlas());
    }
}

impl Entity for Goat {
    fn id(&self) -> Id {
        self.id
    }
    fn kind(&self) -> EntityKind {
        EntityKind::Goat
    }
    fn hitbox(&self) -> Rect {
        Self::hitbox()
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

    fn update(&mut self, _resources: &Resources) {
        
    }
    fn physics_update(&mut self, _player: &Player, others: &mut Vec<&mut Box<dyn Entity>>, _entity_spawner: &mut EntitySpawner, _particles: &mut Particles, level: &mut Level, resources: &Resources) {
        
    }
    fn draw(&self, camera_pos: Vec2, resources: &Resources) {
        Self::draw(false, self.arm, self.pos, camera_pos, WHITE, resources);
    }
}