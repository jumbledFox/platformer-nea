use std::f32::consts::PI;

use macroquad::{color::WHITE, math::{vec2, Rect, Vec2}, rand::gen_range};

use crate::{game::scene::{camera::Camera, particles::{ParticleKind, Particles}}, resources::Resources, util::rect};

use super::{Entity, EntityKind, Id};

const SIZE: Vec2 = vec2(48.0, 40.0);

const TIME: f32 = 0.25;

pub struct Explosion {
    center: Vec2,
    timer: f32,
    id: Id,
}

impl Explosion {
    pub fn new(center: Vec2, id: Id) -> Self {
        Self {
            center: center - SIZE / 2.0,
            timer: 0.0,
            id,
        }
    }
}

impl Entity for Explosion {
    fn id(&self) -> Id {
        self.id
    }
    fn kind(&self) -> EntityKind {
        EntityKind::Explosion
    }
    fn hitbox(&self) -> Rect {
        rect(self.center - SIZE / 2.0, SIZE)
    }
    fn pos(&self) -> Vec2 {
        self.center
    }
    fn vel(&self) -> Vec2 {
        Vec2::ZERO
    }
    fn set_pos(&mut self, _pos: Vec2) {}
    fn set_vel(&mut self, _vel: Vec2) {}

    fn should_destroy(&self) -> bool {
        self.timer >= TIME
    }

    fn physics_update(&mut self, _player: &crate::game::player::Player, _others: &mut Vec<&mut Box<dyn Entity>>, _entity_spawner: &mut crate::game::scene::entity_spawner::EntitySpawner, particles: &mut Particles, level: &mut crate::game::level::Level, camera: &mut Camera, resources: &Resources) {
        if self.timer != 0.0 {
            self.timer += 1.0 / 120.0;
            return;
        }
        self.timer += 1.0 / 120.0;

        camera.shake(1.5);
        particles.add_particle(self.center, Vec2::ZERO, ParticleKind::Explosion);
        for i in 0..8 {
            let angle = Vec2::from_angle((PI * 2.0 / 8.0) * i as f32);
            particles.add_particle(self.center + angle * 16.0, angle * gen_range(0.4, 0.9), ParticleKind::ExplosionSmoke);
        }
    }

    fn draw(&self, camera_pos: Vec2, resources: &Resources) {

    }
}