use macroquad::{color::{Color, WHITE}, math::{vec2, Vec2}, rand::gen_range};

use crate::{game::{level::tile::LockColor, player::PowerupKind}, resources::Resources, text_renderer::{render_text, Align, Font}, util::rect};

use super::{camera::Camera, GRAVITY, MAX_FALL_SPEED};

const ATLAS_ORIGIN: Vec2 = vec2(288.0, 80.0);

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CrateParticleKind {
    Tl, Tr, Bl, Br,
    Straight1, Straight2,
    Diag1, Diag2,
}

#[derive(Clone, Copy, PartialEq)]
pub enum SmokeKind {
    Lock(LockColor),
    Color(Color),
}

#[derive(Clone, Copy, PartialEq)]
pub enum ParticleKind {
    Crate(CrateParticleKind),
    ExplosionSmoke,
    Explosion,
    Sparkle(Color),
    OneUp,
    Stone(usize),
    Powerup(PowerupKind),
    Smoke(SmokeKind, f32),
}

impl Eq for ParticleKind {
    
}

impl Ord for ParticleKind {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        fn order(kind: &ParticleKind) -> u8 {
            match kind {
                ParticleKind::Crate(_) => 0,
                ParticleKind::Stone(_) => 0,
                ParticleKind::Smoke(..) => 0,
                ParticleKind::ExplosionSmoke => 1,
                ParticleKind::Sparkle(_) => 2,
                ParticleKind::Explosion => 2,
                ParticleKind::Powerup(_) => 3,
                ParticleKind::OneUp => 4,
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
        if matches!(self.kind, ParticleKind::Crate(_) | ParticleKind::Stone(_)) {
            self.vel.y = (self.vel.y + GRAVITY).min(MAX_FALL_SPEED);
        }
        if matches!(self.kind, ParticleKind::Smoke(..)) {
            self.vel *= 0.99;
        }

        self.pos += self.vel;
        self.timer += 1.0/120.0;
    }

    pub fn draw(&self, camera_pos: Vec2, resources: &Resources) {
        if let ParticleKind::Powerup(kind) = self.kind {
            let time_left = (1.0 - self.timer / 2.0).clamp(0.0, 1.0);

            let color = kind.text_color();
            let color = Color::new(color.r, color.g, color.b, time_left);

            let t = self.timer * 10.0;
            let stretch = vec2(t.sin(), t.cos()) / 9.0 + 1.0;
            let size = stretch * time_left;

            render_text(kind.name(), color, self.pos - camera_pos, size, Align::Mid, Font::Large, resources);

            return;
        }

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
            ParticleKind::Sparkle(_) if self.timer >= 0.3  => vec2(63.0, 32.0),
            ParticleKind::Sparkle(_) if self.timer >= 0.2  => vec2(58.0, 32.0),
            ParticleKind::Sparkle(_) if self.timer >= 0.1  => vec2(53.0, 32.0),
            ParticleKind::Sparkle(_) => vec2(48.0, 32.0),
            ParticleKind::OneUp => vec2(48.0, 0.0),
            ParticleKind::Stone(i) => vec2(i as f32 * 16.0, 48.0),
            ParticleKind::Smoke(_, l) if self.timer >= l * 4.0 / 5.0 => vec2(64.0, 64.0),
            ParticleKind::Smoke(_, l) if self.timer >= l * 3.0 / 5.0 => vec2(48.0, 64.0),
            ParticleKind::Smoke(_, l) if self.timer >= l * 2.0 / 5.0 => vec2(32.0, 64.0),
            ParticleKind::Smoke(_, l) if self.timer >= l * 1.0 / 5.0 => vec2(16.0, 64.0),
            ParticleKind::Smoke(_, _) => vec2(0.0, 64.0),
            _ => Vec2::ZERO,
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
            ParticleKind::Sparkle(_) => vec2(5.0, 9.0),
            ParticleKind::OneUp => vec2(15.0, 7.0),
            ParticleKind::Stone(_) => vec2(16.0, 16.0),
            ParticleKind::Smoke(..) => vec2(16.0, 16.0),
            _ => Vec2::ZERO,
        };

