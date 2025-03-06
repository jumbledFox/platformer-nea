// I had to name the file 'crate_entity' because crate is a reserved keyword in rust lololol :3

use macroquad::{color::{Color, WHITE}, input::is_key_pressed, math::{vec2, Rect, Vec2}, rand::gen_range};

use crate::{game::{collision::{collision_bottom, collision_left, collision_right, collision_top}, level::{tile::{LockColor, TileHitKind}, Level}, scene::{particles::{CrateParticleKind, ParticleKind, Particles}, GRAVITY, MAX_FALL_SPEED}}, level_pack_data::{level_pos_to_pos, LevelPosition}, resources::Resources};

use super::{frog::Frog, key::Key, Entity};

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
    pos: Vec2,
    vel: Vec2,
    kind: CrateKind,
    spawn_pos: Option<LevelPosition>,
    hit: bool,
}

impl Crate {
    pub fn new(pos: Vec2, spawn_pos: Option<LevelPosition>, kind: CrateKind) -> Self {
        Self {
            pos,
            vel: Vec2::ZERO,
            kind,
            spawn_pos,
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
    fn spawn_pos(&self) -> Option<LevelPosition> {
        self.spawn_pos
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

    fn physics_update(&mut self, new_entities: &mut Vec<Box<dyn Entity>>, particles: &mut Particles, level: &mut Level, resources: &Resources) {
        self.vel.y = (self.vel.y + GRAVITY).min(MAX_FALL_SPEED);
        self.pos += self.vel;

        let prev_pos = self.pos;
        let prev_vel = self.vel;

        let moving_up = self.vel.y < 0.0;
        // Top/bottom
        let (mut t, mut b) = (false, false);
        if moving_up {
            // Top
            if collision_top(&mut self.pos, TOP, level, resources) {
                self.vel.y = 0.0;
                t = true;
            }
        } else {
            // Bottom
            if collision_bottom(&mut self.pos, BOT_L, level, resources)
            || collision_bottom(&mut self.pos, BOT_R, level, resources) {
                self.vel.y = 0.0;
                self.vel.x = 0.0;
                b = true;
            }
        }
        // Sides
        let sl = collision_left(&mut self.pos, SIDE_LT, true, level, resources)
        ||       collision_left(&mut self.pos, SIDE_LB, true, level, resources);
        let sr = collision_right(&mut self.pos, SIDE_RT, true, level, resources)
        ||       collision_right(&mut self.pos, SIDE_RB, true, level, resources);

        let s = sl && self.vel.x < 0.0
        ||      sr && self.vel.x > 0.0;
        if s {
            self.vel.x = 0.0;
        }

        // If we collided and have been thrown... we need to be destroyed and release the entities inside! also hit switches and stuff...
        let should_break = (prev_vel.x.abs() > 1.0 && s)
        || ((prev_vel.y.abs() > 1.5 || (prev_vel.y.abs() > 0.5 && prev_vel.x.abs() > 0.7)) && (t || b));

        if should_break {
            self.hit = true;
            
            match self.kind {
                CrateKind::Key(color)  => new_entities.push(Box::new(Key::new(self.pos, None, color))),
                CrateKind::Frog(false) => new_entities.push(Box::new(Frog::new(self.pos, None))),
                CrateKind::Frog(true) => { 
                    for _ in 0..gen_range(2, 3) {
                        new_entities.push(Box::new(Frog::new(self.pos, None)));
                        new_entities.push(Box::new(Frog::new(self.pos, None)));
                    }
                }
                _ => new_entities.push(Box::new(Key::new(self.pos, None, LockColor::Rainbow)))
            }

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
            for e in new_entities {
                e.set_vel(vec2(gen_range(x_min, x_max), gen_range(y_min, y_max)) * 0.7);
            }

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

            let can_hit_l = prev_vel.x <= -0.5;
            let can_hit_r = prev_vel.x >=  0.5;
            if let Some((_, p)) = [
                (t && self.vel.y >= 0.0, TOP),
                (sl && can_hit_l, SIDE_LT),
                (sl && can_hit_l, SIDE_LB),
                (sr && can_hit_r, SIDE_RT),
                (sr && can_hit_r, SIDE_RB)
            ].iter().find(|(s, _)| *s) {
                level.hit_tile_at_pos(prev_pos + *p, TileHitKind::Soft, resources);
            }
        }
    }

    fn draw(&self, camera_pos: Vec2, resources: &Resources) {
        Self::draw(self.pos, camera_pos, WHITE, resources);
    }
}