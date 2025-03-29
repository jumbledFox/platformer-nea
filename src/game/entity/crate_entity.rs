// I had to name the file 'crate_entity' because crate is a reserved keyword in rust lololol :3

use macroquad::{color::{Color, WHITE}, input::is_key_pressed, math::{vec2, Rect, Vec2}, rand::gen_range};

use crate::{game::{collision::{default_collision, lava_check, solid_on_off_check, spike_check, EntityHitKind}, level::{tile::{LockColor, TileHitKind}, Level}, player::{HeadPowerup, Player, PowerupKind}, scene::{camera::Camera, entity_spawner::EntitySpawner, particles::{CrateParticleKind, ParticleKind, Particles}, GRAVITY, MAX_FALL_SPEED}}, resources::Resources};

use super::{chip::Chip, frog::Frog, key::Key, powerup::Powerup, Entity, EntityKind, Id};

const TOP:     Vec2 = vec2( 8.0,  0.0);
const SIDE_LT: Vec2 = vec2( 0.0,  2.0);
const SIDE_LB: Vec2 = vec2( 0.0, 14.0);
const SIDE_RT: Vec2 = vec2(16.0,  2.0);
const SIDE_RB: Vec2 = vec2(16.0, 14.0);
const BOT_L:   Vec2 = vec2( 4.0, 16.0);
const BOT_R:   Vec2 = vec2(12.0, 16.0);
const CENTER:  Vec2 = vec2( 8.0,  8.0);

const FUSE_FIZZ: f32 = 1.5;
const FUSE_EXPLODE: f32 = 3.0;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CrateKind {
    Frog(bool), // false for few, true for many
    Chip(bool), // ditto
    Powerup(PowerupKind),
    Life,
    Key(LockColor),
    Explosive,
}

pub struct Crate {
    id: Id,
    pos: Vec2,
    vel: Vec2,
    kind: CrateKind,
    hit: bool,
    hit_non_tile: bool,
    // Used for explosive crates...
    fuse: Option<f32>,

    // Kinda janky way of doing things but it WORKS
    t: bool,
    b: bool,
    prev_vel: Vec2,
}

impl Crate {
    pub fn new(kind: CrateKind, pos: Vec2, vel: Vec2, id: Id) -> Self {
        Self {
            id,
            pos,
            vel,
            kind,
            hit: false,
            hit_non_tile: false,
            fuse: None,

            t: false,
            b: false,
            prev_vel: Vec2::ZERO,
        }
    }

    pub fn draw(kind: Option<CrateKind>, pos: Vec2, explosive: Option<f32>, camera_pos: Vec2, color: Color, resources: &Resources) {
        if let Some(fuse) = explosive {
            // Yeah this is janky but it works!!!!
            let mut frame = if fuse == 0.0 { 0 }
            else if fuse < FUSE_FIZZ { 1 }
            else { 3 };
            if fuse.rem_euclid(0.2) > 0.1 {
                frame += 1;
            }
            if fuse.rem_euclid(0.1) > 0.05 && fuse >= FUSE_FIZZ {
                frame -= 2;
            }
            resources.draw_rect(pos - camera_pos - vec2(4.0, 8.0), Rect::new(160.0 + frame as f32 * 24.0, 112.0, 24.0, 24.0), false, false, color, resources.entity_atlas());
            return;
        }
        resources.draw_rect(pos - camera_pos, Rect::new(160.0, 0.0, 16.0, 16.0), false, false, color, resources.entity_atlas());

        let kind = match kind {
            Some(k) => k,
            None => return,
        };
        let (selector_size, selector_offset) = match kind {
            CrateKind::Chip(_) | CrateKind::Life => (Chip::object_selector_size(), Vec2::ZERO),
            CrateKind::Frog(_) => (Frog::object_selector_rect().size(), Frog::object_selector_rect().point()),
            CrateKind::Powerup(_) => (Powerup::hitbox().size(), Vec2::ZERO),
            CrateKind::Key(_)  => (Key::hitbox().size(), Vec2::ZERO),
            CrateKind::Explosive => (Vec2::ZERO, Vec2::ZERO),
        };
        let center = (pos.floor() + 8.0 - (selector_size/2.0) + selector_offset).ceil();
        // ... and make it semi-transparent
        let color = Color::new(1.0, 1.0, 1.0, 0.8);
        // Draw the object, or if there are multiple, draw multiple.
        match kind {
            CrateKind::Frog(false) => Frog::draw_editor(center - 1.0, camera_pos, color, resources),
            CrateKind::Frog(true) => {
                Frog::draw_editor(center - vec2( 1.0, 3.0), camera_pos, color, resources);
                Frog::draw_editor(center - vec2( 1.0, 0.0), camera_pos, color, resources);
            },
            CrateKind::Powerup(kind) => Powerup::draw_editor(false, kind, pos - vec2(1.0, 0.0), camera_pos, color, resources),
            CrateKind::Chip(false) => Chip::draw_editor(false, false, center + 1.0, camera_pos, color, resources),
            CrateKind::Chip(true) => { 
                Chip::draw_editor(false, false, center + vec2(0.0, 0.0), camera_pos, color, resources);
                Chip::draw_editor(false, false, center + vec2(2.0, 2.0), camera_pos, color, resources);
            },
            CrateKind::Life => Chip::draw_editor(false, true, center + 1.0, camera_pos, color, resources),
            CrateKind::Key(key_color) => Key::draw_editor(key_color, center, camera_pos, color, resources),
            CrateKind::Explosive => {}
        }
    }

