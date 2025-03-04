// I had to name the file 'crate_entity' because crate is a reserved keyword in rust lololol :3

use macroquad::{color::{Color, WHITE}, input::is_key_pressed, math::{vec2, Rect, Vec2}};

use crate::{game::{collision::{collision_bottom, collision_left, collision_right, collision_top}, level::{tile::LockColor, Level}, scene::{GRAVITY, MAX_FALL_SPEED}}, resources::Resources};

use super::Entity;

const TOP:     Vec2 = vec2( 8.0,  0.0);
const SIDE_LT: Vec2 = vec2( 0.0,  2.0);
const SIDE_LB: Vec2 = vec2( 0.0, 14.0);
const SIDE_RT: Vec2 = vec2(16.0,  2.0);
const SIDE_RB: Vec2 = vec2(16.0, 14.0);
const BOT_L:   Vec2 = vec2( 2.0, 16.0);
const BOT_R:   Vec2 = vec2(14.0, 16.0);

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
    index: usize,
}

impl Crate {
    pub fn new(pos: Vec2, kind: CrateKind, index: usize) -> Self {
        Self { kind, pos, vel: Vec2::ZERO, index }
    }

    pub fn draw(pos: Vec2, camera_pos: Vec2, color: Color, resources: &Resources) {
        resources.draw_rect(pos - camera_pos, Rect::new(160.0, 0.0, 16.0, 16.0), false, color, resources.entity_atlas());
    }

    pub fn hitbox() -> Rect {
        Rect::new(0.0, 0.0, 16.0, 16.0)
    }
}

impl Entity for Crate {
    fn index(&self) -> usize { self.index }
    fn update(&mut self, resources: &Resources) {
        if is_key_pressed(macroquad::input::KeyCode::G) {
            self.vel = vec2(1.0, -2.0);
        }
    }

    fn physics_update(&mut self, level: &mut Level, resources: &Resources) {
        self.vel.y = (self.vel.y + GRAVITY).min(MAX_FALL_SPEED);
        self.pos += self.vel;

        let moving_up = self.vel.y < 0.0;
        // Sides
        let s = collision_left(&mut self.pos, SIDE_LT, true, level, resources)
        ||      collision_left(&mut self.pos, SIDE_LB, true, level, resources)
        ||      collision_right(&mut self.pos, SIDE_RT, true, level, resources)
        ||      collision_right(&mut self.pos, SIDE_RB, true, level, resources);
        if s { self.vel.x = 0.0 }
        // Top/bottom
        if moving_up {
            // Top
            if collision_top(&mut self.pos, TOP, level, resources) {
                self.vel.y = 0.0;
            }
        } else {
            // Bottom
            if collision_bottom(&mut self.pos, BOT_L, level, resources)
            || collision_bottom(&mut self.pos, BOT_R, level, resources) {
                self.vel.y = 0.0;
                self.vel.x = 0.0;
            }
        }
    }

    fn draw(&self, camera_pos: Vec2, resources: &Resources) {
        Self::draw(self.pos, camera_pos, WHITE, resources);
    }
}