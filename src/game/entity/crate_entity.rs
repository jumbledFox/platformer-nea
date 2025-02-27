// I had to name the file 'crate_entity' because crate is a reserved keyword in rust lololol :3

use macroquad::{color::Color, math::{Rect, Vec2}};

use crate::{game::level::tile::LockColor, resources::Resources};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CrateKind {
    Frog(bool), // false for few, true for many
    Chip(bool), // ditto
    Life,
    Key(LockColor),
}

pub struct Crate {
    kind: CrateKind,
}

impl Crate {
    pub fn draw(pos: Vec2, camera_pos: Vec2, color: Color, resources: &Resources) {
        resources.draw_rect(pos - camera_pos, Rect::new(160.0, 0.0, 16.0, 16.0), false, color, resources.entity_atlas());
    }

    pub fn hitbox() -> Rect {
        Rect::new(0.0, 0.0, 16.0, 16.0)
    }
}