    pub fn hitbox() -> Rect {
        Rect::new(0.0, 0.0, 16.0, 16.0)
    }

    pub fn break_crate(&mut self, t: bool, b: bool, prev_vel: Vec2, entity_spawner: &mut EntitySpawner, particles: &mut Particles) {
        // Velocities for all the items in the crate
        let (y_min, y_max) = match (self.hit_non_tile, t, b) {
            // If we hit the bottom of a tile, or we should break, launch entities upwards
            (true,  _,     _) |
            (_,     false, true)  => (-1.0, -0.4),
            // If we hit the top, launch them sliiightly upwards
            (false, true,  false) => (-0.1,  0.0),
            // If we hit the side (not top/bottom) launch in all directions vertically
            (_,     _,     _)     => (-1.0,  1.0),
        };
        let (x_min, x_max) = match (t || b, prev_vel.x.abs() > 0.5, prev_vel.x > 0.0) {
            // Hitting top/bottom
            (true, true, true)  => ( 0.1,  1.0),
            (true, true, false) => (-1.0, -0.1),
            (true, false, _)    => (-0.5,  0.5),
            // Hitting sides
            (false, _, true)  => (-1.0, -0.5),
            (false, _, false) => ( 0.5,  1.0),
        };
        let mut spawn_entity = |kind: EntityKind| {
            let multiplier = match self.kind {
                CrateKind::Key(_)     => 0.7,
                CrateKind::Frog(_)    => 1.0,
                CrateKind::Powerup(_) => 0.7,
                CrateKind::Chip(_)    => 1.0,
                CrateKind::Life       => 0.7,
                CrateKind::Explosive  => 0.0,
            };
            let vel = vec2(gen_range(x_min, x_max), gen_range(y_min, y_max)) * multiplier;
            entity_spawner.add_entity(self.pos + vec2(0.0, 1.0), vel, kind, None);
        };
        match self.kind {
            CrateKind::Key(color)  => spawn_entity(EntityKind::Key(color)),
            CrateKind::Frog(false) => spawn_entity(EntityKind::Frog(true)),
            CrateKind::Frog(true)  => for _ in 0..gen_range(2, 3) { spawn_entity(EntityKind::Frog(true)) },
            CrateKind::Powerup(kind) => {
                for _ in 0..gen_range(2, 4) { spawn_entity(EntityKind::Chip(true)); }
                spawn_entity(EntityKind::Powerup(kind, true, false));
            }
            CrateKind::Chip(false) => spawn_entity(EntityKind::Chip(true)),
            CrateKind::Chip(true)  => for _ in 0..gen_range(2, 4) { spawn_entity(EntityKind::Chip(true)) },
            CrateKind::Life => {
                spawn_entity(EntityKind::Life(true));
                for _ in 0..gen_range(2, 4) { spawn_entity(EntityKind::Chip(true)) }
            },
            CrateKind::Explosive => {}
        }

        // Very ugly code... but it works!
        for (kind, offset, vel_x, vel_y) in [
            (CrateParticleKind::Tl, vec2(0.0, 0.0), vec2(-1.0, -0.5), vec2(-1.0, -0.5)),
            (CrateParticleKind::Tr, vec2(8.0, 0.0), vec2( 0.5,  1.0), vec2(-1.0, -0.5)),
            (CrateParticleKind::Bl, vec2(0.0, 5.0), vec2(-1.0, -0.5), vec2(-0.5, -0.5)),
            (CrateParticleKind::Br, vec2(5.0, 5.0), vec2( 0.5,  1.0), vec2(-0.5, -0.5)),
            (CrateParticleKind::Straight1, vec2(2.0, 5.0), vec2(-1.5,  1.5), vec2(-1.0, -0.5)),
            (CrateParticleKind::Straight1, vec2(5.0, 7.0), vec2(-1.5,  1.5), vec2(-1.0, -0.5)),
            (CrateParticleKind::Straight2, vec2(7.0, 9.0), vec2(-1.5,  1.5), vec2(-1.0, -0.5)),
            (CrateParticleKind::Straight2, vec2(9.0, 2.0), vec2(-1.5,  1.5), vec2(-1.0, -0.5)),
            (CrateParticleKind::Diag1, vec2( 2.0,  0.0), vec2(-1.5,  1.5), vec2(-1.0, -0.5)),
            (CrateParticleKind::Diag1, vec2( 5.0,  5.0), vec2(-1.5,  1.5), vec2(-1.0, -0.5)),
            (CrateParticleKind::Diag2, vec2( 7.0,  2.0), vec2(-1.5,  1.5), vec2(-1.0, -0.5)),
            (CrateParticleKind::Diag2, vec2(13.0, 13.0), vec2(-1.5,  1.5), vec2(-1.0, -0.5)),
        ] {
            let vel = vec2(gen_range(vel_x.x, vel_x.y), gen_range(vel_y.x, vel_y.y));
            particles.add_particle(self.pos + offset, vel, ParticleKind::Crate(kind));
        }

        if self.kind == CrateKind::Explosive {
            entity_spawner.add_entity(self.hitbox().center(), Vec2::ZERO, EntityKind::Explosion, None);
        }
    }

