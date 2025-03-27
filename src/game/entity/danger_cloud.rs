use macroquad::{color::Color, math::{vec2, Rect, Vec2}, rand::gen_range};

use crate::{game::{level::Level, player::Player, scene::{camera::Camera, entity_spawner::EntitySpawner, particles::Particles}}, resources::Resources, util::rect};

use super::{Entity, EntityKind, Id};

pub struct DangerCloud {
    pos: Vec2,
    vel: Vec2,
    time: f32,
    total_time: f32,
    id: Id,
}

impl DangerCloud {
    pub fn new(pos: Vec2, vel: Vec2, id: Id) -> Self {
        let total_time = gen_range(0.8, 1.6);
        Self { pos, vel, time: total_time, total_time, id }
    }
}

impl Entity for DangerCloud {
    fn id(&self) -> Id {
        self.id
    }
    fn kind(&self) -> EntityKind {
        EntityKind::DangerCloud
    }
    fn hitbox(&self) -> Rect {
        rect(self.pos - 2.0, vec2(12.0, 12.0))
    }
    fn hurtbox(&self) -> Option<Rect> {
        Some(Rect::new(8.0, 8.0, 8.0, 8.0).offset(self.pos))
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
    fn set_vel(&mut self, _vel: Vec2) { }
    fn should_destroy(&self) -> bool {
        self.time <= 0.0
    }
    fn destroy_offscreen(&self) -> bool {
        true
    }
    fn can_hurt(&self) -> bool {
        self.time / self.total_time > 0.65
    }
    fn physics_update(&mut self, _player: &mut Player, _others: &mut Vec<&mut Box<dyn Entity>>, _entity_spawner: &mut EntitySpawner, _particles: &mut Particles, _level: &mut Level, _camera: &mut Camera, _resources: &Resources) {
        self.pos += self.vel;
        self.time -= 1.0 / 120.0;
    }
    fn draw(&self,_player: &Player,  camera_pos: Vec2, resources: &Resources) {
        let color = Color::new(1.0, 0.0, 0.0, self.time / self.total_time);
        resources.draw_rect(self.pos - camera_pos, Rect::new(320.0, 112.0, 16.0, 16.0), false, false, color, resources.entity_atlas());
    }
}