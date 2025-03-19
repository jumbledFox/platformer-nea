use macroquad::{color::WHITE, math::{vec2, Rect, Vec2}};

use crate::{resources::Resources, util::rect, VIEW_SIZE};

use super::{camera::Camera, GRAVITY, MAX_FALL_SPEED};

const ATLAS_ORIGIN: Vec2 = vec2(288.0, 80.0);

#[derive(Clone, Copy)]
pub enum CrateParticleKind {
    Tl, Tr, Bl, Br,
    Straight1, Straight2,
    Diag1, Diag2,
}

#[derive(Clone, Copy)]
pub enum ParticleKind {
    Crate(CrateParticleKind),
}

struct Particle {
    pos: Vec2,
    vel: Vec2,
    kind: ParticleKind
}

impl Particle {
    pub fn update(&mut self) {
        self.vel.y = (self.vel.y + GRAVITY).min(MAX_FALL_SPEED);
        self.pos += self.vel;
    }

    pub fn draw(&self, camera_pos: Vec2, resources: &Resources) {
        let pos = match self.kind {
            ParticleKind::Crate(CrateParticleKind::Tl) => vec2(0.0, 0.0) * 12.0,
            ParticleKind::Crate(CrateParticleKind::Tr) => vec2(1.0, 0.0) * 12.0,
            ParticleKind::Crate(CrateParticleKind::Bl) => vec2(2.0, 0.0) * 12.0,
            ParticleKind::Crate(CrateParticleKind::Br) => vec2(3.0, 0.0) * 12.0,
            ParticleKind::Crate(CrateParticleKind::Straight1) => vec2(0.0, 12.0),
            ParticleKind::Crate(CrateParticleKind::Straight2) => vec2(8.0, 12.0),
            ParticleKind::Crate(CrateParticleKind::Diag1) => vec2(0.0, 16.0),
            ParticleKind::Crate(CrateParticleKind::Diag2) => vec2(9.0, 16.0),
        };
        let size = match self.kind {
            ParticleKind::Crate(CrateParticleKind::Tl) | 
            ParticleKind::Crate(CrateParticleKind::Tr) | 
            ParticleKind::Crate(CrateParticleKind::Bl) | 
            ParticleKind::Crate(CrateParticleKind::Br) => vec2(12.0, 12.0),
            ParticleKind::Crate(CrateParticleKind::Straight1) => vec2( 8.0, 4.0),
            ParticleKind::Crate(CrateParticleKind::Straight2) => vec2(11.0, 4.0),
            ParticleKind::Crate(CrateParticleKind::Diag1) |
            ParticleKind::Crate(CrateParticleKind::Diag2) => vec2(8.0, 8.0),
        };
        resources.draw_rect(self.pos - camera_pos, rect(pos, size).offset(ATLAS_ORIGIN), false, false, WHITE, resources.entity_atlas());
    }
}

pub struct Particles {
    particles: Vec<Particle>,
}

impl Default for Particles {
    fn default() -> Self {
        Self {
            particles: Vec::with_capacity(32),
        }
    }
}

impl Particles {
    pub fn add_particle(&mut self, pos: Vec2, vel: Vec2, kind: ParticleKind) {
        self.particles.push(Particle { pos, vel, kind });
    }

    pub fn update(&mut self, camera: &Camera) {
        for p in &mut self.particles {
            p.update();
        }
        // TODO: Remove off-screen particles
    }

    pub fn draw(&self, camera_pos: Vec2, resources: &Resources) {
        for p in &self.particles {
            p.draw(camera_pos, resources);
        }
    }
}