    pub fn update_fuse(&mut self) {
        if let Some(t) = &mut self.fuse {
            *t += 1.0 / 120.0;
            if *t > FUSE_EXPLODE {
                self.hit = true;
            }
        }
    }
}

impl Entity for Crate {
    fn id(&self) -> Id {
        self.id
    }
    fn kind(&self) -> EntityKind {
        EntityKind::Crate(self.kind)
    }
    fn hitbox(&self) -> Rect {
        Self::hitbox().offset(self.pos)
    }
    fn holdbox(&self) -> Option<Rect> {
        Some(self.hitbox())
    }
    fn hold_offset(&self) -> Option<Vec2> {
        Some(Vec2::ZERO)
    }
    fn hold(&mut self) {
        if self.fuse.is_some() {
            return;
        }
        if self.kind == CrateKind::Explosive {
            self.fuse = Some(0.0);
        }
    }
    fn throw(&mut self, vel: Vec2) {
        self.vel = vel;
    }
    fn throw_push_out(&self) -> bool {
        true
    }
    fn should_throw(&self) -> bool {
        self.hit
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
        self.hit
    }
    fn kill(&mut self) {
        self.hit = true;
    }
    fn destroy(&mut self, entity_spawner: &mut EntitySpawner, particles: &mut Particles) {
        self.break_crate(self.t, self.b, self.prev_vel, entity_spawner, particles);
    }
    fn hit_with_throwable(&mut self, vel: Vec2) -> bool {
        self.hit = true;
        self.hit_non_tile = true;
        self.vel = vel;
        true
    }

    fn update(&mut self,  _resources: &Resources) {
        if is_key_pressed(macroquad::input::KeyCode::G) {
            self.vel = vec2(1.0, -2.0);
        }
    }

    fn hold_fixed_update(&mut self) {
        self.update_fuse();
    }

    fn physics_update(&mut self, _player: &mut Player, others: &mut Vec<&mut Box<dyn Entity>>, _entity_spawner: &mut EntitySpawner, particles: &mut Particles, level: &mut Level, _camera: &mut Camera, resources: &Resources) {
        self.update_fuse();
        
        if let CrateKind::Key(color) = self.kind {
            if level.lock_destroyed(color) {
                self.hit = true;
                return;
            }
        }

        self.vel.y = (self.vel.y + GRAVITY).min(MAX_FALL_SPEED);
        self.pos += self.vel;

        self.prev_vel = self.vel;

        let mut tops   = [(TOP, false)];
        let mut bots   = [(BOT_L, false), (BOT_R, false)];
        let mut lefts  = [(SIDE_LT, true, false), (SIDE_LB, true, false)];
        let mut rights = [(SIDE_RT, true, false), (SIDE_RB, true, false)];
        let tile_hit = match self.kind {
            CrateKind::Explosive => None,
            _ => Some(TileHitKind::Soft),
        };
        let entity_hit = Some((EntityHitKind::AllButCrates, self.hitbox(), 1.5, true, true));
        let (t, b, _, _, hit, hit_entity) = default_collision(&mut self.pos, &mut self.vel, tile_hit, entity_hit, others, &mut tops, &mut bots, &mut lefts, &mut rights, particles, level, resources);
        if b { self.vel.x = 0.0; }

        self.t = t;
        self.b = b;

        self.hit |= hit || hit_entity;

        if self.kind == CrateKind::Explosive {
            for e in others {
                if matches!(e.kind(), EntityKind::Cannonball | EntityKind::Fireball) {
                    if e.hitbox().overlaps(&self.hitbox()) {
                        // self.hit = true;
                    }
                }
            }
        }

        // Spikes
        if spike_check(self.pos, &[TOP], &[BOT_L, BOT_R], &[SIDE_LT, SIDE_LB], &[SIDE_RT, SIDE_RB], level).is_some() {
            if !self.hit {
                self.hit_non_tile = true;
            }
            self.hit = true;
        }
        // Lava
        if lava_check(self.pos, &[CENTER], particles, level) {
            self.hit = true;
        }
        // Solid on/off blocks
        if solid_on_off_check(self.pos, &[CENTER], level) {
            self.hit = true;
        }
    }

    fn draw(&self, player: &Player, camera_pos: Vec2, resources: &Resources) {
        let explosive = match self.kind {
            CrateKind::Explosive => Some(self.fuse.unwrap_or(0.0)),
            _ => None,
        };
        let kind = match player.head_powerup() {
            Some(HeadPowerup::XrayGoggles) => Some(self.kind),
            _ => None,
        };
        Self::draw(kind, self.pos, explosive, camera_pos, WHITE, resources);
    }
}