        let color = match self.kind {
            ParticleKind::ExplosionSmoke => Color::new(1.0, 1.0, 1.0, (1.0 - self.timer * 1.5).clamp(0.0, 1.0)),
            ParticleKind::Sparkle(color) => color,
            ParticleKind::OneUp => Color::new(1.0, 1.0, 1.0, (2.0 - self.timer).clamp(0.0, 1.0)),
            ParticleKind::Smoke(kind, l) => {
                let col = match kind {
                    SmokeKind::Color(c) => c,
                    SmokeKind::Lock(lock_color) => lock_color.color(resources),
                };
                Color::new(col.r, col.g, col.b, (1.0 - self.timer / l).clamp(0.0, 1.0))
            },
            _ => WHITE,
        };

        let (flip_x, flip_y) = match self.kind {
            ParticleKind::Explosion => (
                self.timer.rem_euclid(0.2) > 0.1,
                (self.timer + 0.05).rem_euclid(0.2) > 0.1,
            ),
            ParticleKind::Crate(_) |
            ParticleKind::OneUp    |
            ParticleKind::Stone(_) => (false, false),
            _ => (self.flip_x, self.flip_y),
        };

        let draw_pos = match self.kind {
            ParticleKind::OneUp => vec2((self.timer * 6.0).sin() * 5.0, 0.0),
            _ => Vec2::ZERO,
        } + self.pos - size / 2.0;

        resources.draw_rect(draw_pos - camera_pos, rect(pos, size).offset(ATLAS_ORIGIN), flip_x, flip_y, color, resources.entity_atlas());
    }

    pub fn should_remove(&self) -> bool {
        match self.kind {
            ParticleKind::ExplosionSmoke => self.timer >= 2.0,
            ParticleKind::Explosion  => self.timer >= 0.25,
            ParticleKind::Sparkle(_) => self.timer >= 0.4,
            ParticleKind::OneUp      => self.timer >= 2.0,
            ParticleKind::Powerup(_) => self.timer >= 2.0,
            ParticleKind::Smoke(_, l) => self.timer >= l,
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
    fn sort(&mut self) {
        self.particles.sort_by(|a, b| a.kind.cmp(&b.kind));
    }

    pub fn add_particle(&mut self, pos: Vec2, vel: Vec2, kind: ParticleKind) {
        self.particles.push(Particle { pos, vel, kind, timer: 0.0, flip_x: gen_range(0, 2) == 0, flip_y: gen_range(0, 2) == 0 });
        self.sort();
    }

    pub fn add_powerup(&mut self, pos: Vec2, kind: PowerupKind) {
        self.particles.push(Particle { pos, vel: vec2(0.0, -0.4), kind: ParticleKind::Powerup(kind), timer: 0.0, flip_x: false, flip_y: false });
        self.sort();
    }

    pub fn add_stone_block(&mut self, pos: Vec2) {
        self.add_particle(pos, vec2(gen_range(-1.0, -0.8), gen_range(-0.7, -1.0)), ParticleKind::Stone(0));
        self.add_particle(pos, vec2(gen_range( 1.0,  0.8), gen_range(-0.7, -1.0)), ParticleKind::Stone(1));
        self.add_particle(pos, vec2(gen_range(-1.0, -0.8), gen_range(-0.4, -0.6)), ParticleKind::Stone(2));
        self.add_particle(pos, vec2(gen_range( 1.0,  0.8), gen_range(-0.4, -0.6)), ParticleKind::Stone(3));
        self.sort();
    }

    pub fn add_smoke_cloud(&mut self, pos: Vec2, kind: SmokeKind) {
        let vel = |x: f32, y: f32| -> Vec2 {
            vec2(x * gen_range(0.2, 0.5), y * gen_range(0.2, 0.5))
        };
        let l = || -> f32 {
            gen_range(0.3, 0.7)
        };
        self.add_particle(pos, vel( 1.0,  1.0), ParticleKind::Smoke(kind, l()));
        self.add_particle(pos, vel(-1.0,  1.0), ParticleKind::Smoke(kind, l()));
        self.add_particle(pos, vel( 1.0, -1.0), ParticleKind::Smoke(kind, l()));
        self.add_particle(pos, vel(-1.0, -1.0), ParticleKind::Smoke(kind, l()));
        self.sort();
    }

    pub fn add_lock(&mut self, pos: Vec2, color: LockColor) {
        self.add_smoke_cloud(pos, SmokeKind::Lock(color));
    }

    pub fn update(&mut self, camera: &Camera) {
        for p in &mut self.particles {
            p.update();
        }
        for i in (0..self.particles.len()).rev() {
            if self.particles[i].should_remove() || !camera.on_screen(self.particles[i].pos) {
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