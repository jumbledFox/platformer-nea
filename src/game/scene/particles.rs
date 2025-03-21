use macroquad::{color::Color, math::{vec2, Vec2}, rand::gen_range};

use crate::{resources::Resources, util::rect};

use super::{camera::Camera, GRAVITY, MAX_FALL_SPEED};

const ATLAS_ORIGIN: Vec2 = vec2(288.0, 80.0);

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CrateParticleKind {
    Tl, Tr, Bl, Br,
    Straight1, Straight2,
    Diag1, Diag2,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ParticleKind {
    Crate(CrateParticleKind),
    ExplosionSmoke,
    Explosion,
}

impl Ord for ParticleKind {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        fn order(kind: &ParticleKind) -> u8 {
            match kind {
                ParticleKind::Crate(_) => 0,
                ParticleKind::ExplosionSmoke => 1,
                ParticleKind::Explosion => 2,
            }
        }
        order(self).cmp(&order(other))
    }
}
impl PartialOrd for ParticleKind {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}


struct Particle {
    pos: Vec2,
    vel: Vec2,
    kind: ParticleKind,
    timer: f32,

    flip_x: bool,
    flip_y: bool,
}

impl Particle {
    pub fn update(&mut self) {
        if matches!(self.kind, ParticleKind::Crate(_)) {
            self.vel.y = (self.vel.y + GRAVITY).min(MAX_FALL_SPEED);
        }
        if self.kind == ParticleKind::ExplosionSmoke {

        }

        self.pos += self.vel;
        self.timer += 1.0/120.0;
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
            ParticleKind::ExplosionSmoke => vec2(80.0, 0.0),
            ParticleKind::Explosion => vec2(80.0, 32.0),
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
            ParticleKind::ExplosionSmoke => vec2(41.0, 29.0),
            ParticleKind::Explosion => vec2(48.0, 40.0),
        };

        let alpha = match self.kind {
            ParticleKind::ExplosionSmoke => (1.0 - self.timer * 1.5).clamp(0.0, 1.0),
            _ => 1.0
        };

        let (flip_x, flip_y) = match self.kind {
            ParticleKind::Explosion => (
                self.timer.rem_euclid(0.2) > 0.1,
                (self.timer + 0.05).rem_euclid(0.2) > 0.1,
            ),
            ParticleKind::Crate(_) => (false, false),
            _ => (self.flip_x, self.flip_y),
        };

        let draw_pos = match self.kind {
            _ => self.pos - size / 2.0
        };

        resources.draw_rect(draw_pos - camera_pos, rect(pos, size).offset(ATLAS_ORIGIN), flip_x, flip_y, Color::new(1.0, 1.0, 1.0, alpha), resources.entity_atlas());
    }

    pub fn should_remove(&self) -> bool {
        match self.kind {
            ParticleKind::Explosion => self.timer >= 0.25,
            _ => false,
        }
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
        self.particles.push(Particle { pos, vel, kind, timer: 0.0, flip_x: gen_range(0, 2) == 0, flip_y: gen_range(0, 2) == 0 });
        self.particles.sort_by(|a, b| a.kind.cmp(&b.kind));
    }

    pub fn update(&mut self, camera: &Camera) {
        for p in &mut self.particles {
            p.update();
        }
        // TODO: Remove off-screen particles
        for i in (0..self.particles.len()).rev() {
            if self.particles[i].should_remove() /* || off screen */ {
                self.particles.remove(i);
            }
        }
    }

    pub fn draw(&self, camera_pos: Vec2, resources: &Resources) {
        for p in &self.particles {
            p.draw(camera_pos, resources);
        }
    }
}