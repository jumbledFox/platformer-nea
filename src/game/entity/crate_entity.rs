// I had to name the file 'crate_entity' because crate is a reserved keyword in rust lololol :3

use macroquad::{color::{Color, WHITE}, input::is_key_pressed, math::{vec2, Rect, Vec2}, rand::gen_range};

use crate::{game::{collision::{collision_bottom, collision_left, collision_right, collision_top, default_collision}, level::{tile::{LockColor, TileHitKind}, Level}, scene::{entity_spawner::EntitySpawner, particles::{CrateParticleKind, ParticleKind, Particles}, GRAVITY, MAX_FALL_SPEED}}, level_pack_data::{level_pos_to_pos, LevelPosition}, resources::Resources};

use super::{frog::Frog, key::Key, Entity, EntityKind, Id};

const TOP:     Vec2 = vec2( 8.0,  0.0);
const SIDE_LT: Vec2 = vec2( 0.0,  2.0);
const SIDE_LB: Vec2 = vec2( 0.0, 14.0);
const SIDE_RT: Vec2 = vec2(16.0,  2.0);
const SIDE_RB: Vec2 = vec2(16.0, 14.0);
const BOT_L:   Vec2 = vec2( 4.0, 16.0);
const BOT_R:   Vec2 = vec2(12.0, 16.0);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CrateKind {
    Frog(bool), // false for few, true for many
    Chip(bool), // ditto
    Life,
    Key(LockColor),
}

pub struct Crate {
    id: Id,
    pos: Vec2,
    vel: Vec2,
    kind: CrateKind,
    hit: bool,
}

impl Crate {
    pub fn new(kind: CrateKind, pos: Vec2, vel: Vec2, id: Id) -> Self {
        Self {
            id,
            pos,
            vel,
            kind,
            hit: false,
        }
    }

    pub fn draw(pos: Vec2, camera_pos: Vec2, color: Color, resources: &Resources) {
        resources.draw_rect(pos - camera_pos, Rect::new(160.0, 0.0, 16.0, 16.0), false, color, resources.entity_atlas());
    }

    pub fn hitbox() -> Rect {
        Rect::new(0.0, 0.0, 16.0, 16.0)
    }
}

impl Entity for Crate {
    fn id(&self) -> Id {
        self.id
    }
    fn hitbox(&self) -> Rect {
        Self::hitbox().offset(self.pos)
    }
    fn hold_offset(&self) -> Option<Vec2> {
        Some(Vec2::ZERO)
    }
    fn throw(&mut self, vel: Vec2) {
        self.vel = vel;
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

    fn update(&mut self,  _resources: &Resources) {
        if is_key_pressed(macroquad::input::KeyCode::G) {
            self.vel = vec2(1.0, -2.0);
        }
    }

    fn physics_update(&mut self, entity_spawner: &mut EntitySpawner, particles: &mut Particles, level: &mut Level, resources: &Resources) {
        self.vel.y = (self.vel.y + GRAVITY).min(MAX_FALL_SPEED);
        self.pos += self.vel;

        let prev_vel = self.vel;

        let mut tops   = [(TOP, false)];
        let mut bots   = [(BOT_L, false), (BOT_R, false)];
        let mut lefts  = [(SIDE_LT, true, false), (SIDE_LB, true, false)];
        let mut rights = [(SIDE_RT, true, false), (SIDE_RB, true, false)];
        let (t, b, _, _, hit) = default_collision(&mut self.pos, &mut self.vel, Some(TileHitKind::Soft), &mut tops, &mut bots, &mut lefts, &mut rights, level, resources);
        if b { self.vel.x = 0.0; }

        // If we collided and have been thrown... we need to be destroyed and release the entities inside! also hit switches and stuff...
        if hit {
            self.hit = true;

            // Velocities for all the items in the crate
            let (y_min, y_max) = match (t, b) {
                (true, false)  => ( 0.1,  0.2),
                (false, true)  => (-1.0, -0.4),
                (_, _)         => (-1.0,  1.0),
            };
            let (x_min, x_max) = match (t || b, prev_vel.x.abs() > 0.5, prev_vel.x > 0.0) {
                // Hitting top/bottom
                (true, true, true)  => ( 0.1,  1.0),
                (true, true, false) => (-1.0, -0.1),
                (true, false, _) => (-0.5, 0.5),
                // Hitting sides
                (false, _, true)  => (-1.0, -0.5),
                (false, _, false) => ( 0.5,  1.0),
            };
            let mut spawn_entity = |kind: EntityKind| {
                let vel = vec2(gen_range(x_min, x_max), gen_range(y_min, y_max)) * 0.7;
                entity_spawner.add_entity(self.pos, vel, kind, None);
            };
            match self.kind {
                CrateKind::Key(color)  => spawn_entity(EntityKind::Key(color)),
                CrateKind::Frog(false) => spawn_entity(EntityKind::Frog),
                CrateKind::Frog(true)  => for _ in 0..gen_range(2, 3) { spawn_entity(EntityKind::Frog) },
                CrateKind::Chip(false) => spawn_entity(EntityKind::Chip(true)),
                CrateKind::Chip(true)  => for _ in 0..gen_range(2, 4) { spawn_entity(EntityKind::Chip(true)) },
                CrateKind::Life => {
                    spawn_entity(EntityKind::Life);
                    for _ in 0..gen_range(2, 4) { spawn_entity(EntityKind::Chip(true)) }
                },
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
        }
    }

    fn draw(&self, camera_pos: Vec2, resources: &Resources) {
        Self::draw(self.pos, camera_pos, WHITE, resources);
    }
}